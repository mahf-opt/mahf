//! Firefly Algorithm (FA).
//!
//! # References
//!
//! \[1\] Xin-She Yang. 2009.
//! Firefly algorithms for Multimodal Optimization.
//! In Watanabe, O.; Zeugmann, T. (eds) Stochastic Algorithms: Foundations and Applications.
//! SAGA 2009. Lecture Notes in Computer Science, vol 5792. Springer, Berlin, Heidelberg.
//! DOI:<https://doi.org/10.1007/978-3-642-04944-6_14>

use crate::{
    component::ExecResult,
    components::{boundary, initialization, mapping, swarm},
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    lens::ValueOf,
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem},
    Component,
};

/// Parameters for [`real_fa`].
pub struct RealProblemParameters {
    pub pop_size: u32,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub delta: f64,
}

/// An example single-objective FA operating on a real search space.
///
/// Uses the [`fa`] component internally.
pub fn real_fa<P>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    let RealProblemParameters {
        pop_size,
        alpha,
        beta,
        gamma,
        delta,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(pop_size))
        .evaluate()
        .update_best_individual()
        .do_(fa::<P, Global>(
            Parameters {
                firefly_update: swarm::fa::FireflyPositionsUpdate::new(alpha, beta, gamma),
                constraints: boundary::Saturation::new(),
                alpha_update: Box::from(mapping::sa::GeometricCooling::new(
                    delta,
                    ValueOf::<swarm::fa::RandomizationParameter>::new(),
                )),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`fa`].
pub struct Parameters<P> {
    pub firefly_update: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub alpha_update: Box<dyn Component<P>>,
}

/// A generic single-objective Firefly Algorithm (FA) template.
pub fn fa<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
{
    let Parameters {
        firefly_update,
        constraints,
        alpha_update,
    } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(firefly_update)
                .do_(constraints)
                .evaluate_with::<I>()
                .update_best_individual()
                .do_(alpha_update)
                .do_(Logger::new())
        })
        .build_component()
}
