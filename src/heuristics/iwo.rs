//! Invasive Weed Optimization

use crate::{
    framework::{components, Configuration},
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Invasive Weed Optimization
///
/// # Requirements
/// - initial_population_size <= max_population_size
/// - min_number_of_seeds <= max_number_of_seeds
/// - final_deviation <= initial_deviation
///
/// # References
/// [doi.org/10.1016/j.ecoinf.2006.07.003](https://doi.org/10.1016/j.ecoinf.2006.07.003)
pub fn iwo<P>(
    initial_population_size: u32,
    max_population_size: u32,
    min_number_of_seeds: u32,
    max_number_of_seeds: u32,
    initial_deviation: f64,
    final_deviation: f64,
    modulation_index: u32,
    max_iterations: u32,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem + 'static,
{
    assert!(initial_population_size <= max_population_size);
    assert!(min_number_of_seeds <= max_number_of_seeds);
    assert!(final_deviation <= initial_deviation);

    components::Block::new(vec![
        initialization::RandomSpread::new(initial_population_size),
        components::Loop::new(
            termination::FixedIterations::new(max_iterations),
            vec![
                selection::DeterministicFitnessProportional::new(
                    min_number_of_seeds,
                    max_number_of_seeds,
                ),
                generation::IWOAdaptiveDeviationDelta::new(
                    initial_deviation,
                    final_deviation,
                    modulation_index,
                ),
                components::SimpleEvaluator::new(),
                replacement::MuPlusLambda::new(max_population_size),
            ],
        ),
    ])
}
