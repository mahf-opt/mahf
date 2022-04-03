//! Local Search

use crate::{
    framework::{components::Component, legacy::Configuration},
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Local Search
pub fn local_search<P>(
    max_iterations: u32,
    n_neighbors: u32,
    neighbors: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration {
        initialization: initialization::RandomSpread::new(1),
        selection: selection::CopySingle::new(n_neighbors),
        generation: vec![neighbors],
        replacement: replacement::MuPlusLambda::new(1),
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
}

/// Local Permutation Search
pub fn local_permutation_search<P>(
    max_iterations: u32,
    n_neighbors: u32,
    neighbors: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    Configuration {
        initialization: initialization::RandomPermutation::new(1),
        selection: selection::CopySingle::new(n_neighbors),
        generation: vec![neighbors],
        replacement: replacement::MuPlusLambda::new(1),
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
}
