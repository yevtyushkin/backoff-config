use backoff_config::BackoffConfig;
use backon::Retryable;
use figment::Figment;
use figment::providers::{Env, Toml};
use log::{info, warn};
use rand::random_ratio;
use serde::Deserialize;
use std::time::Duration;

/// Example output:
/// 2025-06-04T11:59:51.518Z INFO  [env] Config: Config { backoff: Exponential { initial_delay: 100ms, factor: 2.0, max_delay: 30s, max_retries: 8, max_total_delay: 60s, jitter_enabled: true, jitter_seed: None } }
/// 2025-06-04T11:59:51.518Z INFO  [env] Doing very important work
/// 2025-06-04T11:59:52.025Z WARN  [env] Failure: 'Very important error', sleeping 129ms
/// 2025-06-04T11:59:52.159Z INFO  [env] Doing very important work
/// 2025-06-04T11:59:52.662Z WARN  [env] Failure: 'Very important error', sleeping 307ms
/// 2025-06-04T11:59:52.972Z INFO  [env] Doing very important work
/// 2025-06-04T11:59:53.477Z INFO  [env] Success!/
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init()?;

    // Or you can set the vars yourself before running this example.
    unsafe {
        std::env::set_var("CONFIG__BACKOFF__STRATEGY", "Exponential");
        std::env::set_var("CONFIG__BACKOFF__INITIAL_DELAY", "100ms");
        std::env::set_var("CONFIG__BACKOFF__FACTOR", "2.0");
        std::env::set_var("CONFIG__BACKOFF__MAX_RETRIES", "8");
    }

    // Load config from env
    let config: Config = Figment::new()
        .merge(Env::prefixed("CONFIG__").split("__"))
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
