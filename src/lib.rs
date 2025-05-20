//! # backoff-config
//!
//! `backoff-config` makes configuring `backoff` more flexible by providing a unified `Backoff` strategy enum and a
//! `BackoffConfig` that supports deserialization.
//!
//! The actual backoff logic is powered by the awesome [backon](https://crates.io/crates/backon) crate. Make sure to check
//! out [backon](https://crates.io/crates/backon) to explore its amazing features and ergonomics!

mod backoff;
mod backoff_config;
mod nullable;

pub use crate::backoff::*;
pub use crate::backoff_config::*;
pub use crate::nullable::*;
