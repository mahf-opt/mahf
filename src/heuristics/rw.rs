//! Random Walk

use crate::{
    framework::{components::Generation, Configuration},
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Random Walk
///
/// # Arguments
///
/// * mutation: The mutation method used to move in the search space.
pub fn random_walk<P>(max_iterations: u32, mutation: Box<dyn Generation<P>>) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration {
        initialization: initialization::RandomSpread::new(1),
        selection: selection::All::new(),
        generation: vec![mutation],
        replacement: replacement::Generational::new(1),
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
}

/// Random Permutation Walk
///
/// # Arguments
///
/// * mutation: The mutation method used to move in the search space.
pub fn random_permutation_walk<P>(
    max_iterations: u32,
    mutation: Box<dyn Generation<P>>,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    Configuration {
        initialization: initialization::RandomPermutation::new(1),
        selection: selection::All::new(),
        generation: vec![mutation],
        replacement: replacement::Generational::new(1),
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
}
