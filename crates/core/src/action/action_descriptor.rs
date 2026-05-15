use crate::{
    action::ActionKind,
    descriptor::types::{OkDescriptor, ParamsDescriptor},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ActionDescriptor {
    pub kind: ActionKind,
    pub params: Option<ParamsDescriptor>,
    pub ok: Option<OkDescriptor>,
}
