#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config")]
    Read {
        #[source]
        source: std::io::Error,
    },

    #[error("failed to deserialize TOML config")]
    DeserializeToml {
        #[source]
        source: toml::de::Error,
    },

    #[error("failed to deserialize YAML config")]
    DeserializeYaml {
        #[source]
        source: serde_yaml::Error,
    },
}
