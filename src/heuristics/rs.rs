//! Random Search

use crate::{
    framework::legacy::Configuration,
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Random Search
pub fn random_search<P>(max_iterations: u32) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration {
        generation: vec![generation::RandomSpread::new_gen()],
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
}

pub fn random_permutation_search<P>(max_iterations: u32) -> Configuration<P>
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    Configuration {
        generation: vec![generation::RandomPermutation::new_gen()],
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
}
