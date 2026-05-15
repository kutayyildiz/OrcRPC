use crate::{
    action::{ActionHandlerFuture, TypedActionHandler},
    error::{ActionExecutionError, OrchestratorError},
    external_method::MethodName,
    runtime::{CallExecutionFactory, CallRuntime},
};
use actrpc_core::{
    DescribeParams,
    action::{ActionSpec, RequestedAction, ResolvedAction},
    interception::InterceptionRequest,
    json_rpc::{JsonRpcMessage, JsonRpcParams, JsonRpcResponse, JsonRpcSingleMessage},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, DescribeParams)]
#[serde(deny_unknown_fields)]
pub struct CallExternalMethodParams {
    pub method: MethodName,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<JsonRpcParams>,
}

pub struct CallExternalMethod;

impl ActionSpec for CallExternalMethod {
    type Params = CallExternalMethodParams;
    type Result = serde_json::Value;

    const KIND: &'static str = "call_external_method";
}

pub struct CallExternalMethodHandler {
    factory: Arc<CallExecutionFactory>,
    parent_call: Arc<CallRuntime>,
}

impl CallExternalMethodHandler {
    pub fn new(factory: Arc<CallExecutionFactory>, parent_call: Arc<CallRuntime>) -> Self {
        Self {
            factory,
            parent_call,
        }
    }
}

impl TypedActionHandler<CallExternalMethod> for CallExternalMethodHandler {
    fn handle_typed<'a>(
        &'a self,
        _request: &'a InterceptionRequest,
        action: RequestedAction<CallExternalMethod>,
    ) -> ActionHandlerFuture<'a, Result<ResolvedAction<CallExternalMethod>, ActionExecutionError>>
    where
        Self: 'a,
    {
        Box::pin(async move {
            let response = self
                .factory
                .run_piped(
                    action.params.method.clone(),
                    action.params.params.clone(),
                    self.parent_call.as_ref(),
                )
                .await
                .map_err(map_orchestrator_error)?;

            let result = decode_response_value(response)?;

            Ok(ResolvedAction {
                params: action.params,
                result: Ok(result),
            })
        })
    }
}

fn decode_response_value(
    message: JsonRpcMessage,
) -> Result<serde_json::Value, ActionExecutionError> {
    match message {
        JsonRpcMessage::Single(JsonRpcSingleMessage::Response(JsonRpcResponse::Success(
            success,
        ))) => Ok(success.result),

        JsonRpcMessage::Single(JsonRpcSingleMessage::Response(JsonRpcResponse::Error(error))) => {
            Err(ActionExecutionError::DependencyFailed {
                dependency: "call_external_method".to_owned(),
                message: format!(
                    "external method returned JSON-RPC error {}: {}",
                    error.error.code, error.error.message
                ),
            })
        }

        _ => Err(ActionExecutionError::DependencyFailed {
            dependency: "call_external_method".to_owned(),
            message: "external method did not return a single JSON-RPC response".to_owned(),
        }),
    }
}

fn map_orchestrator_error(error: OrchestratorError) -> ActionExecutionError {
    ActionExecutionError::DependencyFailed {
        dependency: "call_execution".to_owned(),
        message: error.to_string(),
    }
}
