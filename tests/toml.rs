mod common;

use common::*;

use backoff_config::*;
use figment::providers::{Data, Toml};
use std::time::Duration;

const CONFIG_TOML_PATH: &str = "config.toml";

#[test]
fn constant_backoff_with_defaults() {
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
fn constant_backoff_with_custom_values() {
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
                    delay: Duration::from_secs(123),
                    max_retries: 456,
                    jitter_enabled: false,
                    jitter_seed: Some(1337),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn exponential_backoff_with_defaults() {
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
fn exponential_backoff_with_custom_values() {
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
                    initial_delay: Duration::from_millis(750),
                    factor: 3.5,
                    max_delay: Duration::from_secs(20),
                    max_retries: 10,
                    max_total_delay: Duration::from_secs(90),
                    jitter_enabled: false,
                    jitter_seed: Some(1337),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn fibonacci_backoff_with_defaults() {
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
fn fibonacci_backoff_with_custom_values() {
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
                    initial_delay: Duration::from_millis(750),
                    max_delay: Duration::from_secs(20),
                    max_retries: 10,
                    jitter_enabled: false,
                    jitter_seed: Some(1337),
                }
            }
        );

        Ok(())
    });
}

#[test]
fn no_backoff() {
    figment::Jail::expect_with(|jail| {
        jail.create_file(
            CONFIG_TOML_PATH,
            r#"
                [backoff]
                strategy = "NoBackoff"
            "#,
        )?;

        let config = figment::Figment::new()
            .merge(Data::<Toml>::file(CONFIG_TOML_PATH))
            .extract::<Config>()?;

        assert_eq!(
            config,
            Config {
                backoff: BackoffConfig::NoBackoff
            }
        );

        Ok(())
    });
}
