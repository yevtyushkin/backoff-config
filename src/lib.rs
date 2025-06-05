//! # 🔄 `backoff-config` ⚙️
//!
//! [![codecov](https://codecov.io/github/yevtyushkin/backoff-config/graph/badge.svg?token=TKF2NSE4IT)](https://codecov.io/github/yevtyushkin/backoff-config)
//!
//! This crate enables declarative configuration of [backon](https://crates.io/crates/backon) retry crate strategies via
//! environment variables or configuration files by:
//!
//! - Unifying the backoff strategies provided by the [backon](https://crates.io/crates/backon) retry crate into a single
//!   enum (see [BackoffConfig]).
//!
//! - Implementing [serde::Deserialize] to support loading strategies from various
//!   configuration sources. [std::time::Duration] values are deserialized using human-readable formats (e.g. `5s`, `150 ms`).
//!
//! See [examples](https://github.com/yevtyushkin/backoff-config/tree/main/examples) and [tests](https://github.com/yevtyushkin/backoff-config/tree/main/tests) for example configuration formats.
mod backoff;
mod backoff_config;

pub use crate::backoff::*;
pub use crate::backoff_config::*;
