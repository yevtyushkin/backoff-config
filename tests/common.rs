use backoff_config::BackoffConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
/// Test config for tests.
pub struct Config {
    /// [BackoffConfig] that is being deserialized from various sources.
    pub backoff: BackoffConfig,
}
