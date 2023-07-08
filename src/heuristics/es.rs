//! Evolution Strategy (ES).
//!
//! # References
//!
//! \[1\] Hans-Georg Beyer and Hans-Paul Schwefel. 2002.
//! Evolution strategies – A comprehensive introduction.
//! Natural Computing 1, 1 (March 2002), 3–52.
//! DOI:<https://doi.org/10/djvqhd>

use crate::{
    component::ExecResult,
    components::{boundary, initialization, mutation, replacement, selection},
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
    Component,
};

/// Parameters for [`real_mu_plus_lambda_es`].
pub struct RealProblemParameters {
    pub population_size: u32,
    pub lambda: u32,
    pub deviation: f64,
}

/// An example single-objective (μ+λ)-ES operating on a real search space.
///
/// Uses the [`es`] component internally.
pub fn real_mu_plus_lambda_es<P, O>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    let RealProblemParameters {
        population_size,
        lambda,
        deviation,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(population_size))
        .evaluate()
        .update_best_individual()
        .do_(es::<P, Global>(
            Parameters {
                selection: selection::FullyRandom::new(lambda),
                mutation: mutation::NormalMutation::new_dev(deviation),
                constraints: boundary::Saturation::new(),
                archive: None,
                replacement: replacement::MuPlusLambda::new(population_size),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`es`].
pub struct Parameters<P> {
    pub selection: Box<dyn Component<P>>,
    pub mutation: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub archive: Option<Box<dyn Component<P>>>,
    pub replacement: Box<dyn Component<P>>,
}

/// A generic single-objective Evolution Strategy (ES) template.
pub fn es<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
{
    let Parameters {
        selection,
        mutation,
        constraints,
        archive,
        replacement,
    } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(selection)
                .do_(mutation)
                .do_(constraints)
                .evaluate_with::<I>()
                .update_best_individual()
                .do_if_some_(archive)
                .do_(replacement)
                .do_(Logger::new())
        })
        .build_component()
}
