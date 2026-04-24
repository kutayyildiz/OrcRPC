use crate::error::ActionHandlerError;
use actrpc_core::{
    action::{ActionKind, RequestedActionRecord},
    interception::InterceptionRequest,
};

pub trait ActionHandler: Send + Sync {
    fn kind(&self) -> ActionKind;

    fn handle(
        &self,
        request: &InterceptionRequest,
        action: RequestedActionRecord,
    ) -> Result<actrpc_core::action::ResolvedActionRecord, ActionHandlerError>;
}
