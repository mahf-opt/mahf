//! Invasive Weed Optimization (IWO).
//!
//! # References
//!
//! \[1\] Ali Reza Mehrabian and Caro Lucas. 2006.
//! A novel numerical optimization algorithm inspired from weed colonization.
//! Ecological Informatics 1, 4 (December 2006), 355–366.
//! DOI:<https://doi.org/10/dc4njs>

use eyre::ensure;

use crate::{
    component::ExecResult,
    components::{boundary, initialization, mapping, mutation, replacement, selection, Block},
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    lens::ValueOf,
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
    state::common,
    Component,
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
///
/// Uses the [iwo] component internally.
///
/// # Requirements
/// - initial_population_size <= max_population_size
/// - min_number_of_seeds <= max_number_of_seeds
/// - final_deviation <= initial_deviation
pub fn real_iwo<P>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
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

    ensure!(initial_population_size <= max_population_size, "it is not possible to select more individuals with MuPlusLambda selection than are present");
    ensure!(
        !(initial_deviation..final_deviation).is_empty(),
        "the std_dev range must not be empty for this operator"
    );

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(initial_population_size))
        .evaluate()
        .update_best_individual()
        .do_(iwo::<P, Global>(
            Parameters {
                max_population_size,
                min_number_of_seeds,
                max_number_of_seeds,
                mutation: Block::new([
                    mutation::NormalMutation::new(initial_deviation, 1.),
                    mapping::Polynomial::new(
                        initial_deviation,
                        final_deviation,
                        modulation_index as f64,
                        ValueOf::<common::Progress<ValueOf<common::Iterations>>>::new(),
                        ValueOf::<mutation::MutationStrength<mutation::NormalMutation>>::new(),
                    ),
                ]),
                constraints: boundary::Saturation::new(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`iwo`].
pub struct Parameters<P> {
    pub max_population_size: u32,
    pub min_number_of_seeds: u32,
    pub max_number_of_seeds: u32,
    pub mutation: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Invasive Weed Optimization (IWO) template.
pub fn iwo<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
{
    let Parameters {
        max_population_size,
        min_number_of_seeds,
        max_number_of_seeds,
        mutation,
        constraints,
    } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(selection::iwo::DeterministicFitnessProportional::new(
                    min_number_of_seeds,
                    max_number_of_seeds,
                ))
                .do_(mutation)
                .do_(constraints)
                .evaluate_with::<I>()
                .update_best_individual()
                .do_(replacement::MuPlusLambda::new(max_population_size))
                .do_(Logger::new())
        })
        .build_component()
}
