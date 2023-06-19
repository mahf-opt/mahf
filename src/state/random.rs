//! An erased random number generator (RNG) powered by [rand]'s [`RngCore`].

use std::any::type_name;

use better_any::{Tid, TidAble};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;
use serde::Serialize;

use crate::state::CustomState;

/// A random number generator (RNG).
///
/// Any seedable [`RngCore`] can be used as backend. By default, [`ChaCha12Rng`] is used.
///
/// # Usages
///
/// This state is automatically inserted into the [`State`] by the [`Configuration`]
/// using [`Random::default`], if no generator is inserted manually.
/// Note that this means a random seed.
///
/// [`Configuration`]: crate::Configuration
///
/// # Reproducibility
///
/// As reproducibility and deterministic experiments are a core goal of MAHF, only seedable
/// [`RngCore`]s are allowed as backends.
/// This ensures that all random processes depend on the single seed.
///
/// # Examples
///
/// Using the [`random_mut`] method on [`State`] to retrieve the value:
///
/// [`evaluations`]: crate::State::random_mut
/// [`State`]: crate::State
///
/// ```
/// # use std::cell::RefMut;
/// use rand::Rng;
/// # use mahf::Problem;
/// use mahf::{State, state::Random};
///
/// // `state: State` is assumed to contain `Random`.
/// # pub fn example<P: Problem>(state: &mut State<P>) {
/// let mut rng: RefMut<Random> = state.random_mut();
/// let random_bool = rng.gen_bool(0.5);
/// # }
/// ```
#[derive(Tid)]
pub struct Random {
    config: RandomConfig,
    constructor: fn(u64) -> Random,
    inner: Box<dyn RngCore + Send>,
}

impl CustomState<'_> for Random {}

/// Describes the [Random] instance by its seed and the name of the underlying RNG.
#[derive(Debug, Clone, Serialize)]
pub struct RandomConfig {
    pub name: &'static str,
    pub seed: u64,
}

impl Random {
    /// Constructs a new random number generator from a given seed.
    ///
    /// By default, [`ChaCha12Rng`] is used as backend.
    /// To configure this, use [`Random::with_rng`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rand::Rng;
    /// use mahf::Random;
    ///
    /// let mut rng = Random::new(42);
    /// println!("A random number between 0 and 10: {}", rng.gen_range(0..=10));
    /// ```
    pub fn new(seed: u64) -> Self {
        Random::with_rng::<ChaCha12Rng>(seed)
    }

    /// Constructs a new random generator from a given seed and `RNG` as backend.
    ///
    /// # Examples
    ///
    /// Creating the generator with [`rand::rngs::StdRng`]:
    ///
    /// ```
    /// use rand::{Rng, rngs::StdRng};
    /// use mahf::Random;
    ///
    /// let mut rng = Random::with_rng::<StdRng>(42);
    /// println!("A random number between 0 and 10: {}", rng.gen_range(0..=10));
    /// ```
    pub fn with_rng<RNG>(seed: u64) -> Self
    where
        RNG: RngCore + SeedableRng + Send + 'static,
    {
        Random {
            config: RandomConfig {
                name: type_name::<RNG>(),
                seed,
            },
            constructor: |seed: u64| Random::with_rng::<RNG>(seed),
            inner: Box::new(RNG::seed_from_u64(seed)),
        }
    }

    /// Create a random generator for testing.
    ///
    /// This always uses the default random generator with seed `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rand::Rng;
    /// use mahf::Random;
    ///
    /// let mut rng1 = Random::testing();
    /// let mut rng2 = Random::testing();
    ///
    /// assert_eq!(rng1.gen_bool(0.5), rng2.gen_bool(0.5))
    /// ```
    pub fn testing() -> Self {
        Random::new(0)
    }

    /// Returns the generators configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use mahf::Random;
    ///
    /// let rng = Random::new(42);
    /// assert_eq!(rng.config().seed, 42);
    /// ```
    pub fn config(&self) -> &RandomConfig {
        &self.config
    }

    /// Creates an iterator of child generators that are seeded from the parent.
    ///
    /// The child generators use the same backend as the parent.
    ///
    /// # Examples
    ///
    /// Creating 10 child generators from a given generator:
    ///
    /// ```
    /// use rand::Rng;
    /// use mahf::Random;
    ///
    /// let mut rng = Random::new(42);
    /// let children: Vec<Random> = rng.iter_children().take(10).collect();
    /// ```
    ///
    /// Sending the child generators to threads with a deterministic workload
    /// enables reproducible pseudo-random numbers even across threads.
    ///
    /// ```
    /// use rand::Rng;
    /// use mahf::Random;
    ///
    /// let mut rng1 = Random::new(42);
    /// let mut children1: Vec<Random> = rng1.iter_children().take(10).collect();
    /// let numbers1: Vec<usize> = children1.iter_mut().map(|rng| rng.gen_range(0..10)).collect();
    ///
    /// let mut rng2 = Random::new(42);
    /// let mut children2: Vec<Random> = rng2.iter_children().take(10).collect();
    /// let numbers2: Vec<usize> = children2.iter_mut().map(|rng| rng.gen_range(0..10)).collect();
    ///
    /// assert_eq!(numbers1, numbers2);
    /// ```
    pub fn iter_children(&mut self) -> RandomIter {
        RandomIter { rng: self }
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
    /// Constructs a new random number generator from a random seed.
    ///
    /// The seed is generated using [rand::thread_rng].
    ///
    /// Use this method only if reproducibility is not important.
    fn default() -> Self {
        Random::new(rand::thread_rng().next_u64())
    }
}

/// An iterator that creates child generators from a [`Random`] number generator.
///
/// This `struct` is created by the [`Random::iter_children`] method or [`IntoIterator`].
pub struct RandomIter<'a> {
    rng: &'a mut Random,
}

impl<'a> Iterator for RandomIter<'a> {
    type Item = Random;

    fn next(&mut self) -> Option<Self::Item> {
        let seed = self.rng.next_u64();
        Some((self.rng.constructor)(seed))
    }
}

impl<'a> IntoIterator for &'a mut Random {
    type Item = Random;
    type IntoIter = RandomIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_children()
    }
}
