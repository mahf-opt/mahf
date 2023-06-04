//! Abstractions for the [rand] crate.

use std::any::type_name;

use better_any::{Tid, TidAble};
use rand::{RngCore, SeedableRng};
use serde::Serialize;

use crate::state::CustomState;

/// A random number generator.
///
/// This can only be backed by seedable [`RngCore`]s to allow reconstruction.
#[derive(Tid)]
pub struct Random {
    config: RandomConfig,
    inner: Box<dyn RngCore + Send>,
}

impl CustomState<'_> for Random {}

/// Describes a [Random] generator.
///
/// Useful for debugging and logging.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct RandomConfig {
    pub name: &'static str,
    pub seed: u64,
}

impl Random {
    /// Create a new random generator with the given seed.
    pub fn new<RNG>(seed: u64) -> Self
    where
        RNG: RngCore + SeedableRng + Send + 'static,
    {
        Random {
            config: RandomConfig {
                name: type_name::<RNG>(),
                seed,
            },
            inner: Box::new(RNG::seed_from_u64(seed)),
        }
    }

    /// Create a standard random generator with the given seed.
    pub fn seeded(seed: u64) -> Self {
        Random::new::<rand::rngs::StdRng>(seed)
    }

    /// Create a random generator for testing.
    ///
    /// This always uses the same seed.
    pub fn testing() -> Self {
        Random::seeded(0)
    }

    /// Returns the generators configuration.
    pub fn config(&self) -> RandomConfig {
        self.config
    }
}

impl RngCore for Random {
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.inner.try_fill_bytes(dest)
    }
}

impl Default for Random {
    /// Creates a standard generator with a random seed.
    ///
    /// The seed will be generated with [rand::thread_rng].
    fn default() -> Self {
        Random::seeded(rand::thread_rng().next_u64())
    }
}
