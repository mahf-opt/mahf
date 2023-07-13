//! Genetic Algorithm (GA).
//!
//! # References
//!
//! \[1\] John H. Holland. 1992.
//! Genetic Algorithms.
//! Scientific American 267, 1 (1992), 66–73.
//! DOI:<https://doi.org/10/bmbqnb>
//!
//! \[2\] Kumara Sastry, David E. Goldberg, and Graham Kendall. 2014.
//! Genetic Algorithms.
//! In Search Methodologies: Introductory Tutorials in Optimization and Decision Support Techniques,
//! Edmund K. Burke and Graham Kendall (eds.). Springer US, Boston, MA, 93–117.
//! DOI:<https://doi.org/10.1007/978-1-4614-6940-7_4>

use crate::{
    component::ExecResult,
    components::{
        boundary, initialization, mutation, recombination, replacement, selection, utils,
    },
    conditions,
    configuration::Configuration,
    identifier::{Global, Identifier},
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
    Component, Condition,
};

/// Parameters for [`binary_ga`].
#[derive(Clone, Copy, Debug)]
pub struct BinaryProblemParameters {
    pub population_size: u32,
    pub tournament_size: u32,
    pub rm: f64,
    pub pc: f64,
    pub pm: f64,
}

/// An example single-objective GA operating on a binary search space.
///
/// Uses the [`ga`] component internally.
pub fn binary_ga<P>(
    params: BinaryProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + VectorProblem<Element = bool>,
{
    let BinaryProblemParameters {
        population_size,
        tournament_size,
        rm,
        pc,
        pm,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomBitstring::new_uniform(
            population_size,
        ))
        .evaluate()
        .update_best_individual()
        .do_(ga::<P, Global>(
            Parameters {
                selection: selection::Tournament::new(population_size, tournament_size),
                crossover: recombination::UniformCrossover::new_insert_both(pc),
                pm,
                mutation: mutation::BitFlipMutation::new(rm),
                constraints: utils::Noop::new(),
                archive: None,
                replacement: replacement::Generational::new(population_size),
            },
            condition,
        ))
        .build())
}

/// Parameters for [`real_ga`].
#[derive(Clone, Copy, Debug)]
pub struct RealProblemParameters {
    pub population_size: u32,
    pub tournament_size: u32,
    pub pm: f64,
    pub deviation: f64,
    pub pc: f64,
}

/// An example single-objective GA operating on a real search space.
///
/// Uses the [`ga`] component internally.
pub fn real_ga<P>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    let RealProblemParameters {
        population_size,
        tournament_size,
        pm,
        deviation,
        pc,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(population_size))
        .evaluate()
        .update_best_individual()
        .do_(ga::<P, Global>(
            Parameters {
                selection: selection::Tournament::new(population_size, tournament_size),
                crossover: recombination::UniformCrossover::new_insert_both(pc),
                pm,
                mutation: mutation::NormalMutation::new_dev(deviation),
                constraints: boundary::Saturation::new(),
                archive: None,
                replacement: replacement::Generational::new(population_size),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`ga`].
pub struct Parameters<P> {
    pub selection: Box<dyn Component<P>>,
    pub crossover: Box<dyn Component<P>>,
    pub pm: f64,
    pub mutation: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub archive: Option<Box<dyn Component<P>>>,
    pub replacement: Box<dyn Component<P>>,
}

/// A generic single-objective Genetic Algorithm (GA) template.
pub fn ga<P, I>(params: Parameters<P>, termination: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
{
    let Parameters {
        selection,
        crossover,
        pm,
        mutation,
        constraints,
        archive,
        replacement,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(selection)
                .do_(crossover)
                .if_(conditions::RandomChance::new(pm), |builder| {
                    builder.do_(mutation)
                })
                .do_(constraints)
                .evaluate_with::<I>()
                .update_best_individual()
                .do_if_some_(archive)
                .do_(replacement)
                .do_(Logger::new())
        })
        .build_component()
}
