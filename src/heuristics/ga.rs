//! Genetic Algorithm

use crate::{
    components::*,
    conditions::*,
    framework::Configuration,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
};

/// Parameters for [binary_ga].
#[derive(Clone, Copy, Debug)]
pub struct BinaryProblemParameters {
    pub population_size: u32,
    pub tournament_size: u32,
    pub rm: f64,
    pub pc: f64,
    pub pm: f64,
}

/// An example single-objective Genetic Algorithm operating on a binary search space.
/// Uses the [ga] component internally.
pub fn binary_ga<P>(
    params: BinaryProblemParameters,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<bool>>
        + VectorProblem<Element = bool>
        + LimitedVectorProblem,
{
    let BinaryProblemParameters {
        population_size,
        tournament_size,
        rm,
        pc,
        pm,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomBitstring::new_uniform_init(
            population_size,
        ))
        .evaluate()
        .update_best_individual()
        .do_(ga(
            Parameters {
                selection: selection::Tournament::new(population_size, tournament_size),
                crossover: generation::recombination::UniformCrossover::new_both(pc),
                pm,
                mutation: generation::mutation::BitflipMutation::new(rm),
                constraints: misc::Noop::new(),
                archive: None,
                replacement: replacement::Generational::new(population_size),
            },
            termination,
            logger,
        ))
        .build()
}

/// Parameters for [real_ga].
#[derive(Clone, Copy, Debug)]
pub struct RealProblemParameters {
    pub population_size: u32,
    pub tournament_size: u32,
    pub pm: f64,
    pub deviation: f64,
    pub pc: f64,
}

/// An example single-objective Genetic Algorithm operating on a real search space.
/// Uses the [ga] component internally.
pub fn real_ga<P>(
    params: RealProblemParameters,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>>
        + VectorProblem<Element = f64>
        + LimitedVectorProblem,
{
    let RealProblemParameters {
        population_size,
        tournament_size,
        pm,
        deviation,
        pc,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(population_size))
        .evaluate()
        .update_best_individual()
        .do_(ga(
            Parameters {
                selection: selection::Tournament::new(population_size, tournament_size),
                crossover: generation::recombination::UniformCrossover::new_both(pc),
                pm,
                mutation: generation::mutation::FixedDeviationDelta::new(deviation),
                constraints: constraints::Saturation::new(),
                archive: None,
                replacement: replacement::Generational::new(population_size),
            },
            termination,
            logger,
        ))
        .build()
}

/// Basic building blocks of a Genetic Algorithm.
pub struct Parameters<P> {
    pub selection: Box<dyn Component<P>>,
    pub crossover: Box<dyn Component<P>>,
    pub pm: f64,
    pub mutation: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub archive: Option<Box<dyn Component<P>>>,
    pub replacement: Box<dyn Component<P>>,
}

/// A generic single-objective Genetic Algorithm template.
///
/// # References
/// [link.springer.com/10.1007/978-3-319-07153-4_28-1](http://link.springer.com/10.1007/978-3-319-07153-4_28-1)
pub fn ga<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>> {
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
                .if_(branching::RandomChance::new(pm), |builder| {
                    builder.do_(mutation)
                })
                .do_(constraints)
                .evaluate()
                .update_best_individual()
                .do_if_some_(archive)
                .do_(replacement)
                .do_(logger)
        })
        .build_component()
}
