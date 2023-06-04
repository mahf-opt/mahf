//! Evolution Strategy

use crate::{
    component::ExecResult,
    components::*,
    conditions::Condition,
    configuration::Configuration,
    logging::Logger,
    problems::{Evaluator, LimitedVectorProblem, SingleObjectiveProblem},
};

/// Parameters for [real_mu_plus_lambda_es].
pub struct RealProblemParameters {
    pub population_size: u32,
    pub lambda: u32,
    pub deviation: f64,
}

/// An example single-objective (μ+λ)-Evolution-Strategy operating on a real search space.
/// Uses the [es] component internally.
///
/// # References
/// [doi.org/10.1023/A:1015059928466](https://doi.org/10.1023/A:1015059928466)
pub fn real_mu_plus_lambda_es<P, O>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    O: Evaluator<Problem = P>,
{
    let RealProblemParameters {
        population_size,
        lambda,
        deviation,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(population_size))
        .evaluate::<O>()
        .update_best_individual()
        .do_(es::<P, O>(
            Parameters {
                selection: selection::FullyRandom::new(lambda),
                mutation: <mutation::NormalMutation>::new_dev(deviation),
                constraints: boundary::Saturation::new(),
                archive: None,
                replacement: replacement::MuPlusLambda::new(population_size),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of an Evolution Strategy.
pub struct Parameters<P> {
    pub selection: Box<dyn Component<P>>,
    pub mutation: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub archive: Option<Box<dyn Component<P>>>,
    pub replacement: Box<dyn Component<P>>,
}

/// A generic single-objective Evolution Strategy template.
pub fn es<P, O>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    O: Evaluator<Problem = P>,
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
                .evaluate::<O>()
                .update_best_individual()
                .do_if_some_(archive)
                .do_(replacement)
                .do_(Logger::new())
        })
        .build_component()
}
