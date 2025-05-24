use backon::*;
use std::time::Duration;

#[derive(Debug)]
/// Supported backoffs.
pub enum Backoff {
    /// Constant backoff.
    Constant(ConstantBackoff),

    /// Exponential backoff.
    Exponential(ExponentialBackoff),

    /// Fibonacci backoff.
    Fibonacci(FibonacciBackoff),

    /// No backoff.
    NoBackoff,
}

impl Iterator for Backoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Backoff::Constant(c) => c.next(),
            Backoff::Exponential(e) => e.next(),
            Backoff::Fibonacci(f) => f.next(),
            Backoff::NoBackoff => None,
        }
    }
}
