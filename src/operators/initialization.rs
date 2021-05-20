//! Initialization methods

use crate::{
    heuristic::components::*,
    problem::{LimitedVectorProblem, Problem},
    random::Random,
};
use rand::distributions::uniform::SampleUniform;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Uniformly distributes initial solutions in the search space.
#[derive(Serialize, Deserialize)]
pub struct RandomSpread {
    /// Size of the initial population.
    pub initial_population_size: u32,
}
impl<P, D> Initialization<P> for RandomSpread
where
    D: SampleUniform + PartialOrd,
    P: Problem<Encoding = Vec<D>> + LimitedVectorProblem<T = D>,
{
    fn initialize(&self, problem: &P, rng: &mut Random, population: &mut Vec<Vec<D>>) {
        for _ in 0..self.initial_population_size {
            let solution = (0..problem.dimension())
                .map(|d| rng.gen_range(problem.range(d)))
                .collect::<Vec<D>>();
            population.push(solution);
        }
    }
}
