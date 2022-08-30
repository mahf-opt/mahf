//! Random Search

use crate::{
    framework::{components::Component, conditions::Condition, Configuration},
    operators::*,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
};

/// An example single-objective Random Search operating on a real search space.
/// Uses the [random_search] component internally.
pub fn real_random_search<P>(
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    Configuration::builder()
        .do_(generation::RandomSpread::new_init(1))
        .evaluate_sequential()
        .update_best_individual()
        .do_(random_search(
            Parameters {
                randomizer: generation::RandomSpread::new_gen(),
            },
            termination,
            logger,
        ))
        .build()
}

/// An example single-objective Random Search operating on a permutation search space.
/// Uses the [random_search] component internally.
pub fn permutation_random_search<P>(
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    Configuration::builder()
        .do_(generation::RandomPermutation::new_init(1))
        .evaluate_sequential()
        .update_best_individual()
        .do_(random_search(
            Parameters {
                randomizer: generation::RandomPermutation::new_gen(),
            },
            termination,
            logger,
        ))
        .build()
}

/// Basic building blocks of an Random Search.
pub struct Parameters<P> {
    pub randomizer: Box<dyn Component<P>>,
}

/// A generic single-objective Random Search template.
pub fn random_search<P>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
{
    let Parameters { randomizer } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(selection::All::new())
                .do_(randomizer)
                .evaluate_sequential()
                .update_best_individual()
                .do_(replacement::MuPlusLambda::new(1))
                .do_(logger)
        })
        .build_component()
}
