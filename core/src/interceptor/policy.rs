use crate::action::{ActionKind, RequestedActionRecord};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterceptorPolicy {
    pub interceptor_name: String,
    pub allowed_actions: HashSet<ActionKind>,
}

impl InterceptorPolicy {
    /// Returns true if every requested action is allowed by this policy.
    pub fn allows_all(&self, actions: &[RequestedActionRecord]) -> bool {
        actions
            .iter()
            .all(|action| self.allowed_actions.contains(&action.kind))
    }

    /// Returns the requested actions that conflict with this policy.
    pub fn conflicting_actions<'a>(
        &self,
        actions: &'a [RequestedActionRecord],
    ) -> Vec<&'a RequestedActionRecord> {
        actions
            .iter()
            .filter(|action| !self.allowed_actions.contains(&action.kind))
            .collect()
    }
}
