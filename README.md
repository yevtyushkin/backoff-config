# 🔄 `backoff-config` ⚙️

[![codecov](https://codecov.io/github/yevtyushkin/backoff-config/graph/badge.svg?token=TKF2NSE4IT)](https://codecov.io/github/yevtyushkin/backoff-config)

This crate enables declarative configuration of [backon](https://crates.io/crates/backon) retry crate strategies via
environment variables or configuration files by:

- Unifying the backoff strategies provided by the [backon](https://crates.io/crates/backon) retry crate into a single
  enum (see [BackoffConfig](src/backoff_config.rs)).

- Implementing [serde](https://docs.rs/serde/latest/serde/)’s [Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html) to support loading strategies from various
  configuration sources. `Duration` values are deserialized using human-readable formats (e.g. `5s`, `150 ms`).

## Examples

- Loading from TOML and `figment` crate:

```bash
cargo run --example toml
```

- Loading from env and `figment` crate:

```bash
cargo run --example env
```

Consider the following unit test specs for the detailed configuration formats:

- [Env](tests/env.rs)
- [TOML](tests/toml.rs)
