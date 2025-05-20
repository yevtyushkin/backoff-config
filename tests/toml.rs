mod common;

use common::*;

use backoff_config::*;
use figment::providers::{Data, Toml};
use std::time::Duration;

const CONFIG_TOML_PATH: &str = "config.toml";

#[test]
fn constant_backoff_with_defaults_toml() {
    figment::Jail::expect_with(|jail| {
        jail.create_file(
            CONFIG_TOML_PATH,
            r#"
                [backoff]
                strategy = "Constant"
            "#,
        )?;

        let config = figment::Figment::new()
            .merge(Data::<Toml>::file(CONFIG_TOML_PATH))
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
fn constant_backoff_with_custom_values_toml() {
    figment::Jail::expect_with(|jail| {
        jail.create_file(
            CONFIG_TOML_PATH,
            r#"
                [backoff]
                strategy = "Constant"
                delay = "123 s"
                max_retries = 456
                jitter_enabled = false
                jitter_seed = 1337
            "#,
        )?;

        let config = figment::Figment::new()
            .merge(Data::<Toml>::file(CONFIG_TOML_PATH))
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
fn constant_backoff_with_null_values_toml() {
    figment::Jail::expect_with(|jail| {
        jail.create_file(
            CONFIG_TOML_PATH,
            r#"
                [backoff]
                strategy = "Constant"
                max_retries = "null"
            "#,
        )?;

        let config = figment::Figment::new()
            .merge(Data::<Toml>::file(CONFIG_TOML_PATH))
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
fn exponential_backoff_with_defaults_toml() {
    figment::Jail::expect_with(|jail| {
        jail.create_file(
            CONFIG_TOML_PATH,
            r#"
                [backoff]
                strategy = "Exponential"
            "#,
        )?;

        let config = figment::Figment::new()
            .merge(Data::<Toml>::file(CONFIG_TOML_PATH))
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
fn exponential_backoff_with_custom_values_toml() {
    figment::Jail::expect_with(|jail| {
        jail.create_file(
            CONFIG_TOML_PATH,
            r#"
                [backoff]
                strategy = "Exponential"
                initial_delay = "750 ms"
                factor = 3.5
                max_delay = "20 s"
                max_retries = 10
                max_total_delay = "90 s"
                jitter_enabled = false
                jitter_seed = 1337
            "#,
        )?;

        let config = figment::Figment::new()
            .merge(Data::<Toml>::file(CONFIG_TOML_PATH))
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
fn exponential_backoff_with_null_values_toml() {
    figment::Jail::expect_with(|jail| {
        jail.create_file(
            CONFIG_TOML_PATH,
            r#"
                [backoff]
                strategy = "Exponential"
                max_delay = "null"
                max_retries = "null"
            "#,
        )?;

        let config = figment::Figment::new()
            .merge(Data::<Toml>::file(CONFIG_TOML_PATH))
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
fn fibonacci_backoff_with_defaults_toml() {
    figment::Jail::expect_with(|jail| {
        jail.create_file(
            CONFIG_TOML_PATH,
            r#"
                [backoff]
                strategy = "Fibonacci"
            "#,
        )?;

        let config = figment::Figment::new()
            .merge(Data::<Toml>::file(CONFIG_TOML_PATH))
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
fn fibonacci_backoff_with_custom_values_toml() {
    figment::Jail::expect_with(|jail| {
        jail.create_file(
            CONFIG_TOML_PATH,
            r#"
                [backoff]
                strategy = "Fibonacci"
                initial_delay = "750 ms"
                max_delay = "20 s"
                max_retries = 10
                jitter_enabled = false
                jitter_seed = 1337
            "#,
        )?;

        let config = figment::Figment::new()
            .merge(Data::<Toml>::file(CONFIG_TOML_PATH))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::Fibonacci {
                    initial_delay: Duration::from_millis(750).into(),
                    max_delay: Nullable::Some(Duration::from_secs(20).into()),
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
fn fibonacci_backoff_with_null_values_toml() {
    figment::Jail::expect_with(|jail| {
        jail.create_file(
            CONFIG_TOML_PATH,
            r#"
                [backoff]
                strategy = "Fibonacci"
                max_delay = "null"
                max_retries = "null"
            "#,
        )?;

        let config = figment::Figment::new()
            .merge(Data::<Toml>::file(CONFIG_TOML_PATH))
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
