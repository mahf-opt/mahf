//! Evolutionary Strategy

use crate::{
    framework::Configuration,
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// (μ+λ)-Evolutionary-Strategy
///
/// # References
/// [doi.org/10.1023/A:1015059928466](https://doi.org/10.1023/A:1015059928466)
pub fn mu_plus_lambda<P>(
    population_size: u32,
    lambda: u32,
    deviation: f64,
    max_iterations: u32,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration::new(
        initialization::RandomSpread {
            initial_population_size: population_size,
        },
        selection::FullyRandom { offspring: lambda },
        generation::FixedDeviationDelta { deviation },
        replacement::MuPlusLambda {
            max_population_size: population_size,
        },
        termination::FixedIterations { max_iterations },
    )
}
