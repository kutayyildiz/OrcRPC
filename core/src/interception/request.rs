use crate::{
    action::ResolvedActionRecord, json_rpc::JsonRpcMessage, participant::Participant, phase::Phase,
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

    pub fn phase(&self) -> Result<Phase, &'static str> {
        self.message.phase()
    }
}
