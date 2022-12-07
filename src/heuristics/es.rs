//! Evolution Strategy

use crate::{
    components::*,
    framework::{components::Component, conditions::Condition, Configuration},
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
};

/// Parameters for [real_mu_plus_lambda].
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
pub fn real_mu_plus_lambda<P>(
    params: RealProblemParameters,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    let RealProblemParameters {
        population_size,
        lambda,
        deviation,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(population_size))
        .evaluate_sequential()
        .update_best_individual()
        .do_(es(
            Parameters {
                selection: selection::FullyRandom::new(lambda),
                mutation: generation::mutation::FixedDeviationDelta::new(deviation),
                constraints: constraints::Saturation::new(),
                archive: None,
                replacement: replacement::MuPlusLambda::new(population_size),
            },
            termination,
            logger,
        ))
        .build()
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
pub fn es<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>> {
    let Parameters {
        selection,
        mutation,
        constraints,
        archive,
        replacement,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(selection)
                .do_(mutation)
                .do_(constraints)
                .evaluate_sequential()
                .update_best_individual()
                .do_optional_(archive)
                .do_(replacement)
                .do_(logger)
        })
        .build_component()
}
