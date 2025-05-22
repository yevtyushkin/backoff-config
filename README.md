# 🔄 `backoff-config` ⚙️

[![codecov](https://codecov.io/github/yevtyushkin/backoff-config/graph/badge.svg?token=TKF2NSE4IT)](https://codecov.io/github/yevtyushkin/backoff-config)

`backoff-config` makes configuring `backoff` more flexible by providing a unified `Backoff` strategy enum and a
`BackoffConfig` that supports deserialization.

The actual backoff logic is powered by the awesome [backon](https://crates.io/crates/backon) crate. Make sure to check
out [backon](https://crates.io/crates/backon) to explore its amazing features and ergonomics!

`backoff-config` integrates with [backon](https://crates.io/crates/backon) by implementing:

- [`backon::BackoffBuilder`](https://docs.rs/backon/1.5.0/backon/trait.BackoffBuilder.html) for [
  `BackoffConfig`](src/backoff_config.rs).
- [`backon::Backoff`](https://docs.rs/backon/1.5.0/backon/trait.Backoff.html) for [`Backoff`](src/backoff.rs).

## Usage

1. Add `backoff-config` to your dependencies:

```bash
cargo add backoff-config
```

2. Load `BackoffConfig` and use it straight away in retries. Example with [figment](https://crates.io/crates/figment):

```rust
use serde::Deserialize;
use backon::Retryable;

#[derive(Deserialize)]
pub struct Config {
    pub backoff: BackoffConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Env variables:
    //
    // CONFIG__BACKOFF__STRATEGY=Constant
    // CONFIG__BACKOFF__DELAY=1s
    // CONFIG__BACKOFF__MAX_RETRIES=4
    let config = figment::Figment::new()
        .merge(Env::prefixed("CONFIG__").split("__"))
        .extract::<Config>()?;

    // Use in retries: 
    let response_body = fetch.retry(config.backoff).await?;
    println!("Response body: {response_body}");

    Ok(())
}

// Function that may fail
async fn fetch() -> anyhow::Result<String> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    Ok(body)
}
```

## Examples

- TOML + [figment](https://crates.io/crates/figment):

```bash
cargo run --example toml
```

And some examples on data formats:

- [Env](tests/env.rs) – loading config from environment variables.
- [TOML](tests/toml.rs) – loading config from a TOML file.
