use crate::*;
use duration_str::*;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(tag = "strategy")]
/// Configuration for [Backoff].
pub enum BackoffConfig {
    /// Configuration for [Backoff::Constant].
    Constant {
        /// Backoff delay.
        ///
        /// Defaults to `500 millis` - see [defaults::delay].
        #[serde(default = "defaults::delay", deserialize_with = "deserialize_duration")]
        delay: Duration,

        /// Maximum amount of retries.
        ///
        /// Defaults to `4` - see [defaults::max_retries].
        #[serde(default = "defaults::max_retries")]
        max_retries: usize,

        /// Whether jitter is enabled.
        ///
        /// Defaults to `true` - see [defaults::jitter_enabled].
        #[serde(default = "defaults::jitter_enabled")]
        jitter_enabled: bool,

        /// Random seed to initialize the random jitter generator.
        ///
        /// Defaults to `None` - see [defaults::jitter_seed].
        #[serde(default = "defaults::jitter_seed")]
        jitter_seed: Option<u64>,
    },

    /// Configuration for [Backoff::Exponential].
    Exponential {
        /// Initial backoff delay.
        ///
        /// Defaults to `500 millis` - see [defaults::delay].
        #[serde(default = "defaults::delay", deserialize_with = "deserialize_duration")]
        initial_delay: Duration,

        /// Backoff factor.
        ///
        /// Defaults to `2.0` - see [defaults::factor].
        #[serde(default = "defaults::factor")]
        factor: f32,

        /// Maximum backoff delay.
        ///
        /// Defaults to `60 seconds` - see [defaults::max_delay].
        #[serde(
            default = "defaults::max_delay",
            deserialize_with = "deserialize_duration"
        )]
        max_delay: Duration,

        /// Maximum amount of retries.
        ///
        /// Defaults to `4` - see [defaults::max_retries].
        #[serde(default = "defaults::max_retries")]
        max_retries: usize,

        /// Maximum total backoff delay.
        ///
        /// Defaults to `None` - see [defaults::max_total_delay]
        #[serde(
            default = "defaults::max_total_delay",
            deserialize_with = "deserialize_duration"
        )]
        max_total_delay: Duration,

        /// Whether jitter is enabled.
        ///
        /// Defaults to `true` - see [defaults::jitter_enabled].
        #[serde(default = "defaults::jitter_enabled")]
        jitter_enabled: bool,

        /// Random seed to initialize the random jitter generator.
        ///
        /// Defaults to `None` - see [defaults::jitter_seed].
        #[serde(default = "defaults::jitter_seed")]
        jitter_seed: Option<u64>,
    },

    /// Configuration for [Backoff::Fibonacci].
    Fibonacci {
        /// Initial backoff delay.
        ///
        /// Defaults to `500 millis` - see [defaults::delay].
        #[serde(default = "defaults::delay", deserialize_with = "deserialize_duration")]
        initial_delay: Duration,

        /// Maximum backoff delay.
        ///
        /// Defaults to `60 seconds` - see [defaults::max_delay].
        #[serde(
            default = "defaults::max_delay",
            deserialize_with = "deserialize_duration"
        )]
        max_delay: Duration,

        /// Maximum amount of retries.
        ///
        /// Defaults to `4` - see [defaults::max_retries].
        #[serde(default = "defaults::max_retries")]
        max_retries: usize,

        /// Whether jitter is enabled.
        ///
        /// Defaults to `true` - see [defaults::jitter_enabled].
        #[serde(default = "defaults::jitter_enabled")]
        jitter_enabled: bool,

        /// Random seed to initialize the random jitter generator.
        ///
        /// Defaults to `None` - see [defaults::jitter_seed].
        #[serde(default = "defaults::jitter_seed")]
        jitter_seed: Option<u64>,
    },
}

impl backon::BackoffBuilder for BackoffConfig {
    type Backoff = Backoff;

    fn build(self) -> Backoff {
        match self {
            BackoffConfig::Constant {
                delay,
                max_retries,
                jitter_enabled,
                jitter_seed,
            } => {
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

            BackoffConfig::Exponential {
                initial_delay,
                factor,
                max_delay,
                max_retries,
                max_total_delay,
                jitter_enabled,
                jitter_seed,
            } => {
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

            BackoffConfig::Fibonacci {
                initial_delay,
                max_delay,
                max_retries,
                jitter_enabled,
                jitter_seed,
            } => {
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
    fn constant_backoff_config_to_backoff() {
        let config = BackoffConfig::Constant {
            delay: Duration::from_secs(1),
            max_retries: 3,
            jitter_enabled: false,
            jitter_seed: None,
        };

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
    fn exponential_backoff_config_to_backoff() {
        let config = BackoffConfig::Exponential {
            initial_delay: Duration::from_millis(100),
            factor: 2_f32,
            max_delay: Duration::from_millis(800),
            max_retries: 5,
            max_total_delay: Duration::from_secs(1000),
            jitter_enabled: false,
            jitter_seed: None,
        };

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
    fn exponential_backoff_config_to_backoff_with_max_total_delay() {
        let config = BackoffConfig::Exponential {
            initial_delay: Duration::from_millis(100),
            factor: 2_f32,
            max_delay: Duration::from_millis(800),
            max_retries: 5,
            max_total_delay: Duration::from_millis(1500 + 1),
            jitter_enabled: false,
            jitter_seed: None,
        };

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
        let config = BackoffConfig::Fibonacci {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(800),
            max_retries: 5,
            jitter_enabled: false,
            jitter_seed: None,
        };

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
}
