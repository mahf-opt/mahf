//! Genetic Algorithm

use crate::{
    framework::{components, Configuration},
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Genetic Algorithm
///
/// # References
/// [link.springer.com/10.1007/978-3-319-07153-4_28-1](http://link.springer.com/10.1007/978-3-319-07153-4_28-1)
pub fn ga<P>(population_size: u32, deviation: f64, pc: f64, max_iterations: u32) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem + 'static,
{
    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(population_size))
        .do_(components::SimpleEvaluator::new())
        .while_(
            termination::FixedIterations::new(max_iterations),
            |builder| {
                builder
                    .do_(selection::FullyRandom::new(population_size))
                    .do_(generation::recombination::UniformCrossover::new(pc))
                    .do_(generation::mutation::FixedDeviationDelta::new(deviation))
                    .do_(components::SimpleEvaluator::new())
                    .do_(replacement::Generational::new(population_size))
            },
        )
        .build()
}
