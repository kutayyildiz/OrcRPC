use crate::{
    error::OrchestratorError,
    external_method::MethodName,
    orchestrator::{Orchestrator, OrchestratorFuture},
    runtime::CallExecutionFactory,
};
use actrpc_core::json_rpc::{JsonRpcMessage, JsonRpcParams};
use std::sync::Arc;

pub struct DefaultOrchestrator {
    factory: Arc<CallExecutionFactory>,
}

impl DefaultOrchestrator {
    pub fn new(factory: Arc<CallExecutionFactory>) -> Self {
        Self { factory }
    }

    pub async fn call(
        &self,
        method: MethodName,
        params: Option<JsonRpcParams>,
    ) -> Result<JsonRpcMessage, OrchestratorError> {
        self.factory.run_root(method, params).await
    }
}

impl Orchestrator for DefaultOrchestrator {
    type Error = OrchestratorError;

    fn call<'a>(
        &'a self,
        method: MethodName,
        params: Option<JsonRpcParams>,
    ) -> OrchestratorFuture<'a, Result<JsonRpcMessage, Self::Error>>
    where
        Self: 'a,
    {
        Box::pin(async move { self.call(method, params).await })
    }
}
