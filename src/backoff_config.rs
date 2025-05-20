use crate::nullable::Nullable;
use crate::*;
use duration_string::DurationString;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(tag = "strategy")]
/// Configuration for [Backoff].
pub enum BackoffConfig {
    /// Configuration for [Backoff::Constant].
    Constant {
        /// Backoff delay.
        ///
        /// Defaults to `500 millis` - see [defaults::delay].
        #[serde(default = "defaults::delay")]
        delay: DurationString,

        /// Maximum amount of retries.
        ///
        /// Removes the retry limit if the value is [Nullable::Null].
        ///
        /// Defaults to `4` - see [defaults::max_retries].
        #[serde(default = "defaults::max_retries")]
        max_retries: Nullable<usize>,

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
        #[serde(default = "defaults::delay")]
        initial_delay: DurationString,

        /// Backoff factor.
        ///
        /// Defaults to `2.0` - see [defaults::factor].
        #[serde(default = "defaults::factor")]
        factor: f32,

        /// Maximum backoff delay.
        ///
        /// Removes the max backoff delay limit if the value is [Nullable::Null].
        ///
        /// Defaults to `60 seconds` - see [defaults::max_delay].
        #[serde(default = "defaults::max_delay")]
        max_delay: Nullable<DurationString>,

        /// Maximum amount of retries.
        ///
        /// Removes the retry limit if the value is [Nullable::Null].
        ///
        /// Defaults to `4` - see [defaults::max_retries].
        #[serde(default = "defaults::max_retries")]
        max_retries: Nullable<usize>,

        /// Maximum total backoff delay.
        ///
        /// Defaults to `None` - see [defaults::max_total_delay]
        #[serde(default = "defaults::max_total_delay")]
        max_total_delay: Option<DurationString>,

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
        #[serde(default = "defaults::delay")]
        initial_delay: DurationString,

        /// Maximum backoff delay.
        ///
        /// Removes the max backoff delay limit if the value is [Nullable::Null].
        ///
        /// Defaults to `60 seconds` - see [defaults::max_delay].
        #[serde(default = "defaults::max_delay")]
        max_delay: Nullable<DurationString>,

        /// Maximum amount of retries.
        ///
        /// Removes the retry limit if the value is [Nullable::Null].
        ///
        /// Defaults to `4` - see [defaults::max_retries].
        #[serde(default = "defaults::max_retries")]
        max_retries: Nullable<usize>,

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
                let mut builder = backon::ConstantBuilder::new().with_delay(delay.into());

                builder = match max_retries {
                    Nullable::Some(max_retries) => builder.with_max_times(max_retries),
                    Nullable::Null => builder.without_max_times(),
                };

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
                    .with_min_delay(initial_delay.into())
                    .with_factor(factor);

                builder = match max_delay {
                    Nullable::Some(max_delay) => builder.with_max_delay(max_delay.into()),
                    Nullable::Null => builder.without_max_delay(),
                };

                builder = match max_retries {
                    Nullable::Some(max_retries) => builder.with_max_times(max_retries),
                    Nullable::Null => builder.without_max_times(),
                };

                builder = builder.with_total_delay(max_total_delay.map(Into::into));

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
                let mut builder =
                    backon::FibonacciBuilder::new().with_min_delay(initial_delay.into());

                builder = match max_delay {
                    Nullable::Some(max_delay) => builder.with_max_delay(max_delay.into()),
                    Nullable::Null => builder.without_max_delay(),
                };

                builder = match max_retries {
                    Nullable::Some(max_retries) => builder.with_max_times(max_retries),
                    Nullable::Null => builder.without_max_times(),
                };

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
    use crate::nullable::Nullable;
    use duration_string::DurationString;
    use std::time::Duration;

    /// Default value for constant / initial backoff delay.
    pub fn delay() -> DurationString {
        Duration::from_millis(500).into()
    }

    /// Default value for max retries.
    pub const fn max_retries() -> Nullable<usize> {
        Nullable::Some(4)
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
    pub fn max_delay() -> Nullable<DurationString> {
        Nullable::Some(Duration::from_secs(30).into())
    }

    /// Default value for max total backoff delay.
    pub fn max_total_delay() -> Option<DurationString> {
        Some(Duration::from_secs(60).into())
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
            delay: Duration::from_secs(1).into(),
            max_retries: Nullable::Some(3),
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
            initial_delay: Duration::from_millis(100).into(),
            factor: 2_f32,
            max_delay: Nullable::Some(Duration::from_millis(800).into()),
            max_retries: Nullable::Some(5),
            max_total_delay: None,
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
            initial_delay: Duration::from_millis(100).into(),
            factor: 2_f32,
            max_delay: Nullable::Some(Duration::from_millis(800).into()),
            max_retries: Nullable::Some(5),
            max_total_delay: Some(Duration::from_millis(1500 + 1).into()),
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
            initial_delay: Duration::from_millis(100).into(),
            max_delay: Nullable::Some(Duration::from_millis(800).into()),
            max_retries: Nullable::Some(5),
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
