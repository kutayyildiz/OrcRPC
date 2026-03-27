use crate::{action::ActionSpec, error::ActionError};

pub trait ActionExecutor<A: ActionSpec> {
    fn execute(&self, params: A::Params) -> Result<A::Result, ActionError>;
}
