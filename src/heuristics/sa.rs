//! Simulated Annealing (SA).
//!
//! \[1\] S. Kirkpatrick, C. D. Gelatt, and M. P. Vecchi. 1983.
//! Optimization by Simulated Annealing.
//! Science 220, 4598 (May 1983), 671–680.
//! DOI:<https://doi.org/10/cn7jh2>
//!
//! \[2\] Alexander G. Nikolaev and Sheldon H. Jacobson. 2010.
//! Simulated Annealing.
//! In Handbook of Metaheuristics, Michel Gendreau and Jean-Yves Potvin (eds.).
//! Springer US, Boston, MA, 1–39.
//! DOI:<https://doi.org/10.1007/978-1-4419-1665-5_1>

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::{boundary, initialization, mapping, mutation, replacement, selection, utils},
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    lens::ValueOf,
    logging::Logger,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
    Component,
};

/// Parameters for [`real_sa`].
pub struct RealProblemParameters {
    pub t_0: f64,
    pub alpha: f64,
    pub deviation: f64,
}

/// An example single-objective SA operating on a real search space.
///
/// Uses the [`sa`] component internally.
pub fn real_sa<P>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
{
    let RealProblemParameters {
        t_0,
        alpha,
        deviation,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(1))
        .evaluate()
        .update_best_individual()
        .do_(sa::<P, Global>(
            Parameters {
                t_0,
                generation: mutation::NormalMutation::new_dev(deviation),
                cooling_schedule: mapping::sa::GeometricCooling::new(
                    alpha,
                    ValueOf::<replacement::sa::Temperature>::new(),
                )
                .wrap_err("failed to construct geometric cooling component")?,
                constraints: boundary::Saturation::new(),
            },
            condition,
        ))
        .build())
}

/// Parameters for [`permutation_sa`].
pub struct PermutationProblemParameters {
    pub t_0: f64,
    pub alpha: f64,
    pub num_swap: u32,
}

/// An example single-objective SA operating on a permutation search space.
///
/// Uses the [`sa`] component internally.
pub fn permutation_sa<P>(
    params: PermutationProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + VectorProblem<Element = usize>,
{
    let PermutationProblemParameters {
        t_0,
        alpha,
        num_swap,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomPermutation::new(1))
        .evaluate()
        .update_best_individual()
        .do_(sa::<P, Global>(
            Parameters {
                t_0,
                generation: <mutation::SwapMutation>::new(num_swap)
                    .wrap_err("failed to construct swap mutation")?,
                cooling_schedule: mapping::sa::GeometricCooling::new(
                    alpha,
                    ValueOf::<replacement::sa::Temperature>::new(),
                )
                .wrap_err("failed to construct geometric cooling component")?,
                constraints: utils::Noop::new(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`sa`].
pub struct Parameters<P> {
    pub t_0: f64,
    pub generation: Box<dyn Component<P>>,
    pub cooling_schedule: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Simulated Annealing (SA) template.
pub fn sa<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
{
    let Parameters {
        t_0,
        generation,
        cooling_schedule,
        constraints,
    } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(selection::All::new())
                .do_(generation)
                .do_(constraints)
                .evaluate_with::<I>()
                .update_best_individual()
                .do_(cooling_schedule)
                .do_(replacement::sa::ExponentialAnnealingAcceptance::new(t_0))
                .do_(Logger::new())
        })
        .build_component()
}
