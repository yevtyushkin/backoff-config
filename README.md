# 🔄 `backoff-config` ⚙️

`backoff-config` makes configuring `backoff` more flexible by providing a unified `Backoff` strategy enum and a
`BackoffConfig` that supports deserialization.

The actual backoff logic is powered by the awesome [backon](https://crates.io/crates/backon) crate. Make sure to check
out [backon](https://crates.io/crates/backon) to explore its amazing features and ergonomics!   

`backoff-config` integrates with [backon](https://crates.io/crates/backon) by implementing:

- [`backon::BackoffBuilder`](https://docs.rs/backon/1.5.0/backon/trait.BackoffBuilder.html) for [
  `BackoffConfig`](src/backoff_config.rs).
- [`backon::Backoff`](https://docs.rs/backon/1.5.0/backon/trait.Backoff.html) for [`Backoff`](src/backoff.rs).

## Usage

1. Add to your `Cargo.toml`:

    ```toml
    backoff-config = "0.1"
    ```

2. Load [`BackoffConfig`](src/backoff_config.rs) from your data source. Here’s an example using environment variables
   and the [figment](https://crates.io/crates/figment) configuration library:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub backoff: BackoffConfig,
}

// Example environment variables:
// CONFIG__BACKOFF__STRATEGY=Constant
// CONFIG__BACKOFF__DELAY=1s
// CONFIG__BACKOFF__MAX_RETRIES=123

let config = figment::Figment::new()
.merge(Env::prefixed("CONFIG__").split("__"))
.extract::<Config>() ?;
```

3. Use `config.backoff` to build and apply your backoff strategy when calling retryable operations.

## Examples

For an end-to-end usage example, see the [TOML example](examples/toml.rs) and run it with:

```bash
cargo run --example toml
```

For more examples on the data formats, see:

- [Env examples](tests/env.rs) – loading config from environment variables.
- [TOML examples](tests/toml.rs) – loading config from a TOML file.
