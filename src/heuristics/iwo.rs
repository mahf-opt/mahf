//! Invasive Weed Optimization

use crate::{
    components::*,
    framework::{components::Component, conditions::Condition, Configuration},
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
    tracking::Logger,
};

#[derive(Clone, Debug)]
pub struct RealProblemParameters {
    pub initial_population_size: u32,
    pub max_population_size: u32,
    pub min_number_of_seeds: u32,
    pub max_number_of_seeds: u32,
    pub initial_deviation: f64,
    pub final_deviation: f64,
    pub modulation_index: u32,
}

/// An example single-objective Invasive Weed Optimization operating on a real search space.
/// Uses the [iwo] component internally.
///
/// # Requirements
/// - initial_population_size <= max_population_size
/// - min_number_of_seeds <= max_number_of_seeds
/// - final_deviation <= initial_deviation
///
/// # References
/// [doi.org/10.1016/j.ecoinf.2006.07.003](https://doi.org/10.1016/j.ecoinf.2006.07.003)
pub fn real_iwo<P>(
    params: RealProblemParameters,
    termination: Box<dyn Condition<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    let RealProblemParameters {
        initial_population_size,
        max_population_size,
        min_number_of_seeds,
        max_number_of_seeds,
        initial_deviation,
        final_deviation,
        modulation_index,
    } = params;

    assert!(initial_population_size <= max_population_size);

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(
            params.initial_population_size,
        ))
        .evaluate()
        .update_best_individual()
        .do_(iwo(
            Parameters {
                max_population_size,
                min_number_of_seeds,
                max_number_of_seeds,
                mutation: generation::mutation::IWOAdaptiveDeviationDelta::new(
                    initial_deviation,
                    final_deviation,
                    modulation_index,
                ),
                constraints: constraints::Saturation::new(),
            },
            termination,
        ))
        .build()
}

/// Basic building blocks of Invasive Weed Optimization.
pub struct Parameters<P> {
    pub max_population_size: u32,
    pub min_number_of_seeds: u32,
    pub max_number_of_seeds: u32,
    pub mutation: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Invasive Weed Optimization template.
pub fn iwo<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
) -> Box<dyn Component<P>> {
    let Parameters {
        max_population_size,
        min_number_of_seeds,
        max_number_of_seeds,
        mutation,
        constraints,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(selection::DeterministicFitnessProportional::new(
                    min_number_of_seeds,
                    max_number_of_seeds,
                ))
                .do_(mutation)
                .do_(constraints)
                .evaluate()
                .update_best_individual()
                .do_(replacement::MuPlusLambda::new(max_population_size))
                .do_(Logger::new())
        })
        .build_component()
}
