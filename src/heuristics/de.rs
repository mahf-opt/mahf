//! Differential Evolution

use crate::{
    components::*,
    conditions::Condition,
    framework::Configuration,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
};

/// Parameters for [real_de].
pub struct RealProblemParameters {
    pub population_size: u32,
    pub y: usize,
    pub f: f64,
    pub pc: f64,
}

/// An example single-objective Differential Evolution algorithm operating on a real search space.
/// Uses the [de] component internally.
///
/// # References
/// [doi.org/10.1016/j.ins.2021.09.058](https://doi.org/10.1016/j.ins.2021.09.058)
pub fn real_de<P>(
    params: RealProblemParameters,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    let RealProblemParameters {
        population_size,
        y,
        f,
        pc,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(population_size))
        .evaluate()
        .update_best_individual()
        .do_(de(
            Parameters {
                selection: selection::DEBest::new(y),
                mutation: generation::mutation::DEMutation::new(y, f),
                crossover: generation::recombination::DEBinomialCrossover::new(pc),
                constraints: constraints::Saturation::new(),
                replacement: replacement::IndividualPlus::new(),
            },
            termination,
            logger,
        ))
        .build()
}

/// Basic building blocks of Differential Evolution.
pub struct Parameters<P> {
    pub selection: Box<dyn Component<P>>,
    pub mutation: Box<dyn Component<P>>,
    pub crossover: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub replacement: Box<dyn Component<P>>,
}

/// A generic single-objective Differential Evolution template.
pub fn de<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>> {
    let Parameters {
        selection,
        mutation,
        crossover,
        constraints,
        replacement,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(selection)
                .do_(mutation)
                .do_(crossover)
                .do_(constraints)
                .evaluate()
                .update_best_individual()
                .do_(replacement)
                .do_(logger)
        })
        .build_component()
}
