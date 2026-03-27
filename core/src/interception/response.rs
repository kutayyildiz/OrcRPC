use crate::action::RequestedActionRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterceptionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<RequestedActionRecord>>,
    pub is_final: bool,
}

impl InterceptionResponse {
    pub fn has_actions(&self) -> bool {
        self.actions
            .as_ref()
            .is_some_and(|actions| !actions.is_empty())
    }
    pub fn is_final(&self) -> bool {
        self.is_final || !self.has_actions()
    }
}
