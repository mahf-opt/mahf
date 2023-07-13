//! Random Search (RS).

use crate::{
    component::ExecResult,
    components::{initialization, mutation, replacement, selection},
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
    Component,
};

/// An example single-objective random search operating on a real search space.
///
/// Uses the [`rs`] component internally.
pub fn real_rs<P>(condition: Box<dyn Condition<P>>) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(1))
        .evaluate()
        .update_best_individual()
        .do_(rs::<P, Global>(
            Parameters {
                randomizer: mutation::PartialRandomSpread::new_full(),
            },
            condition,
        ))
        .build())
}

/// An example single-objective random search operating on a permutation search space.
///
/// Uses the [`rs`] component internally.
pub fn permutation_rs<P>(condition: Box<dyn Condition<P>>) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + VectorProblem<Element = usize>,
{
    Ok(Configuration::builder()
        .do_(initialization::RandomPermutation::new(1))
        .evaluate()
        .update_best_individual()
        .do_(rs::<P, Global>(
            Parameters {
                randomizer: <mutation::ScrambleMutation>::new_full(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`rs`].
pub struct Parameters<P> {
    pub randomizer: Box<dyn Component<P>>,
}

/// A generic single-objective Random Search (RS) template.
pub fn rs<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
{
    let Parameters { randomizer } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(selection::All::new())
                .do_(randomizer)
                .evaluate_with::<I>()
                .update_best_individual()
                .do_(replacement::MuPlusLambda::new(1))
                .do_(Logger::new())
        })
        .build_component()
}
