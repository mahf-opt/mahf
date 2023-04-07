//! Local Search

use crate::{
    components::*,
    conditions::Condition,
    framework::Configuration,
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
    tracking::Logger,
};

/// Parameters for [real_sa].
pub struct RealProblemParameters {
    pub t_0: f64,
    pub alpha: f64,
    pub deviation: f64,
}

/// An example single-objective Local Search operating on a real search space.
/// Uses the [sa] component internally.
pub fn real_sa<P>(
    params: RealProblemParameters,
    termination: Box<dyn Condition<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    let RealProblemParameters {
        t_0,
        alpha,
        deviation,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(1))
        .evaluate()
        .update_best_individual()
        .do_(evaluation::UpdateBestIndividual::new())
        .do_(sa(
            Parameters {
                t_0,
                generation: generation::mutation::FixedDeviationDelta::new(deviation),
                cooling_schedule: mapping::GeometricCooling::new::<_, replacement::Temperature>(
                    alpha,
                ),
                constraints: constraints::Saturation::new(),
            },
            termination,
        ))
        .build()
}

/// Parameters for [permutation_sa].
pub struct PermutationProblemParameters {
    pub t_0: f64,
    pub alpha: f64,
    pub n_swap: usize,
}

/// An example single-objective Simulated Annealing operating on a permutation search space.
/// Uses the [sa] component internally.
pub fn permutation_sa<P>(
    params: PermutationProblemParameters,
    termination: Box<dyn Condition<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    let PermutationProblemParameters { t_0, alpha, n_swap } = params;

    Configuration::builder()
        .do_(initialization::RandomPermutation::new_init(1))
        .evaluate()
        .update_best_individual()
        .do_(sa(
            Parameters {
                t_0,
                generation: generation::mutation::SwapMutation::new(n_swap),
                cooling_schedule: mapping::GeometricCooling::new::<_, replacement::Temperature>(
                    alpha,
                ),
                constraints: misc::Noop::new(),
            },
            termination,
        ))
        .build()
}

/// Basic building blocks of a Local Search.
pub struct Parameters<P> {
    pub t_0: f64,
    pub generation: Box<dyn Component<P>>,
    pub cooling_schedule: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Simulated Annealing template.
pub fn sa<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
) -> Box<dyn Component<P>> {
    let Parameters {
        t_0,
        generation,
        cooling_schedule,
        constraints,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(selection::All::new())
                .do_(generation)
                .do_(constraints)
                .evaluate()
                .update_best_individual()
                .do_(cooling_schedule)
                .do_(replacement::ExponentialAnnealingAcceptance::new(t_0))
                .do_(Logger::new())
        })
        .build_component()
}
