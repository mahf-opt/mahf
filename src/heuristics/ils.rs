//! Iterated Local Search (ILS).

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::*,
    conditions::Condition,
    configuration::Configuration,
    heuristics::ls,
    identifier::{Global, Identifier},
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
};

/// Parameters for [real_ils].
pub struct RealProblemParameters<P> {
    pub ls_params: ls::RealProblemParameters,
    pub ls_condition: Box<dyn Condition<P>>,
}

/// An example single-objective Iterated Local Search operating on a real search space.
/// Uses the [ils] component internally.
pub fn real_ils<P>(
    params: RealProblemParameters<P>,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    let RealProblemParameters {
        ls_params,
        ls_condition,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(1))
        .evaluate()
        .update_best_individual()
        .do_(ils::<P, Global>(
            Parameters {
                perturbation: <mutation::PartialRandomSpread>::new_full(),
                ls: ls::real_ls::<P>(ls_params, ls_condition)
                    .wrap_err("failed to construct local search")?
                    .into_inner(),
            },
            condition,
        ))
        .build())
}

/// Parameters for [`permutation_ils`].
pub struct PermutationProblemParameters<P> {
    pub ls_params: ls::PermutationProblemParameters,
    pub ls_condition: Box<dyn Condition<P>>,
}

/// An example single-objective Iterated Local Search operating on a permutation search space.
/// Uses the [ils] component internally.
pub fn permutation_ils<P>(
    params: PermutationProblemParameters<P>,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + VectorProblem<Element = usize>,
{
    let PermutationProblemParameters {
        ls_params,
        ls_condition,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomPermutation::new(1))
        .evaluate()
        .update_best_individual()
        .do_(ils::<P, Global>(
            Parameters {
                perturbation: <mutation::ScrambleMutation>::new_full(),
                ls: ls::permutation_ls::<P>(ls_params, ls_condition)
                    .wrap_err("failed to construct local search")?
                    .into_inner(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of an Iterated Local Search.
pub struct Parameters<P> {
    pub perturbation: Box<dyn Component<P>>,
    pub ls: Box<dyn Component<P>>,
}

/// A generic single-objective Iterated Local Search template.
pub fn ils<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
{
    let Parameters { perturbation, ls } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(perturbation)
                .evaluate_with::<I>()
                .do_(selection::All::new())
                .scope_(|builder| builder.do_(ls))
                .update_best_individual()
                .do_(replacement::MuPlusLambda::new(1))
                .do_(Logger::new())
        })
        .build_component()
}
