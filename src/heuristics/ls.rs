//! Local Search (LS).

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::*,
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
};

/// Parameters for [real_ls].
pub struct RealProblemParameters {
    pub n_neighbors: u32,
    pub deviation: f64,
}

/// An example single-objective Local Search operating on a real search space.
/// Uses the [ls] component internally.
pub fn real_ls<P>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    let RealProblemParameters {
        n_neighbors,
        deviation,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(1))
        .evaluate()
        .update_best_individual()
        .do_(evaluation::BestIndividualUpdate::new())
        .do_(ls::<P, Global>(
            Parameters {
                num_neighbors: n_neighbors,
                neighbors: mutation::NormalMutation::new_dev(deviation),
                constraints: boundary::Saturation::new(),
            },
            condition,
        ))
        .build())
}

/// Parameters for [permutation_ls].
pub struct PermutationProblemParameters {
    pub num_neighbors: u32,
    pub num_swap: u32,
}

/// An example single-objective Local Search operating on a permutation search space.
/// Uses the [ls] component internally.
pub fn permutation_ls<P>(
    params: PermutationProblemParameters,
    termination: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + VectorProblem<Element = usize>,
{
    let PermutationProblemParameters {
        num_neighbors,
        num_swap,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomPermutation::new(1))
        .evaluate()
        .update_best_individual()
        .do_(ls::<P, Global>(
            Parameters {
                num_neighbors,
                neighbors: mutation::SwapMutation::new(num_swap)
                    .wrap_err("failed to construct the swap mutation")?,
                constraints: utils::Noop::new(),
            },
            termination,
        ))
        .build())
}

/// Basic building blocks of a Local Search.
pub struct Parameters<P> {
    pub num_neighbors: u32,
    pub neighbors: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Local Search template.
pub fn ls<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
{
    let Parameters {
        num_neighbors: n_neighbors,
        neighbors,
        constraints,
    } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(selection::CloneSingle::new(n_neighbors))
                .do_(neighbors)
                .do_(constraints)
                .evaluate_with::<I>()
                .update_best_individual()
                .do_(replacement::MuPlusLambda::new(1))
                .do_(Logger::new())
        })
        .build_component()
}
