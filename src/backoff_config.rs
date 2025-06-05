use crate::*;
use duration_str::*;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(tag = "strategy")]
/// Configuration for [Backoff].
pub enum BackoffConfig {
    /// Configuration for [Backoff::Constant].
    Constant(ConstantBackoffConfig),

    /// Configuration for [Backoff::Exponential].
    Exponential(ExponentialBackoffConfig),

    /// Configuration for [Backoff::Fibonacci].
    Fibonacci(FibonacciBackoffConfig),

    /// Configuration for [Backoff::NoBackoff].
    NoBackoff,
}

impl From<ConstantBackoffConfig> for BackoffConfig {
    fn from(config: ConstantBackoffConfig) -> BackoffConfig {
        BackoffConfig::Constant(config)
    }
}

impl From<ExponentialBackoffConfig> for BackoffConfig {
    fn from(config: ExponentialBackoffConfig) -> BackoffConfig {
        BackoffConfig::Exponential(config)
    }
}

impl From<FibonacciBackoffConfig> for BackoffConfig {
    fn from(config: FibonacciBackoffConfig) -> BackoffConfig {
        BackoffConfig::Fibonacci(config)
    }
}

#[derive(Debug, smart_default::SmartDefault, Clone, Copy, Deserialize, PartialEq)]
/// Configuration for [Backoff::Constant].
pub struct ConstantBackoffConfig {
    /// Backoff delay.
    ///
    /// Defaults to `500 millis` - see [defaults::delay].
    #[serde(default = "defaults::delay", deserialize_with = "deserialize_duration")]
    #[default(defaults::delay())]
    pub delay: Duration,

    /// Maximum amount of retries.
    ///
    /// Defaults to `4` - see [defaults::max_retries].
    #[serde(default = "defaults::max_retries")]
    #[default(defaults::max_retries())]
    pub max_retries: usize,

    /// Whether jitter is enabled.
    ///
    /// Defaults to `true` - see [defaults::jitter_enabled].
    #[serde(default = "defaults::jitter_enabled")]
    #[default(defaults::jitter_enabled())]
    pub jitter_enabled: bool,

    /// Random seed to initialize the random jitter generator.
    ///
    /// Defaults to `None` - see [defaults::jitter_seed].
    #[serde(default = "defaults::jitter_seed")]
    #[default(defaults::jitter_seed())]
    pub jitter_seed: Option<u64>,
}

#[derive(Debug, smart_default::SmartDefault, Clone, Copy, Deserialize, PartialEq)]
/// Configuration for [Backoff::Exponential].
pub struct ExponentialBackoffConfig {
    /// Initial backoff delay.
    ///
    /// Defaults to `500 millis` - see [defaults::delay].
    #[serde(default = "defaults::delay", deserialize_with = "deserialize_duration")]
    #[default(defaults::delay())]
    pub initial_delay: Duration,

    /// Backoff factor.
    ///
    /// Defaults to `2.0` - see [defaults::factor].
    #[serde(default = "defaults::factor")]
    #[default(defaults::factor())]
    pub factor: f32,

    /// Maximum backoff delay.
    ///
    /// Defaults to `30 seconds` - see [defaults::max_delay].
    #[serde(
        default = "defaults::max_delay",
        deserialize_with = "deserialize_duration"
    )]
    #[default(defaults::max_delay())]
    pub max_delay: Duration,

    /// Maximum amount of retries.
    ///
    /// Defaults to `4` - see [defaults::max_retries].
    #[serde(default = "defaults::max_retries")]
    #[default(defaults::max_retries())]
    pub max_retries: usize,

    /// Maximum total backoff delay.
    ///
    /// Defaults to `60 seconds` - see [defaults::max_total_delay]
    #[serde(
        default = "defaults::max_total_delay",
        deserialize_with = "deserialize_duration"
    )]
    #[default(defaults::max_total_delay())]
    pub max_total_delay: Duration,

    /// Whether jitter is enabled.
    ///
    /// Defaults to `true` - see [defaults::jitter_enabled].
    #[serde(default = "defaults::jitter_enabled")]
    #[default(defaults::jitter_enabled())]
    pub jitter_enabled: bool,

    /// Random seed to initialize the random jitter generator.
    ///
    /// Defaults to `None` - see [defaults::jitter_seed].
    #[serde(default = "defaults::jitter_seed")]
    #[default(defaults::jitter_seed())]
    pub jitter_seed: Option<u64>,
}

