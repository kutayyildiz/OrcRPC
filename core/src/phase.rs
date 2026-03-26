use crate::json_rpc::{JsonRpcMessage, JsonRpcSingleMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Outbound,
    Inbound,
}

impl Phase {
    pub fn is_outbound(self) -> bool {
        matches!(self, Self::Outbound)
    }
    pub fn is_inbound(self) -> bool {
        matches!(self, Self::Inbound)
    }
}

impl JsonRpcMessage {
    pub fn phase(&self) -> Result<Phase, &'static str> {
        match self {
            JsonRpcMessage::Single(JsonRpcSingleMessage::Response(_)) => Ok(Phase::Inbound),

            JsonRpcMessage::Single(
                JsonRpcSingleMessage::Request(_) | JsonRpcSingleMessage::Notification(_),
            ) => Ok(Phase::Outbound),

            JsonRpcMessage::Batch(batch) => {
                let all_inbound = batch
                    .0
                    .iter()
                    .all(|msg| matches!(msg, JsonRpcSingleMessage::Response(_)));

                let all_outbound = batch.0.iter().all(|msg| {
                    matches!(
                        msg,
                        JsonRpcSingleMessage::Request(_) | JsonRpcSingleMessage::Notification(_)
                    )
                });

                match (all_inbound, all_outbound) {
                    (true, false) => Ok(Phase::Inbound),
                    (false, true) => Ok(Phase::Outbound),
                    (false, false) => Err("mixed JSON-RPC batch is invalid"),
                    (true, true) => Err("empty JSON-RPC batch is invalid"),
                }
            }
        }
    }
}
