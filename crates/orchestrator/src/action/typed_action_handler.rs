use crate::error::ActionExecutionError;
use actrpc_core::{
    action::{ActionSpec, RequestedAction},
    interception::InterceptionRequest,
};

pub trait TypedActionHandler<A>: Send + Sync
where
    A: ActionSpec,
{
    fn handle_typed(
        &self,
        request: &InterceptionRequest,
        action: RequestedAction<A>,
    ) -> Result<actrpc_core::action::ResolvedAction<A>, ActionExecutionError>;
}
