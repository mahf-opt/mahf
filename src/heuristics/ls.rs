//! Local Search

use rand::distributions::uniform::SampleUniform;
use serde::Serialize;

use crate::framework::components::{Component, Generation};
use crate::framework::State;
use crate::random::Random;
use crate::{
    framework::Configuration,
    operators::*,
    problems,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Local Search
pub fn local_search<P>(
    max_iterations: u32,
    n_neighbors: u32,
    neighbors: impl Generation<P>,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration::new(
        initialization::RandomSpread {
            initial_population_size: 1,
        },
        selection::CopySingle {
            offspring: n_neighbors,
        },
        neighbors,
        replacement::Fittest {
            max_population_size: 1,
        },
        termination::FixedIterations { max_iterations },
    )
}

/// Local Permutation Search
pub fn local_permutation_search<P>(
    max_iterations: u32,
    n_neighbors: u32,
    neighbors: impl Generation<P>,
) -> Configuration<P>
    where
        P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    Configuration::new(
        initialization::RandomPermutation {
            initial_population_size: 1,
        },
        selection::CopySingle {
            offspring: n_neighbors,
        },
        neighbors,
        replacement::Fittest {
            max_population_size: 1,
        },
        termination::FixedIterations { max_iterations },
    )
}