#[derive(Debug, smart_default::SmartDefault, Clone, Copy, Deserialize, PartialEq)]
/// Configuration for [Backoff::Fibonacci].
pub struct FibonacciBackoffConfig {
    /// Initial backoff delay.
    ///
    /// Defaults to `500 millis` - see [defaults::delay].
    #[serde(default = "defaults::delay", deserialize_with = "deserialize_duration")]
    #[default(defaults::delay())]
    pub initial_delay: Duration,

    /// Maximum backoff delay.
    ///
    /// Defaults to `30 seconds` - see [defaults::max_delay].
    #[serde(
        default = "defaults::max_delay",
        deserialize_with = "deserialize_duration"
    )]
    #[default(defaults::max_delay())]
    pub max_delay: Duration,

    /// Maximum amount of retries.
    ///
    /// Defaults to `4` - see [defaults::max_retries].
    #[serde(default = "defaults::max_retries")]
    #[default(defaults::max_retries())]
    pub max_retries: usize,

    /// Whether jitter is enabled.
    ///
    /// Defaults to `true` - see [defaults::jitter_enabled].
    #[serde(default = "defaults::jitter_enabled")]
    #[default(defaults::jitter_enabled())]
    pub jitter_enabled: bool,

    /// Random seed to initialize the random jitter generator.
    ///
    /// Defaults to `None` - see [defaults::jitter_seed].
    #[serde(default = "defaults::jitter_seed")]
    #[default(defaults::jitter_seed())]
    pub jitter_seed: Option<u64>,
}

impl backon::BackoffBuilder for BackoffConfig {
    type Backoff = Backoff;

    fn build(self) -> Backoff {
        match self {
            BackoffConfig::Constant(ConstantBackoffConfig {
                delay,
                max_retries,
                jitter_enabled,
                jitter_seed,
            }) => {
                let mut builder = backon::ConstantBuilder::new()
                    .with_delay(delay)
                    .with_max_times(max_retries);

                if jitter_enabled {
                    builder = builder.with_jitter();
                }

                if let Some(jitter_seed) = jitter_seed {
                    builder = builder.with_jitter_seed(jitter_seed);
                }

                Backoff::Constant(builder.build())
            }

            BackoffConfig::Exponential(ExponentialBackoffConfig {
                initial_delay,
                factor,
                max_delay,
                max_retries,
                max_total_delay,
                jitter_enabled,
                jitter_seed,
            }) => {
                let mut builder = backon::ExponentialBuilder::new()
                    .with_min_delay(initial_delay)
                    .with_factor(factor)
                    .with_max_delay(max_delay)
                    .with_max_times(max_retries)
                    .with_total_delay(Some(max_total_delay));

                if jitter_enabled {
                    builder = builder.with_jitter();
                }

                if let Some(jitter_seed) = jitter_seed {
                    builder = builder.with_jitter_seed(jitter_seed);
                }

                Backoff::Exponential(builder.build())
            }

            BackoffConfig::Fibonacci(FibonacciBackoffConfig {
                initial_delay,
                max_delay,
                max_retries,
                jitter_enabled,
                jitter_seed,
            }) => {
                let mut builder = backon::FibonacciBuilder::new()
                    .with_min_delay(initial_delay)
                    .with_max_delay(max_delay)
                    .with_max_times(max_retries);

                if jitter_enabled {
                    builder = builder.with_jitter();
                }

                if let Some(jitter_seed) = jitter_seed {
                    builder = builder.with_jitter_seed(jitter_seed);
                }

                Backoff::Fibonacci(builder.build())
            }

            BackoffConfig::NoBackoff => Backoff::NoBackoff,
        }
    }
}

