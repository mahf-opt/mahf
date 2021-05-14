use rand::{RngCore, SeedableRng};
use serde::Serialize;
use std::any::type_name;

pub struct Random {
    config: RandomDescription,
    inner: Box<dyn RngCore>,
}

pub enum RandomConfig {
    Automatic,
    Seeded(u64),
    Custom(Random),
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct RandomDescription {
    pub name: &'static str,
    pub seed: u64,
}

impl Random {
    pub fn new<RNG>(seed: u64) -> Self
    where
        RNG: RngCore + SeedableRng + 'static,
    {
        Random {
            config: RandomDescription {
                name: type_name::<RNG>(),
                seed,
            },
            inner: Box::new(RNG::seed_from_u64(seed)),
        }
    }

    pub fn std_rng(seed: u64) -> Self {
        Random::new::<rand::rngs::StdRng>(seed)
    }

    pub fn test_rng() -> Self {
        Random::std_rng(0)
    }

    pub fn config(&self) -> RandomDescription {
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

impl Default for RandomConfig {
    fn default() -> Self {
        RandomConfig::Automatic
    }
}

impl From<RandomConfig> for Random {
    fn from(config: RandomConfig) -> Self {
        match config {
            RandomConfig::Automatic => Random::std_rng(rand::thread_rng().next_u64()),
            RandomConfig::Seeded(seed) => Random::std_rng(seed),
            RandomConfig::Custom(rng) => rng,
        }
    }
}
