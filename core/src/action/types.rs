use crate::json_rpc::JsonRpcParams;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NoParams;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NoOutput;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalMethodParam {
    pub name: String,
    pub required: bool,
    pub description: Option<String>,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalMethodInfo {
    pub name: String,
    pub description: Option<String>,
    pub params: Vec<ExternalMethodParam>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExternalMethodParams {
    pub method: String,
    pub params: JsonRpcParams,
}
