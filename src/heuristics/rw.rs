//! Random Walk

use crate::{
    components::*,
    conditions::Condition,
    framework::Configuration,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
};

/// Parameters for [real_random_walk].
pub struct RealProblemParameters {
    pub deviation: f64,
}

/// An example single-objective Random Walk operating on a real search space.
/// Uses the [random_walk] component internally.
pub fn real_random_walk<P>(
    params: RealProblemParameters,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    let RealProblemParameters { deviation } = params;

    Configuration::builder()
        .do_(generation::RandomSpread::new_init(1))
        .do_(random_walk(
            Parameters {
                neighbor: generation::mutation::FixedDeviationDelta::new(deviation),
                constraints: constraints::Saturation::new(),
            },
            termination,
            logger,
        ))
        .build()
}

/// Parameters for [permutation_random_walk].
pub struct PermutationProblemParameters {
    pub n_swap: usize,
}

/// An example single-objective Random Walk operating on a permutation search space.
/// Uses the [random_walk] component internally.
pub fn permutation_random_walk<P>(
    params: PermutationProblemParameters,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    let PermutationProblemParameters { n_swap } = params;

    Configuration::builder()
        .do_(generation::RandomPermutation::new_init(1))
        .do_(random_walk(
            Parameters {
                neighbor: generation::mutation::SwapMutation::new(n_swap),
                constraints: misc::Noop::new(),
            },
            termination,
            logger,
        ))
        .build()
}

pub struct Parameters<P> {
    pub neighbor: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Random Search template.
pub fn random_walk<P>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
{
    let Parameters {
        neighbor,
        constraints,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(selection::All::new())
                .do_(neighbor)
                .do_(constraints)
                .evaluate()
                .update_best_individual()
                .do_(replacement::Generational::new(1))
                .do_(logger)
        })
        .build_component()
}
