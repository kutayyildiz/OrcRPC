#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum InterceptorError {
    #[error("invalid interceptor configuration: {message}")]
    Config { message: String },

    #[error(transparent)]
    Interception(#[from] actrpc_core::error::InterceptionError),

    #[error(transparent)]
    Protocol(#[from] actrpc_core::error::ProtocolError),

    #[error(transparent)]
    Codec(#[from] actrpc_core::error::CodecError),

    #[error("transport error: {message}")]
    Transport { message: String },

    #[error("internal interceptor error: {message}")]
    Internal { message: String },
}
