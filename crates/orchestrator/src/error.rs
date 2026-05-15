use actrpc_core::error::CodecError;
use actrpc_transport::TransportError;

mod action;
mod action_execution;
mod action_handler;
mod external_method_error;
mod interceptor;
mod interceptor_runtime;

pub use action::ActionError;
pub use action_execution::ActionExecutionError;
pub use action_handler::ActionHandlerError;
pub use external_method_error::ExternalMethodError;
pub use interceptor::InterceptorError;
pub use interceptor_runtime::InterceptorRuntimeError;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum OrchestratorError {
    #[error(transparent)]
    ExternalMethod(#[from] ExternalMethodError),

    #[error(transparent)]
    Action(#[from] ActionError),

    #[error(transparent)]
    Interceptor(#[from] InterceptorError),

    #[error(transparent)]
    Transport(#[from] TransportError),

    #[error(transparent)]
    Codec(#[from] CodecError),

    #[error("internal orchestrator error: {message}")]
    Internal { message: String },
}
