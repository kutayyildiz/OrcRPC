use crate::{
    action::TypedActionHandler, error::ActionExecutionError, runtime::CurrentCallRejection,
};
use actrpc_core::{
    DescribeParams,
    action::{ActionSpec, NoOk, RequestedAction, ResolvedAction},
    interception::InterceptionRequest,
    json_rpc::JsonRpcError,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, DescribeParams)]
pub struct RejectCallParams {
    pub error: JsonRpcError,
}

pub struct RejectCall;

impl ActionSpec for RejectCall {
    type Params = RejectCallParams;
    type Result = NoOk;

    const KIND: &'static str = "reject_call";
}

pub struct RejectCallHandler {
    rejection: Arc<CurrentCallRejection>,
}

impl RejectCallHandler {
    pub fn new(rejection: Arc<CurrentCallRejection>) -> Self {
        Self { rejection }
    }
}

impl TypedActionHandler<RejectCall> for RejectCallHandler {
    fn handle_typed(
        &self,
        _request: &InterceptionRequest,
        action: RequestedAction<RejectCall>,
    ) -> Result<ResolvedAction<RejectCall>, ActionExecutionError> {
        self.rejection.set(action.params.error.clone());

        Ok(ResolvedAction {
            params: action.params,
            result: Ok(NoOk),
        })
    }
}
