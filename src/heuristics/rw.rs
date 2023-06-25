//! Random Walk

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::*,
    conditions::Condition,
    configuration::Configuration,
    logging::Logger,
    problems::{Evaluator, LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
};

/// Parameters for [real_rw].
pub struct RealProblemParameters {
    pub deviation: f64,
}

/// An example single-objective Random Walk operating on a real search space.
/// Uses the [rw] component internally.
pub fn real_rw<P, O>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    O: Evaluator<Problem = P>,
{
    let RealProblemParameters { deviation } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(1))
        .do_(rw::<P, O>(
            Parameters {
                neighbor: <mutation::NormalMutation>::new_dev(deviation),
                constraints: boundary::Saturation::new(),
            },
            condition,
        ))
        .build())
}

/// Parameters for [permutation_random_walk].
pub struct PermutationProblemParameters {
    pub num_swap: u32,
}

/// An example single-objective Random Walk operating on a permutation search space.
/// Uses the [rw] component internally.
pub fn permutation_random_walk<P, O>(
    params: PermutationProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + VectorProblem<Element = usize>,
    O: Evaluator<Problem = P>,
{
    let PermutationProblemParameters { num_swap } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomPermutation::new(1))
        .do_(rw::<P, O>(
            Parameters {
                neighbor: <mutation::SwapMutation>::new(num_swap)
                    .wrap_err("failed to construct swap mutation")?,
                constraints: misc::Noop::new(),
            },
            condition,
        ))
        .build())
}

pub struct Parameters<P> {
    pub neighbor: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Random Search template.
pub fn rw<P, O>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    O: Evaluator<Problem = P>,
{
    let Parameters {
        neighbor,
        constraints,
    } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(selection::All::new())
                .do_(neighbor)
                .do_(constraints)
                .evaluate_with::<O>()
                .update_best_individual()
                .do_(replacement::Generational::new(1))
                .do_(Logger::new())
        })
        .build_component()
}
