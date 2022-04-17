//! Initialization methods

use rand::seq::SliceRandom;
use rand::{distributions::uniform::SampleUniform, Rng};
use serde::{Deserialize, Serialize};

use crate::problems::VectorProblem;
use crate::{
    framework::{components::*, legacy::components::*, State},
    problems::{LimitedVectorProblem, Problem},
    random::Random,
};

/// Doesn't do anything.
#[derive(Serialize)]
pub struct Noop;
impl Noop {
    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: Problem,
    {
        Box::new(Initializer(Self))
    }
}
impl<P: Problem> Initialization<P> for Noop {
    fn initialize(
        &self,
        _state: &mut State,
        _problem: &P,
        _rng: &mut Random,
        _population: &mut Vec<P::Encoding>,
    ) {
        // Noop
    }
}

/// Uniformly distributes initial solutions in the search space.
#[derive(Serialize, Deserialize)]
pub struct RandomSpread {
    /// Size of the initial population.
    pub initial_population_size: u32,
}
impl RandomSpread {
    pub fn new<P, D>(initial_population_size: u32) -> Box<dyn Component<P>>
    where
        D: SampleUniform + PartialOrd,
        P: Problem<Encoding = Vec<D>> + LimitedVectorProblem<T = D>,
    {
        Box::new(Initializer(Self {
            initial_population_size,
        }))
    }
}
impl<P, D> Initialization<P> for RandomSpread
where
    D: SampleUniform + PartialOrd,
    P: Problem<Encoding = Vec<D>> + LimitedVectorProblem<T = D>,
{
    fn initialize(
        &self,
        _state: &mut State,
        problem: &P,
        rng: &mut Random,
        population: &mut Vec<Vec<D>>,
    ) {
        for _ in 0..self.initial_population_size {
            let solution = (0..problem.dimension())
                .map(|d| rng.gen_range(problem.range(d)))
                .collect::<Vec<D>>();
            population.push(solution);
        }
    }
}

/// Random initialization of combinatorial solutions.
#[derive(Serialize, Deserialize)]
pub struct RandomPermutation {
    /// Size of the initial population.
    pub initial_population_size: u32,
}
impl RandomPermutation {
    pub fn new<P>(initial_population_size: u32) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
    {
        Box::new(Initializer(Self {
            initial_population_size,
        }))
    }
}
impl<P> Initialization<P> for RandomPermutation
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    fn initialize(
        &self,
        _state: &mut State,
        problem: &P,
        rng: &mut Random,
        population: &mut Vec<Vec<usize>>,
    ) {
        for _ in 0..self.initial_population_size {
            let mut solution = (0..problem.dimension()).collect::<Vec<usize>>();
            solution.shuffle(rng);
            population.push(solution);
        }
    }
}
