//! Initialization methods

use crate::{
    heuristic::components::*,
    problem::{LimitedVectorProblem, Problem},
};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Uniformly distributes initial solutions in the search space.
#[derive(Serialize, Deserialize)]
pub struct RandomSpread {
    /// Size of the initial population.
    pub initial_population_size: u32,
}
impl<P> Initialization<P> for RandomSpread
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn initialize(&self, problem: &P, population: &mut Vec<Vec<f64>>) {
        let rng = &mut rand::thread_rng();
        for _ in 0..self.initial_population_size {
            let solution = (0..problem.dimension())
                .map(|d| rng.gen_range(problem.range(d)))
                .collect::<Vec<f64>>();
            population.push(solution);
        }
    }
}
