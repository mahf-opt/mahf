use rand::{RngCore, SeedableRng};
use serde::Serialize;
use std::any::type_name;

pub struct Random {
    config: RandomConfig,
    inner: Box<dyn RngCore>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct RandomConfig {
    pub name: &'static str,
    pub seed: u64,
}

impl Random {
    pub fn new<RNG>(seed: u64) -> Self
    where
        RNG: RngCore + SeedableRng + 'static,
    {
        Random {
            config: RandomConfig {
                name: type_name::<RNG>(),
                seed,
            },
            inner: Box::new(RNG::seed_from_u64(seed)),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Random::new::<rand::rngs::StdRng>(seed)
    }

    pub fn testing() -> Self {
        Random::seeded(0)
    }

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
    fn default() -> Self {
        Random::seeded(rand::thread_rng().next_u64())
    }
}
