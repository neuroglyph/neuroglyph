// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Time abstraction for dependency injection

/// Trait for time operations to enable testing
pub trait Clock: Send + Sync + Clone {
    /// Get the current timestamp in seconds since Unix epoch
    fn now(&self) -> i64;
}

/// Default implementation using system time
#[derive(Clone)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> i64 {
        chrono::Utc::now().timestamp()
    }
}

#[cfg(test)]
pub mod test_doubles {
    use super::*;

    /// Fixed clock for deterministic testing
    #[derive(Clone)]
    pub struct FixedClock {
        timestamp: i64,
    }

    impl FixedClock {
        pub fn new(timestamp: i64) -> Self {
            Self { timestamp }
        }
    }

    impl Clock for FixedClock {
        fn now(&self) -> i64 {
            self.timestamp
        }
    }
}
