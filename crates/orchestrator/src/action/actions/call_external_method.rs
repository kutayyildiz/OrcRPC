use crate::{
    action::TypedActionHandler, error::ActionExecutionError,
    runtime::external_methods::TransportExternalMethodCaller,
};
use actrpc_core::{
    DescribeParams,
    action::{ActionSpec, RequestedAction, ResolvedAction},
    interception::InterceptionRequest,
    json_rpc::{JsonRpcParams, JsonRpcResponse},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, DescribeParams)]
pub struct CallExternalMethodParams {
    pub method: String,
    pub params: Option<JsonRpcParams>,
}

pub struct CallExternalMethod;

impl ActionSpec for CallExternalMethod {
    type Params = CallExternalMethodParams;
    type Result = JsonRpcResponse;

    const KIND: &'static str = "call_external_method";
}

pub struct CallExternalMethodHandler<P>
where
    P: actrpc_transport::JsonRpcClientProvider<Error = actrpc_transport::TransportError>,
{
    caller: Arc<TransportExternalMethodCaller<P>>,
}

impl<P> CallExternalMethodHandler<P>
where
    P: actrpc_transport::JsonRpcClientProvider<Error = actrpc_transport::TransportError>,
{
    pub fn new(caller: Arc<TransportExternalMethodCaller<P>>) -> Self {
        Self { caller }
    }
}

impl<P> TypedActionHandler<CallExternalMethod> for CallExternalMethodHandler<P>
where
    P: actrpc_transport::JsonRpcClientProvider<Error = actrpc_transport::TransportError>
        + Send
        + Sync
        + 'static,
{
    fn handle_typed(
        &self,
        request: &InterceptionRequest,
        action: RequestedAction<CallExternalMethod>,
    ) -> Result<ResolvedAction<CallExternalMethod>, ActionExecutionError> {
        if action.params.method.trim().is_empty() {
            return Err(ActionExecutionError::InvalidParams {
                action: CallExternalMethod::action_kind(),
            });
        }

        let response =
            self.caller
                .call(request, &action.params.method, action.params.params.clone())?;

        Ok(ResolvedAction {
            params: action.params,
            result: Ok(response),
        })
    }
}
