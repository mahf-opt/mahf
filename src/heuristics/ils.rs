//! Iterated Local Search

use crate::{
    framework::{components::Component, conditions::Condition, Configuration},
    heuristics::ls,
    operators::*,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
    tracking,
};

/// Parameters for [real_iterated_local_search].
pub struct RealParameters<P> {
    pub local_search_params: ls::RealParameters,
    pub local_search_termination: Box<dyn Condition<P>>,
}

/// An example single-objective Iterated Local Search operating on a real search space.
/// Uses the [iterated_local_search] component internally.
pub fn real_iterated_local_search<P>(
    params: RealParameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    let RealParameters {
        local_search_params,
        local_search_termination,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(1))
        .evaluate_serial()
        .update_best_individual()
        .do_(iterated_local_search(
            Parameters {
                perturbation: generation::RandomSpread::new_gen(),
                local_search: ls::real_local_search(
                    local_search_params,
                    local_search_termination,
                    tracking::Logger::default(),
                )
                .into_inner(),
            },
            termination,
            logger,
        ))
        .build()
}

/// Parameters for [iterated_local_permutation_search].
pub struct PermutationParameters<P> {
    pub local_search_params: ls::PermutationParameters,
    pub local_search_termination: Box<dyn Condition<P>>,
}

/// An example single-objective Iterated Local Search operating on a permutation search space.
/// Uses the [iterated_local_search] component internally.
pub fn permutation_iterated_local_search<P>(
    params: PermutationParameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    let PermutationParameters {
        local_search_params,
        local_search_termination,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomPermutation::new_init(1))
        .evaluate_serial()
        .update_best_individual()
        .do_(iterated_local_search(
            Parameters {
                perturbation: generation::RandomPermutation::new_gen(),
                local_search: ls::permutation_local_search(
                    local_search_params,
                    local_search_termination,
                    tracking::Logger::default(),
                )
                .into_inner(),
            },
            termination,
            logger,
        ))
        .build()
}

/// Basic building blocks of an Iterated Local Search.
pub struct Parameters<P> {
    pub perturbation: Box<dyn Component<P>>,
    pub local_search: Box<dyn Component<P>>,
}

/// A generic single-objective Iterated Local Search template.
pub fn iterated_local_search<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>> {
    let Parameters {
        perturbation,
        local_search,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(perturbation)
                .evaluate_serial()
                .do_(selection::All::new())
                .scope_(|builder| builder.do_(local_search))
                .update_best_individual()
                .do_(replacement::MuPlusLambda::new(1))
                .do_(logger)
        })
        .build_component()
}
