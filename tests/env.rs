mod common;

use crate::common::*;
use backoff_config::*;
use figment::providers::Env;
use std::time::Duration;

#[test]
fn constant_backoff_with_defaults() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("CONFIG__BACKOFF__STRATEGY", "Constant");

        let config = figment::Figment::new()
            .merge(Env::prefixed("CONFIG__").split("__"))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::Constant {
                    delay: defaults::delay(),
                    max_retries: defaults::max_retries(),
                    jitter_enabled: defaults::jitter_enabled(),
                    jitter_seed: defaults::jitter_seed(),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn constant_backoff_with_custom_values() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("CONFIG__BACKOFF__STRATEGY", "Constant");

        jail.set_env("CONFIG__BACKOFF__DELAY", "123s");
        jail.set_env("CONFIG__BACKOFF__MAX_RETRIES", "456");
        jail.set_env("CONFIG__BACKOFF__JITTER_ENABLED", "false");
        jail.set_env("CONFIG__BACKOFF__JITTER_SEED", "1337");

        let config = figment::Figment::new()
            .merge(Env::prefixed("CONFIG__").split("__"))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::Constant {
                    delay: Duration::from_secs(123).into(),
                    max_retries: Nullable::Some(456),
                    jitter_enabled: false,
                    jitter_seed: Some(1337),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn constant_backoff_with_null_values() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("CONFIG__BACKOFF__STRATEGY", "Constant");

        jail.set_env("CONFIG__BACKOFF__MAX_RETRIES", "null");

        let config = figment::Figment::new()
            .merge(Env::prefixed("CONFIG__").split("__"))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::Constant {
                    delay: defaults::delay(),
                    max_retries: Nullable::Null,
                    jitter_enabled: defaults::jitter_enabled(),
                    jitter_seed: defaults::jitter_seed(),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn exponential_backoff_with_defaults() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("CONFIG__BACKOFF__STRATEGY", "Exponential");

        let config = figment::Figment::new()
            .merge(Env::prefixed("CONFIG__").split("__"))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::Exponential {
                    initial_delay: defaults::delay(),
                    factor: defaults::factor(),
                    max_delay: defaults::max_delay(),
                    max_retries: defaults::max_retries(),
                    max_total_delay: defaults::max_total_delay(),
                    jitter_enabled: defaults::jitter_enabled(),
                    jitter_seed: defaults::jitter_seed(),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn exponential_backoff_with_custom_values() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("CONFIG__BACKOFF__STRATEGY", "Exponential");

        jail.set_env("CONFIG__BACKOFF__INITIAL_DELAY", "750ms");
        jail.set_env("CONFIG__BACKOFF__FACTOR", "3.5");
        jail.set_env("CONFIG__BACKOFF__MAX_DELAY", "20s");
        jail.set_env("CONFIG__BACKOFF__MAX_RETRIES", "10");
        jail.set_env("CONFIG__BACKOFF__MAX_TOTAL_DELAY", "90s");
        jail.set_env("CONFIG__BACKOFF__JITTER_ENABLED", "false");
        jail.set_env("CONFIG__BACKOFF__JITTER_SEED", "1337");

        let config = figment::Figment::new()
            .merge(Env::prefixed("CONFIG__").split("__"))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::Exponential {
                    initial_delay: Duration::from_millis(750).into(),
                    factor: 3.5,
                    max_delay: Nullable::Some(Duration::from_secs(20).into()),
                    max_retries: Nullable::Some(10),
                    max_total_delay: Some(Duration::from_secs(90).into()),
                    jitter_enabled: false,
                    jitter_seed: Some(1337),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn exponential_backoff_with_null_values() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("CONFIG__BACKOFF__STRATEGY", "Exponential");

        jail.set_env("CONFIG__BACKOFF__MAX_DELAY", "null");
        jail.set_env("CONFIG__BACKOFF__MAX_RETRIES", "null");

        let config = figment::Figment::new()
            .merge(Env::prefixed("CONFIG__").split("__"))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::Exponential {
                    initial_delay: defaults::delay(),
                    factor: defaults::factor(),
                    max_delay: Nullable::Null,
                    max_retries: Nullable::Null,
                    max_total_delay: defaults::max_total_delay(),
                    jitter_enabled: defaults::jitter_enabled(),
                    jitter_seed: defaults::jitter_seed(),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn fibonacci_backoff_with_defaults() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("CONFIG__BACKOFF__STRATEGY", "Fibonacci");

        let config = figment::Figment::new()
            .merge(Env::prefixed("CONFIG__").split("__"))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::Fibonacci {
                    initial_delay: defaults::delay(),
                    max_delay: defaults::max_delay(),
                    max_retries: defaults::max_retries(),
                    jitter_enabled: defaults::jitter_enabled(),
                    jitter_seed: defaults::jitter_seed(),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn fibonacci_backoff_with_custom_values() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("CONFIG__BACKOFF__STRATEGY", "Fibonacci");

        jail.set_env("CONFIG__BACKOFF__INITIAL_DELAY", "1234ms");
        jail.set_env("CONFIG__BACKOFF__MAX_DELAY", "123456789ns");
        jail.set_env("CONFIG__BACKOFF__MAX_RETRIES", "10");
        jail.set_env("CONFIG__BACKOFF__JITTER_ENABLED", "false");
        jail.set_env("CONFIG__BACKOFF__JITTER_SEED", "1337");

        let config = figment::Figment::new()
            .merge(Env::prefixed("CONFIG__").split("__"))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::Fibonacci {
                    initial_delay: Duration::from_millis(1234).into(),
                    max_delay: Nullable::Some(Duration::from_nanos(123456789).into()),
                    max_retries: Nullable::Some(10),
                    jitter_enabled: false,
                    jitter_seed: Some(1337),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn fibonacci_backoff_with_null_values() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("CONFIG__BACKOFF__STRATEGY", "Fibonacci");

        jail.set_env("CONFIG__BACKOFF__MAX_DELAY", "null");
        jail.set_env("CONFIG__BACKOFF__MAX_RETRIES", "null");

        let config = figment::Figment::new()
            .merge(Env::prefixed("CONFIG__").split("__"))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::Fibonacci {
                    initial_delay: defaults::delay(),
                    max_delay: Nullable::Null,
                    max_retries: Nullable::Null,
                    jitter_enabled: defaults::jitter_enabled(),
                    jitter_seed: defaults::jitter_seed(),
                }
            }
        );

        Ok(())
    });
}
