use crate::{
    action::{ActionKind, RequestedActionRecord},
    interception::InterceptionPhase,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterceptorPolicy {
    pub interceptor_name: String,
    pub allowed_actions_by_phase: HashMap<InterceptionPhase, HashSet<ActionKind>>,
}

impl InterceptorPolicy {
    pub fn allows_all(&self, phase: InterceptionPhase, actions: &[RequestedActionRecord]) -> bool {
        let Some(allowed) = self.allowed_actions_by_phase.get(&phase) else {
            return actions.is_empty();
        };

        actions.iter().all(|action| allowed.contains(&action.kind))
    }

    pub fn conflicting_actions<'a>(
        &self,
        phase: InterceptionPhase,
        actions: &'a [RequestedActionRecord],
    ) -> Vec<&'a RequestedActionRecord> {
        let Some(allowed) = self.allowed_actions_by_phase.get(&phase) else {
            return actions.iter().collect();
        };

        actions
            .iter()
            .filter(|action| !allowed.contains(&action.kind))
            .collect()
    }
}
