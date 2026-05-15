use actrpc_core::json_rpc::JsonRpcError;
use actrpc_transport::TransportError;

use crate::external_method::MethodName;

#[derive(Debug, thiserror::Error)]
pub enum ExternalMethodError {
    #[error("failed to create client for external method {name}: {source}")]
    ClientCreate {
        name: MethodName,
        #[source]
        source: TransportError,
    },

    #[error("external method {name} call failed: {source}")]
    CallFailed {
        name: MethodName,
        #[source]
        source: TransportError,
    },

    #[error("external method {name} returned remote JSON-RPC error: {error:?}")]
    RemoteError {
        name: MethodName,
        error: JsonRpcError,
    },

    #[error("failed to read external method config {path}: {source}")]
    ConfigRead {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to deserialize YAML external method config {path}: {source}")]
    ConfigYaml {
        path: String,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("failed to deserialize TOML external method config {path}: {source}")]
    ConfigToml {
        path: String,
        #[source]
        source: toml::de::Error,
    },

    #[error("unsupported external method config format for {path}: {extension}")]
    UnsupportedConfigFormat { path: String, extension: String },

    #[error("duplicate external method: {name}")]
    DuplicateMethod { name: MethodName },

    #[error("external method not found: {name}")]
    MethodNotFound { name: MethodName },

    #[error("external method {name} returned invalid JSON-RPC response: {message}")]
    InvalidResponse { name: MethodName, message: String },
}
