use actrpc_core::{
    error::InterceptionError,
    interception::{InterceptionRequest, InterceptionResponse},
};

pub trait Interceptor: Send + Sync {
    fn intercept(
        &self,
        request: InterceptionRequest,
    ) -> Result<InterceptionResponse, InterceptionError>;
}
