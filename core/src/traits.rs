use serde::{Serialize, de::DeserializeOwned};

use crate::{
    action_kind::ActionKind,
    error::ActionError,
    interception::{InterceptionRequest, InterceptionResponse},
    json_rpc::JsonRpcMessage,
    resolved_action_record::ResolvedActionRecord,
};

pub trait ActionSpec {
    type Params: Serialize + DeserializeOwned;
    type Result: Serialize + DeserializeOwned;

    const KIND: ActionKind;
}

pub trait ActionExecutor<A: ActionSpec> {
    fn execute(&self, params: A::Params) -> Result<A::Result, ActionError>;
}

pub trait InterceptionHandler {
    fn handle(&self, req: &InterceptionRequest) -> Result<InterceptionResponse, ActionError> {
        if let Some(resolved_actions) = &req.resolved_actions {
            self.handle_with_resolved_actions(&req.message, resolved_actions)
        } else {
            self.handle_without_resolved_actions(&req.message)
        }
    }

    fn handle_action_error(&self, _error: ActionError) {}

    fn handle_with_resolved_actions(
        &self,
        msg: &JsonRpcMessage,
        resolved_actions: &[ResolvedActionRecord],
    ) -> Result<InterceptionResponse, ActionError>;

    fn handle_without_resolved_actions(
        &self,
        msg: &JsonRpcMessage,
    ) -> Result<InterceptionResponse, ActionError>;
}
