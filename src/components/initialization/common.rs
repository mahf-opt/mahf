//! Common initialization components.

use rand::distributions::uniform::SampleUniform;
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult,
    components::{
        initialization::{functional as f, initialization, Initialization},
        Component,
    },
    problems::{LimitedVectorProblem, VectorProblem},
    state::random::Random,
    Problem, State,
};

/// Initializes an empty population.
#[derive(Clone, Serialize, Deserialize)]
pub struct Empty;

impl Empty {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P: Problem> Component<P> for Empty {
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.populations_mut().push(Vec::new());
        Ok(())
    }
}

/// Generates uniformly distributed solutions within the boundaries of the search space.
#[derive(Clone, Serialize, Deserialize)]
pub struct RandomSpread {
    /// Size of the population to be generated.
    pub population_size: u32,
}

impl RandomSpread {
    pub fn from_params(population_size: u32) -> Self {
        Self { population_size }
    }

    pub fn new<P, D>(population_size: u32) -> Box<dyn Component<P>>
    where
        D: SampleUniform + Clone + PartialOrd + 'static,
        P: LimitedVectorProblem<Element = D>,
    {
        Box::new(Self::from_params(population_size))
    }
}

impl<P, D> Initialization<P> for RandomSpread
where
    D: SampleUniform + Clone + PartialOrd + 'static,
    P: LimitedVectorProblem<Element = D>,
{
    fn initialize(&self, problem: &P, rng: &mut Random) -> Vec<P::Encoding> {
        f::random_spread(&problem.domain(), self.population_size as usize, rng)
    }
}

impl<P, D> Component<P> for RandomSpread
where
    D: SampleUniform + Clone + PartialOrd + 'static,
    P: LimitedVectorProblem<Element = D>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        initialization(self, problem, state)
    }
}

/// Generates random permutations.
#[derive(Clone, Serialize, Deserialize)]
pub struct RandomPermutation {
    /// Size of the population to be generated.
    pub population_size: u32,
}

impl RandomPermutation {
    pub fn from_params(population_size: u32) -> Self {
        Self { population_size }
    }

    pub fn new<P>(population_size: u32) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = usize>,
    {
        Box::new(Self::from_params(population_size))
    }
}

impl<P> Initialization<P> for RandomPermutation
where
    P: VectorProblem<Element = usize>,
{
    fn initialize(&self, problem: &P, rng: &mut Random) -> Vec<P::Encoding> {
        f::random_permutation(problem.dimension(), self.population_size as usize, rng)
    }
}

impl<P> Component<P> for RandomPermutation
where
    P: VectorProblem<Element = usize>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        initialization(self, problem, state)
    }
}

/// Generates new random binary strings with a 1 or `true` having a probability of `p`.
#[derive(Clone, Serialize, Deserialize)]
pub struct RandomBitstring {
    /// Size of the population to be generated.
    pub population_size: u32,
    /// Probability of generating a 1 or `true`.
    pub p: f64,
}

impl RandomBitstring {
    pub fn from_params(population_size: u32, p: f64) -> Self {
        Self { population_size, p }
    }

    pub fn new<P>(population_size: u32, p: f64) -> Box<dyn Component<P>>
    where
        P: Problem + VectorProblem<Element = bool>,
    {
        Box::new(Self::from_params(population_size, p))
    }

    pub fn new_uniform<P>(population_size: u32) -> Box<dyn Component<P>>
    where
        P: Problem + VectorProblem<Element = bool>,
    {
        Self::new::<P>(population_size, 0.5)
    }
}

impl<P> Initialization<P> for RandomBitstring
where
    P: VectorProblem<Element = bool>,
{
    fn initialize(&self, problem: &P, rng: &mut Random) -> Vec<P::Encoding> {
        f::random_bitstring(
            problem.dimension(),
            self.p,
            self.population_size as usize,
            rng,
        )
    }
}

impl<P> Component<P> for RandomBitstring
where
    P: VectorProblem<Element = bool>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        initialization(self, problem, state)
    }
}
