//! Random Walk (RW).

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::{boundary, initialization, mutation, replacement, selection, utils},
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
    Component,
};

/// Parameters for [`real_rw`].
pub struct RealProblemParameters {
    pub deviation: f64,
}

/// An example single-objective random walk operating on a real search space.
///
/// Uses the [`rw`] component internally.
pub fn real_rw<P>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    let RealProblemParameters { deviation } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(1))
        .do_(rw::<P, Global>(
            Parameters {
                neighbor: mutation::NormalMutation::new_dev(deviation),
                constraints: boundary::Saturation::new(),
            },
            condition,
        ))
        .build())
}

/// Parameters for [`permutation_random_walk`].
pub struct PermutationProblemParameters {
    pub num_swap: u32,
}

/// An example single-objective random walk operating on a permutation search space.
///
/// Uses the [`rw`] component internally.
pub fn permutation_random_walk<P>(
    params: PermutationProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + VectorProblem<Element = usize>,
{
    let PermutationProblemParameters { num_swap } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomPermutation::new(1))
        .do_(rw::<P, Global>(
            Parameters {
                neighbor: <mutation::SwapMutation>::new(num_swap)
                    .wrap_err("failed to construct swap mutation")?,
                constraints: utils::Noop::new(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`rw`].
pub struct Parameters<P> {
    pub neighbor: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Random Walk (RW) template.
pub fn rw<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
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
                .evaluate_with::<I>()
                .update_best_individual()
                .do_(replacement::Generational::new(1))
                .do_(Logger::new())
        })
        .build_component()
}
