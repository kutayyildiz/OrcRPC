use crate::{
    phase::Phase,
    requested_action_record::RequestedActionRecord, resolved_action_record::ResolvedActionRecord,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterceptionRequest {
    pub origin: Participant,
    pub message: JsonRpcMessage, // ← see below
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_actions: Option<Vec<ResolvedActionRecord>>,
}
impl InterceptionRequest {
    pub fn has_previous_actions(&self) -> bool {
        self.resolved_actions
            .as_ref()
            .is_some_and(|actions| !actions.is_empty())
    }

    pub fn phase(&self) -> Phase {
        if matches!(self.message, JsonRpcMessage::Response(_)) {
            Phase::Inbound
        } else {
            Phase::Outbound
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterceptionDecision {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<RequestedActionRecord>>,
    pub is_final: bool,
}
