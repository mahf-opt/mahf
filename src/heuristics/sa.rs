//! Local Search

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::*,
    conditions::Condition,
    configuration::Configuration,
    lens::ValueOf,
    logging::Logger,
    problems::{Evaluator, LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
};

/// Parameters for [real_sa].
pub struct RealProblemParameters {
    pub t_0: f64,
    pub alpha: f64,
    pub deviation: f64,
}

/// An example single-objective Local Search operating on a real search space.
/// Uses the [sa] component internally.
pub fn real_sa<P, O>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    O: Evaluator<Problem = P>,
{
    let RealProblemParameters {
        t_0,
        alpha,
        deviation,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(1))
        .evaluate_with::<O>()
        .update_best_individual()
        .do_(sa::<P, O>(
            Parameters {
                t_0,
                generation: <mutation::NormalMutation>::new_dev(deviation),
                cooling_schedule: mapping::sa::GeometricCooling::new(
                    alpha,
                    ValueOf::<replacement::sa::Temperature>::new(),
                )
                .wrap_err("failed to construct geometric cooling component")?,
                constraints: boundary::Saturation::new(),
            },
            condition,
        ))
        .build())
}

/// Parameters for [permutation_sa].
pub struct PermutationProblemParameters {
    pub t_0: f64,
    pub alpha: f64,
    pub num_swap: u32,
}

/// An example single-objective Simulated Annealing operating on a permutation search space.
/// Uses the [sa] component internally.
pub fn permutation_sa<P, O>(
    params: PermutationProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + VectorProblem<Element = usize>,
    O: Evaluator<Problem = P>,
{
    let PermutationProblemParameters {
        t_0,
        alpha,
        num_swap,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomPermutation::new(1))
        .evaluate_with::<O>()
        .update_best_individual()
        .do_(sa::<P, O>(
            Parameters {
                t_0,
                generation: <mutation::SwapMutation>::new(num_swap)
                    .wrap_err("failed to construct swap mutation")?,
                cooling_schedule: mapping::sa::GeometricCooling::new(
                    alpha,
                    ValueOf::<replacement::sa::Temperature>::new(),
                )
                .wrap_err("failed to construct geometric cooling component")?,
                constraints: misc::Noop::new(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of a Local Search.
pub struct Parameters<P> {
    pub t_0: f64,
    pub generation: Box<dyn Component<P>>,
    pub cooling_schedule: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Simulated Annealing template.
pub fn sa<P, O>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    O: Evaluator<Problem = P>,
{
    let Parameters {
        t_0,
        generation,
        cooling_schedule,
        constraints,
    } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(selection::All::new())
                .do_(generation)
                .do_(constraints)
                .evaluate_with::<O>()
                .update_best_individual()
                .do_(cooling_schedule)
                .do_(replacement::sa::ExponentialAnnealingAcceptance::new(t_0))
                .do_(Logger::new())
        })
        .build_component()
}
