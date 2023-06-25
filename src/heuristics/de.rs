//! Differential Evolution

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::*,
    conditions::Condition,
    configuration::Configuration,
    logging::Logger,
    problems::{Evaluator, LimitedVectorProblem, SingleObjectiveProblem},
};

/// Parameters for [real_de].
pub struct RealProblemParameters {
    pub population_size: u32,
    pub y: u32,
    pub f: f64,
    pub pc: f64,
}

/// An example single-objective Differential Evolution algorithm operating on a real search space.
/// Uses the [de] component internally.
///
/// # References
/// [doi.org/10.1016/j.ins.2021.09.058](https://doi.org/10.1016/j.ins.2021.09.058)
pub fn real_de<P, O>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    O: Evaluator<Problem = P>,
{
    let RealProblemParameters {
        population_size,
        y,
        f,
        pc,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(population_size))
        .evaluate_with::<O>()
        .update_best_individual()
        .do_(de::<P, O>(
            Parameters {
                selection: selection::de::DEBest::new(y)
                    .wrap_err("failed to construct DE selection")?,
                mutation: mutation::de::DEMutation::new(y, f)
                    .wrap_err("failed to construct DE mutation")?,
                crossover: recombination::de::DEBinomialCrossover::new(pc),
                constraints: boundary::Saturation::new(),
                replacement: replacement::KeepBetterAtIndex::new(),
            },
            condition,
        ))
        .build())
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
pub fn de<P, O>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    O: Evaluator<Problem = P>,
{
    let Parameters {
        selection,
        mutation,
        crossover,
        constraints,
        replacement,
    } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(selection)
                .do_(mutation)
                .do_(crossover)
                .do_(constraints)
                .evaluate_with::<O>()
                .update_best_individual()
                .do_(replacement)
                .do_(Logger::new())
        })
        .build_component()
}
