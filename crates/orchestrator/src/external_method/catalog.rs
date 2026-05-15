use crate::{
    error::ExternalMethodError,
    external_method::{ExternalMethodConfig, MethodDescriptor, MethodName},
};
use actrpc_core::json_rpc::{
    JsonRpcId, JsonRpcMessage, JsonRpcParams, JsonRpcRequest, JsonRpcResponse,
    JsonRpcSingleMessage, JsonRpcVersion,
};
use actrpc_transport::{JsonRpcClient, JsonRpcClientProvider, TransportError};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

pub struct ExternalMethod {
    remote_method: String,
    client: Arc<dyn JsonRpcClient<Error = TransportError>>,
}

impl ExternalMethod {
    pub fn new(
        remote_method: impl Into<String>,
        client: Arc<dyn JsonRpcClient<Error = TransportError>>,
    ) -> Self {
        Self {
            remote_method: remote_method.into(),
            client,
        }
    }
}

pub struct ExternalMethodCatalog {
    methods: HashMap<MethodName, ExternalMethod>,
    descriptors: HashMap<MethodName, MethodDescriptor>,
    next_id: AtomicU64,
}

impl ExternalMethodCatalog {
    pub async fn from_configs<P>(
        configs: Vec<ExternalMethodConfig>,
        provider: &P,
    ) -> Result<Self, ExternalMethodError>
    where
        P: JsonRpcClientProvider<
                Client = Arc<dyn JsonRpcClient<Error = TransportError>>,
                Error = TransportError,
            >,
    {
        let mut methods = HashMap::new();
        let mut descriptors = HashMap::new();

        for config in configs {
            let name = config.descriptor.name.clone();

            if methods.contains_key(&name) {
                return Err(ExternalMethodError::DuplicateMethod { name });
            }

            let client = provider
                .get_client(&config.target)
                .await
                .map_err(|source| ExternalMethodError::ClientCreate {
                    name: name.clone(),
                    source,
                })?;

            methods.insert(
                name.clone(),
                ExternalMethod::new(config.remote_method, client),
            );

            descriptors.insert(name, config.descriptor);
        }

        Ok(Self {
            methods,
            descriptors,
            next_id: AtomicU64::new(1),
        })
    }

    pub fn descriptors(&self) -> impl Iterator<Item = &MethodDescriptor> {
        self.descriptors.values()
    }

    pub fn descriptor(&self, name: &MethodName) -> Option<&MethodDescriptor> {
        self.descriptors.get(name)
    }

    pub fn contains(&self, name: &MethodName) -> bool {
        self.methods.contains_key(name)
    }

    pub fn request_message(
        &self,
        name: &MethodName,
        params: Option<JsonRpcParams>,
    ) -> Result<JsonRpcMessage, ExternalMethodError> {
        let method = self
            .methods
            .get(name)
            .ok_or_else(|| ExternalMethodError::MethodNotFound { name: name.clone() })?;

        let id = JsonRpcId::Number(self.next_id.fetch_add(1, Ordering::Relaxed).into());

        Ok(JsonRpcMessage::Single(JsonRpcSingleMessage::Request(
            JsonRpcRequest {
                jsonrpc: JsonRpcVersion::V2_0,
                id,
                method: method.remote_method.clone(),
                params,
            },
        )))
    }

    pub async fn send_message(
        &self,
        name: &MethodName,
        message: JsonRpcMessage,
    ) -> Result<JsonRpcMessage, ExternalMethodError> {
        let method = self
            .methods
            .get(name)
            .ok_or_else(|| ExternalMethodError::MethodNotFound { name: name.clone() })?;

        method
            .client
            .send(message)
            .await
            .map_err(|source| ExternalMethodError::CallFailed {
                name: name.clone(),
                source,
            })
    }

    pub async fn call(
        &self,
        name: &MethodName,
        params: Option<JsonRpcParams>,
    ) -> Result<serde_json::Value, ExternalMethodError> {
        let request = self.request_message(name, params)?;
        let expected_id = request_id(name, &request)?;

        let response = self.send_message(name, request).await?;

        decode_external_method_response(name, expected_id, response)
    }
}

fn request_id(
    name: &MethodName,
    message: &JsonRpcMessage,
) -> Result<JsonRpcId, ExternalMethodError> {
    let JsonRpcMessage::Single(JsonRpcSingleMessage::Request(request)) = message else {
        return Err(ExternalMethodError::InvalidResponse {
            name: name.clone(),
            message: "external method request builder produced a non-request message".to_owned(),
        });
    };

    Ok(request.id.clone())
}

fn decode_external_method_response(
    name: &MethodName,
    expected_id: JsonRpcId,
    message: JsonRpcMessage,
) -> Result<serde_json::Value, ExternalMethodError> {
    let JsonRpcMessage::Single(JsonRpcSingleMessage::Response(response)) = message else {
        return Err(ExternalMethodError::InvalidResponse {
            name: name.clone(),
            message: "external method returned a non-response JSON-RPC message".to_owned(),
        });
    };

    match response {
        JsonRpcResponse::Success(success) => {
            if success.id != expected_id {
                return Err(ExternalMethodError::InvalidResponse {
                    name: name.clone(),
                    message: "external method returned response with mismatched id".to_owned(),
                });
            }

            Ok(success.result)
        }

        JsonRpcResponse::Error(error) => {
            if error.id != expected_id {
                return Err(ExternalMethodError::InvalidResponse {
                    name: name.clone(),
                    message: "external method returned error response with mismatched id"
                        .to_owned(),
                });
            }

            Err(ExternalMethodError::RemoteError {
                name: name.clone(),
                error: error.error,
            })
        }
    }
}
