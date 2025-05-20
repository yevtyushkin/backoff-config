use backoff_config::BackoffConfig;
use backon::Retryable;
use figment::Figment;
use figment::providers::{Data, Toml};
use serde::Deserialize;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Debug, Deserialize, PartialEq)]
pub struct MyAwesomeConfig {
    pub backoff: BackoffConfig,
}

#[derive(Default)]
pub struct MyServiceThatCanFail {
    pub counter: Arc<AtomicU32>,
}

impl MyServiceThatCanFail {
    pub async fn call(&self) -> Result<&str, Box<dyn std::error::Error>> {
        let cnt = self.counter.fetch_add(1, Relaxed);

        if cnt <= 5 {
            return Err(format!("oops. failed. counter = {cnt}").into());
        }

        Ok("well done")
    }
}

#[tokio::main]
/// Example output:
/// Retrying my_service.call(), err: oops. failed. counter = 0, sleeping: 178ms
/// Retrying my_service.call(), err: oops. failed. counter = 1, sleeping: 332ms
/// Retrying my_service.call(), err: oops. failed. counter = 2, sleeping: 515ms
/// Retrying my_service.call(), err: oops. failed. counter = 3, sleeping: 1421ms
/// Retrying my_service.call(), err: oops. failed. counter = 4, sleeping: 2173ms
/// Retrying my_service.call(), err: oops. failed. counter = 5, sleeping: 5924ms
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: MyAwesomeConfig = Figment::new()
        .merge(Data::<Toml>::file("examples/example-config.toml"))
        .extract()?;

    println!("Config: {config:?}");

    let my_service = MyServiceThatCanFail::default();

    let result = (|| async { my_service.call().await })
        .retry(config.backoff)
        .notify(|e, d| {
            println!(
                "Retrying my_service.call(), err: {e}, sleeping: {}ms",
                d.as_millis()
            )
        })
        .await?;

    println!("Result: {result:?}");

    Ok(())
}
