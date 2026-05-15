use crate::{
    error::OrchestratorError,
    external_method::MethodName,
    runtime::{CallExecution, CallRuntime, OrchestratorResources},
};
use actrpc_core::json_rpc::{JsonRpcMessage, JsonRpcParams};
use std::sync::Arc;

pub const DEFAULT_MAX_CALL_DEPTH: usize = 64;

pub struct CallExecutionFactory {
    resources: Arc<OrchestratorResources>,
    max_depth: usize,
}

impl CallExecutionFactory {
    pub fn new(resources: Arc<OrchestratorResources>) -> Self {
        Self {
            resources,
            max_depth: DEFAULT_MAX_CALL_DEPTH,
        }
    }

    pub fn with_max_depth(resources: Arc<OrchestratorResources>, max_depth: usize) -> Self {
        Self {
            resources,
            max_depth,
        }
    }

    pub fn resources(&self) -> &Arc<OrchestratorResources> {
        &self.resources
    }

    pub fn create_root(
        self: &Arc<Self>,
        method: MethodName,
        params: Option<JsonRpcParams>,
    ) -> Result<CallExecution, OrchestratorError> {
        let message = self
            .resources
            .external_method_catalog
            .request_message(&method, params)?;

        Ok(CallExecution::new(
            self.clone(),
            Arc::new(CallRuntime::root(message)),
            method,
        ))
    }

    pub fn create_piped(
        self: &Arc<Self>,
        method: MethodName,
        params: Option<JsonRpcParams>,
        parent: &CallRuntime,
    ) -> Result<CallExecution, OrchestratorError> {
        let depth = parent.depth() + 1;

        if depth > self.max_depth {
            return Err(OrchestratorError::Internal {
                message: format!(
                    "maximum piped call depth exceeded: depth {depth}, limit {}",
                    self.max_depth
                ),
            });
        }

        let message = self
            .resources
            .external_method_catalog
            .request_message(&method, params)?;

        Ok(CallExecution::new(
            self.clone(),
            Arc::new(CallRuntime::nested(
                message,
                parent.transcript.clone(),
                parent.depth(),
            )),
            method,
        ))
    }

    pub async fn run_root(
        self: &Arc<Self>,
        method: MethodName,
        params: Option<JsonRpcParams>,
    ) -> Result<JsonRpcMessage, OrchestratorError> {
        let execution = self.create_root(method, params)?;
        execution.run().await
    }

    pub async fn run_piped(
        self: &Arc<Self>,
        method: MethodName,
        params: Option<JsonRpcParams>,
        parent: &CallRuntime,
    ) -> Result<JsonRpcMessage, OrchestratorError> {
        let execution = self.create_piped(method, params, parent)?;
        execution.run().await
    }
}
