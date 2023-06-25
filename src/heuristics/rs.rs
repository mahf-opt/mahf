//! Random Search

use crate::{
    component::ExecResult,
    components::*,
    conditions::Condition,
    configuration::Configuration,
    logging::Logger,
    problems::{Evaluator, LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
};

/// An example single-objective Random Search operating on a real search space.
/// Uses the [rs] component internally.
pub fn real_rs<P, O>(condition: Box<dyn Condition<P>>) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    O: Evaluator<Problem = P>,
{
    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(1))
        .evaluate_with::<O>()
        .update_best_individual()
        .do_(rs::<P, O>(
            Parameters {
                randomizer: <mutation::PartialRandomSpread>::new_full(),
            },
            condition,
        ))
        .build())
}

/// An example single-objective Random Search operating on a permutation search space.
/// Uses the [rs] component internally.
pub fn permutation_rs<P, O>(condition: Box<dyn Condition<P>>) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + VectorProblem<Element = usize>,
    O: Evaluator<Problem = P>,
{
    Ok(Configuration::builder()
        .do_(initialization::RandomPermutation::new(1))
        .evaluate_with::<O>()
        .update_best_individual()
        .do_(rs::<P, O>(
            Parameters {
                randomizer: <mutation::ScrambleMutation>::new_full(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of an Random Search.
pub struct Parameters<P> {
    pub randomizer: Box<dyn Component<P>>,
}

/// A generic single-objective Random Search template.
pub fn rs<P, O>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    O: Evaluator<Problem = P>,
{
    let Parameters { randomizer } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(selection::All::new())
                .do_(randomizer)
                .evaluate_with::<O>()
                .update_best_individual()
                .do_(replacement::MuPlusLambda::new(1))
                .do_(Logger::new())
        })
        .build_component()
}
