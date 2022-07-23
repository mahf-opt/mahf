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
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem + 'static,
{
    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(population_size))
        .while_(
            termination::FixedIterations::new(max_iterations),
            |builder| {
                builder
                    .do_(selection::FullyRandom::new(lambda))
                    .do_(generation::mutation::FixedDeviationDelta::new(deviation))
                    .do_(evaluation::SerialEvaluator::new())
                    .do_(replacement::MuPlusLambda::new(population_size))
            },
        )
        .build()
}