/// Contains the defaults used by the [crate::BackoffConfig].
pub mod defaults {
    use std::time::Duration;

    /// Default value for constant / initial backoff delay.
    pub const fn delay() -> Duration {
        Duration::from_millis(500)
    }

    /// Default value for max retries.
    pub const fn max_retries() -> usize {
        4
    }

    /// Default value whether jitter is enabled.
    pub const fn jitter_enabled() -> bool {
        true
    }

    /// Default value for jitter seed.
    pub const fn jitter_seed() -> Option<u64> {
        None
    }

    /// Default value for backoff factor.
    pub const fn factor() -> f32 {
        2.0
    }

    /// Default value for max backoff delay.
    pub const fn max_delay() -> Duration {
        Duration::from_secs(30)
    }

    /// Default value for max total backoff delay.
    pub const fn max_total_delay() -> Duration {
        Duration::from_secs(60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backon::BackoffBuilder;
    use std::time::Duration;
    #[test]
    fn backoff_config_from() {
        let constant_config = ConstantBackoffConfig {
            delay: Duration::from_secs(1),
            max_retries: 3,
            jitter_enabled: false,
            jitter_seed: None,
        };
        let backoff_config: BackoffConfig = constant_config.into();
        assert_eq!(backoff_config, BackoffConfig::Constant(constant_config));

        let exponential_config = ExponentialBackoffConfig {
            initial_delay: Duration::from_millis(100),
            factor: 2_f32,
            max_delay: Duration::from_millis(800),
            max_retries: 5,
            max_total_delay: Duration::from_secs(1000),
            jitter_enabled: false,
            jitter_seed: None,
        };
        let backoff_config: BackoffConfig = exponential_config.into();
        assert_eq!(
            backoff_config,
            BackoffConfig::Exponential(exponential_config)
        );

        let fibonacci_config = FibonacciBackoffConfig {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(800),
            max_retries: 5,
            jitter_enabled: false,
            jitter_seed: None,
        };
        let backoff_config: BackoffConfig = fibonacci_config.into();
        assert_eq!(backoff_config, BackoffConfig::Fibonacci(fibonacci_config));
    }

    #[test]
    fn defaults() {
        let constant = ConstantBackoffConfig::default();
        assert_eq!(
            constant,
            ConstantBackoffConfig {
                delay: defaults::delay(),
                max_retries: defaults::max_retries(),
                jitter_enabled: defaults::jitter_enabled(),
                jitter_seed: defaults::jitter_seed(),
            }
        );

        let exponential = ExponentialBackoffConfig::default();
        assert_eq!(
            exponential,
            ExponentialBackoffConfig {
                initial_delay: defaults::delay(),
                factor: defaults::factor(),
                max_delay: defaults::max_delay(),
                max_retries: defaults::max_retries(),
                max_total_delay: defaults::max_total_delay(),
                jitter_enabled: defaults::jitter_enabled(),
                jitter_seed: defaults::jitter_seed(),
            }
        );

        let fibonacci = FibonacciBackoffConfig::default();
        assert_eq!(
            fibonacci,
            FibonacciBackoffConfig {
                initial_delay: defaults::delay(),
                max_delay: defaults::max_delay(),
                max_retries: defaults::max_retries(),
                jitter_enabled: defaults::jitter_enabled(),
                jitter_seed: defaults::jitter_seed(),
            }
        );
    }

    #[test]
    fn constant_backoff_config_to_backoff() {
        let config = BackoffConfig::Constant(ConstantBackoffConfig {
            delay: Duration::from_secs(1),
            max_retries: 3,
            jitter_enabled: false,
            jitter_seed: None,
        });

        let backoff = config.build();
        assert!(matches!(backoff, Backoff::Constant(_)));

        assert_eq!(
            backoff
                .take(100)
                .map(|duration| duration.as_millis())
                .collect::<Vec<_>>(),
            vec![1000; 3]
        );
    }

    #[test]
    fn constant_backoff_config_to_backoff_with_jitter() {
        let config = BackoffConfig::Constant(ConstantBackoffConfig {
            delay: Duration::from_secs(1),
            max_retries: 3,
            jitter_enabled: true,
            jitter_seed: Some(0),
        });

        let backoff = config.build();
        assert!(matches!(backoff, Backoff::Constant(_)));

        assert_eq!(
            backoff
                .take(100)
                .map(|duration| duration.as_millis())
                .collect::<Vec<_>>(),
            vec![1552, 1096, 1593]
        );
    }

    #[test]
    fn exponential_backoff_config_to_backoff() {
        let config = BackoffConfig::Exponential(ExponentialBackoffConfig {
            initial_delay: Duration::from_millis(100),
            factor: 2_f32,
            max_delay: Duration::from_millis(800),
            max_retries: 5,
            max_total_delay: Duration::from_secs(1000),
            jitter_enabled: false,
            jitter_seed: None,
        });

        let backoff = config.build();
        assert!(matches!(backoff, Backoff::Exponential(_)));

        assert_eq!(
            backoff
                .take(100)
                .map(|duration| duration.as_millis())
                .collect::<Vec<_>>(),
            vec![100, 200, 400, 800, 800]
        );
    }

    #[test]
    fn exponential_backoff_config_to_backoff_with_jitter() {
        let config = BackoffConfig::Exponential(ExponentialBackoffConfig {
            initial_delay: Duration::from_millis(100),
            factor: 2_f32,
            max_delay: Duration::from_millis(800),
            max_retries: 5,
            max_total_delay: Duration::from_secs(1000),
            jitter_enabled: true,
            jitter_seed: Some(0),
        });

        let backoff = config.build();
        assert!(matches!(backoff, Backoff::Exponential(_)));

        assert_eq!(
            backoff
                .take(100)
                .map(|duration| duration.as_millis())
                .collect::<Vec<_>>(),
            vec![155, 219, 637, 923, 1102]
        );
    }

    #[test]
    fn exponential_backoff_config_to_backoff_with_max_total_delay() {
        let config = BackoffConfig::Exponential(ExponentialBackoffConfig {
            initial_delay: Duration::from_millis(100),
            factor: 2_f32,
            max_delay: Duration::from_millis(800),
            max_retries: 5,
            max_total_delay: Duration::from_millis(1500 + 1),
            jitter_enabled: false,
            jitter_seed: None,
        });

        let backoff = config.build();
        assert!(matches!(backoff, Backoff::Exponential(_)));

        assert_eq!(
            backoff
                .take(100)
                .map(|duration| duration.as_millis())
                .collect::<Vec<_>>(),
            vec![100, 200, 400, 800]
        );
    }

    #[test]
    fn fibonacci_backoff_config_to_backoff() {
        let config = BackoffConfig::Fibonacci(FibonacciBackoffConfig {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(800),
            max_retries: 5,
            jitter_enabled: false,
            jitter_seed: None,
        });

        let backoff = config.build();
        assert!(matches!(backoff, Backoff::Fibonacci(_)));

        assert_eq!(
            backoff
                .take(usize::MAX)
                .map(|duration| duration.as_millis())
                .collect::<Vec<_>>(),
            vec![100, 100, 200, 300, 500]
        );
    }

    #[test]
    fn fibonacci_backoff_config_to_backoff_with_jitter() {
        let config = BackoffConfig::Fibonacci(FibonacciBackoffConfig {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(800),
            max_retries: 5,
            jitter_enabled: true,
            jitter_seed: Some(0),
        });

        let backoff = config.build();
        assert!(matches!(backoff, Backoff::Fibonacci(_)));

        assert_eq!(
            backoff
                .take(usize::MAX)
                .map(|duration| duration.as_millis())
                .collect::<Vec<_>>(),
            vec![155, 109, 259, 315, 537]
        );
    }

    #[test]
    fn no_backoff_backoff_config_to_backoff() {
        let config = BackoffConfig::NoBackoff;

        let mut backoff = config.build();
        assert!(matches!(backoff, Backoff::NoBackoff));

        assert!(backoff.next().is_none());
    }
}
