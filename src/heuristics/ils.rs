//! Iterated Local Search

use crate::{
    components::*,
    conditions::Condition,
    framework::Configuration,
    heuristics::ls,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
    tracking::Logger,
};

/// Parameters for [real_iterated_local_search].
pub struct RealProblemParameters<P> {
    pub local_search_params: ls::RealProblemParameters,
    pub local_search_termination: Box<dyn Condition<P>>,
}

/// An example single-objective Iterated Local Search operating on a real search space.
/// Uses the [iterated_local_search] component internally.
pub fn real_iterated_local_search<P>(
    params: RealProblemParameters<P>,
    termination: Box<dyn Condition<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    let RealProblemParameters {
        local_search_params,
        local_search_termination,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(1))
        .evaluate()
        .update_best_individual()
        .do_(iterated_local_search(
            Parameters {
                perturbation: generation::RandomSpread::new_gen(),
                local_search: ls::real_local_search(local_search_params, local_search_termination)
                    .into_inner(),
            },
            termination,
        ))
        .build()
}

/// Parameters for [iterated_local_permutation_search].
pub struct PermutationProblemParameters<P> {
    pub local_search_params: ls::PermutationProblemParameters,
    pub local_search_termination: Box<dyn Condition<P>>,
}

/// An example single-objective Iterated Local Search operating on a permutation search space.
/// Uses the [iterated_local_search] component internally.
pub fn permutation_iterated_local_search<P>(
    params: PermutationProblemParameters<P>,
    termination: Box<dyn Condition<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    let PermutationProblemParameters {
        local_search_params,
        local_search_termination,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomPermutation::new_init(1))
        .evaluate()
        .update_best_individual()
        .do_(iterated_local_search(
            Parameters {
                perturbation: generation::RandomPermutation::new_gen(),
                local_search: ls::permutation_local_search(
                    local_search_params,
                    local_search_termination,
                )
                .into_inner(),
            },
            termination,
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
) -> Box<dyn Component<P>> {
    let Parameters {
        perturbation,
        local_search,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(perturbation)
                .evaluate()
                .do_(selection::All::new())
                .scope_(|builder| builder.do_(local_search))
                .update_best_individual()
                .do_(replacement::MuPlusLambda::new(1))
                .do_(Logger::new())
        })
        .build_component()
}
