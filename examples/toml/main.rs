use backoff_config::BackoffConfig;
use backon::Retryable;
use figment::providers::{Data, Toml};
use figment::Figment;
use log::{info, warn};
use rand::random_ratio;
use serde::Deserialize;
use std::time::Duration;

// 2025-06-05T19:53:29.048Z INFO  [toml] Config: Config { backoff: Exponential(ExponentialBackoffConfig { initial_delay: 100ms, factor: 2.0, max_delay: 30s, max_retries: 8, max_total_delay: 60s, jitter_enabled: true, jitter_seed: None }) }
// 2025-06-05T19:53:29.048Z INFO  [toml] Doing very important work
// 2025-06-05T19:53:29.555Z WARN  [toml] Failure: 'Very important error', sleeping 123ms
// 2025-06-05T19:53:29.682Z INFO  [toml] Doing very important work
// 2025-06-05T19:53:30.186Z WARN  [toml] Failure: 'Very important error', sleeping 309ms
// 2025-06-05T19:53:30.499Z INFO  [toml] Doing very important work
// 2025-06-05T19:53:31.006Z WARN  [toml] Failure: 'Very important error', sleeping 533ms
// 2025-06-05T19:53:31.544Z INFO  [toml] Doing very important work
// 2025-06-05T19:53:32.050Z WARN  [toml] Failure: 'Very important error', sleeping 918ms
// 2025-06-05T19:53:32.972Z INFO  [toml] Doing very important work
// 2025-06-05T19:53:33.476Z INFO  [toml] Success!
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
