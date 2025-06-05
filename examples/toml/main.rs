use backoff_config::BackoffConfig;
use backon::Retryable;
use figment::Figment;
use figment::providers::{Data, Toml};
use log::{info, warn};
use rand::random_ratio;
use serde::Deserialize;
use std::time::Duration;

/// Example output:
/// 2025-06-04T11:54:27.724Z INFO  [toml] Config: Config { backoff: Exponential { initial_delay: 100ms, factor: 2.0, max_delay: 30s, max_retries: 8, max_total_delay: 60s, jitter_enabled: true, jitter_seed: None } }
/// 2025-06-04T11:54:27.725Z INFO  [toml] Doing very important work
/// 2025-06-04T11:54:28.232Z WARN  [toml] Failure: 'Very important error', sleeping 178ms
/// 2025-06-04T11:54:28.417Z INFO  [toml] Doing very important work
/// 2025-06-04T11:54:28.919Z INFO  [toml] Success!
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init()?;

    // Load config from Toml
    let config: Config = Figment::new()
        .merge(Data::<Toml>::file("examples/toml/config.toml"))
        .extract()?;

    info!("Config: {config:?}");

    // Use it in retries.
    may_fail
        .retry(config.backoff)
        .notify(|e, d| warn!("Failure: '{e}', sleeping {}ms", d.as_millis()))
        .await?;

    info!("Success!");

    Ok(())
}

#[derive(Debug, Deserialize)]
/// Example configuration.
pub struct Config {
    /// [BackoffConfig] to use in retries.
    pub backoff: BackoffConfig,
}

/// Emulates work that can fail or succeed.
async fn may_fail() -> anyhow::Result<()> {
    info!("Doing very important work");

    tokio::time::sleep(Duration::from_millis(500)).await;

    if random_ratio(1, 4) {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Very important error"))?
    }
}
