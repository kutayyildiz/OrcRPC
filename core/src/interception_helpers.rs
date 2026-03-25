use crate::{
    interception::{InterceptionDecision, InterceptionRequest},
    json_rpc::{JsonRpcRequest, JsonRpcResponse},
};

}

/// Orchestrator → Interceptor  
pub fn create_interception_request(
    interception: InterceptionRequest,
    id: Option<Value>,
) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id,
        method: "intercept".to_string(),
        params: serde_json::to_value(interception)
            .expect("Failed to serialize InterceptionRequest (bug in core)"),
    }
}

/// Interceptor → Orchestrator  
pub fn create_interception_response(
    decision: InterceptionDecision,
    request_id: Option<Value>,
) -> JsonRpcResponse {
    JsonRpcResponse::Success {
        jsonrpc: "2.0".to_string(),
        id: request_id,
        result: serde_json::to_value(decision)
            .expect("Failed to serialize InterceptionDecision (bug in core)"),
    }
}

impl InterceptionRequest {
    pub fn into_rpc_request(self, id: Option<Value>) -> JsonRpcRequest {
        create_interception_request(self, id)
    }
}

impl InterceptionDecision {
    /// Same as `create_inspect_response` but on the struct itself.
    pub fn into_rpc_response(self, request_id: Option<Value>) -> JsonRpcResponse {
        create_interception_response(self, request_id)
    }
}
