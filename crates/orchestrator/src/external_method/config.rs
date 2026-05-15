use crate::external_method::MethodDescriptor;
use actrpc_transport::TransportTarget;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalMethodConfig {
    pub descriptor: MethodDescriptor,
    pub target: TransportTarget,
    pub remote_method: String,
}
