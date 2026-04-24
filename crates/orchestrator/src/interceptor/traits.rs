use crate::error::InterceptorRuntimeError;
use actrpc_core::{
    InterceptorInitialization,
    interception::{InterceptionRequest, InterceptionResponse},
};

pub trait Interceptor: Send + Sync {
    fn initialize(&self) -> Result<InterceptorInitialization, InterceptorRuntimeError>;

    fn intercept(
        &self,
        request: &InterceptionRequest,
    ) -> Result<InterceptionResponse, InterceptorRuntimeError>;
}
