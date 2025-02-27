//! Differential Evolution (DE).
//!
//! # References
//!
//! \[1\] Anna V. Kononova, Fabio Caraffini, and Thomas Bäck. 2021.
//! Differential evolution outside the box.
//! Information Sciences 581, (December 2021), 587–604.
//! DOI:<https://doi.org/10/grsff3>
//!
//! \[2\] R. Storn. 1996.
//! On the usage of differential evolution for function optimization.
//! In Proceedings of North American Fuzzy Information Processing, 519–523.
//! DOI:<https://doi.org/10/ds8ctb>

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::{boundary, initialization, mutation, recombination, replacement, selection},
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
    Component,
};

/// Parameters for [`real_de`].
pub struct RealProblemParameters {
    pub population_size: u32,
    pub y: u32,
    pub f: f64,
    pub pc: f64,
}

/// An example single-objective DE operating on a real search space.
///
/// Uses the [`de`] component internally.
pub fn real_de<P>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    let RealProblemParameters {
        population_size,
        y,
        f,
        pc,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(population_size))
        .evaluate()
        .update_best_individual()
        .do_(de::<P, Global>(
            Parameters {
                selection: selection::de::DEBest::new(y)
                    .wrap_err("failed to construct DE selection")?,
                mutation: mutation::de::DEMutation::new(y, f)
                    .wrap_err("failed to construct DE mutation")?,
                crossover: recombination::de::DEBinomialCrossover::new(pc)?,
                constraints: boundary::Saturation::new(),
                replacement: replacement::KeepBetterAtIndex::new(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`de`].
pub struct Parameters<P> {
    pub selection: Box<dyn Component<P>>,
    pub mutation: Box<dyn Component<P>>,
    pub crossover: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub replacement: Box<dyn Component<P>>,
}

/// A generic single-objective Differential Evolution (DE) template.
pub fn de<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
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
                .evaluate_with::<I>()
                .update_best_individual()
                .do_(replacement)
                .do_(Logger::new())
        })
        .build_component()
}
