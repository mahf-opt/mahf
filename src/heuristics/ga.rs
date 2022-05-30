//! Genetic Algorithm

use crate::{
    framework::legacy::Configuration,
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Genetic Algorithm
///
/// # References
/// [link.springer.com/10.1007/978-3-319-07153-4_28-1](http://link.springer.com/10.1007/978-3-319-07153-4_28-1)
pub fn ga<P>(
    population_size: u32,
    deviation: f64,
    _p_mutation: f64,
    pc: f64,
    max_iterations: u32,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration {
        initialization: initialization::RandomSpread::new_init(population_size),
        selection: selection::FullyRandom::new(population_size),
        generation: vec![
            generation::recombination::UniformCrossover::new(pc),
            generation::mutation::FixedDeviationDelta::new(deviation),
        ],
        replacement: replacement::Generational::new(population_size),
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
}
