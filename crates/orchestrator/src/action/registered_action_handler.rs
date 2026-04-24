use crate::{
    action::{ActionHandler, TypedActionHandler},
    error::ActionHandlerError,
};
use actrpc_core::action::{ActionKind, ActionSpec};
use std::marker::PhantomData;

pub struct RegisteredActionHandler<A, H>
where
    A: ActionSpec,
    H: TypedActionHandler<A>,
{
    inner: H,
    _marker: PhantomData<A>,
}

impl<A, H> RegisteredActionHandler<A, H>
where
    A: ActionSpec,
    H: TypedActionHandler<A>,
{
    pub fn new(inner: H) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<A, H> ActionHandler for RegisteredActionHandler<A, H>
where
    A: ActionSpec + Send + Sync + 'static,
    H: TypedActionHandler<A> + Send + Sync + 'static,
{
    fn kind(&self) -> ActionKind {
        A::action_kind()
    }

    fn handle(
        &self,
        request: &actrpc_core::interception::InterceptionRequest,
        action: actrpc_core::action::RequestedActionRecord,
    ) -> Result<actrpc_core::action::ResolvedActionRecord, ActionHandlerError> {
        let typed_action = action.try_into()?;
        let resolved = self.inner.handle_typed(request, typed_action)?;
        let record: actrpc_core::action::ResolvedActionRecord =
            resolved.try_into().map_err(map_serde_serialize_err)?;

        Ok(record)
    }
}

fn map_serde_serialize_err(e: serde_json::Error) -> actrpc_core::error::CodecError {
    actrpc_core::error::CodecError::Serialize(e.to_string())
}
