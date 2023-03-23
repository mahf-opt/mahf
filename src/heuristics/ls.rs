//! Local Search

use crate::{
    components::*,
    framework::{components::Component, conditions::Condition, Configuration},
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
    tracking::Logger,
};

/// Parameters for [real_local_search].
pub struct RealProblemParameters {
    pub n_neighbors: u32,
    pub deviation: f64,
}

/// An example single-objective Local Search operating on a real search space.
/// Uses the [local_search] component internally.
pub fn real_local_search<P>(
    params: RealProblemParameters,
    termination: Box<dyn Condition<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    let RealProblemParameters {
        n_neighbors,
        deviation,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(1))
        .evaluate()
        .update_best_individual()
        .do_(evaluation::UpdateBestIndividual::new())
        .do_(local_search(
            Parameters {
                n_neighbors,
                neighbors: generation::mutation::FixedDeviationDelta::new(deviation),
                constraints: constraints::Saturation::new(),
            },
            termination,
        ))
        .build()
}

/// Parameters for [permutation_local_search].
pub struct PermutationProblemParameters {
    pub n_neighbors: u32,
    pub n_swap: usize,
}

/// An example single-objective Local Search operating on a permutation search space.
/// Uses the [local_search] component internally.
pub fn permutation_local_search<P>(
    params: PermutationProblemParameters,
    termination: Box<dyn Condition<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    let PermutationProblemParameters {
        n_neighbors,
        n_swap,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomPermutation::new_init(1))
        .evaluate()
        .update_best_individual()
        .do_(local_search(
            Parameters {
                n_neighbors,
                neighbors: generation::mutation::SwapMutation::new(n_swap),
                constraints: misc::Noop::new(),
            },
            termination,
        ))
        .build()
}

/// Basic building blocks of a Local Search.
pub struct Parameters<P> {
    pub n_neighbors: u32,
    pub neighbors: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Local Search template.
pub fn local_search<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
) -> Box<dyn Component<P>> {
    let Parameters {
        n_neighbors,
        neighbors,
        constraints,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(selection::DuplicateSingle::new(n_neighbors))
                .do_(neighbors)
                .do_(constraints)
                .evaluate()
                .update_best_individual()
                .do_(replacement::MuPlusLambda::new(1))
                .do_(Logger::new())
        })
        .build_component()
}
