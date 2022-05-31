//! Invasive Weed Optimization

use crate::{
    framework::{
        components::{self, Component, Condition},
        Configuration,
    },
    operators::*,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

#[derive(Clone, Copy, Debug)]
pub struct Parameters {
    pub initial_population_size: u32,
    pub max_population_size: u32,
    pub min_number_of_seeds: u32,
    pub max_number_of_seeds: u32,
    pub initial_deviation: f64,
    pub final_deviation: f64,
    pub modulation_index: u32,
}

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
    params: Parameters,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem + 'static,
{
    assert!(params.initial_population_size <= params.max_population_size);
    assert!(params.min_number_of_seeds <= params.max_number_of_seeds);
    assert!(params.final_deviation <= params.initial_deviation);

    components::Block::new(vec![
        initialization::RandomSpread::new(params.initial_population_size),
        components::SimpleEvaluator::new(),
        components::Loop::new(
            termination,
            vec![
                selection::DeterministicFitnessProportional::new(
                    params.min_number_of_seeds,
                    params.max_number_of_seeds,
                ),
                generation::IWOAdaptiveDeviationDelta::new(
                    params.initial_deviation,
                    params.final_deviation,
                    params.modulation_index,
                ),
                components::SimpleEvaluator::new(),
                replacement::MuPlusLambda::new(params.max_population_size),
                logger,
            ],
        ),
    ])
}
