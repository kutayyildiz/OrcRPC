use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    ModifyParams,
    ModifyResult,
    ModifyError,
    CallExternalMethod,
    ListExternalMethods,
    GetInterceptorOrder,
    GetInterceptorState,
    GetInterceptorPolicy,
    EnableInterceptors,
    DisableInterceptors,
    GetTranscript,
}
