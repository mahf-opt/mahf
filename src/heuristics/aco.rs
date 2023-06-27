//! Ant Colony Optimization (ACO).

use eyre::WrapErr;

use crate::{
    component::ExecResult,
    components::*,
    conditions::Condition,
    configuration::Configuration,
    identifier::{Global, Identifier},
    logging::Logger,
    problems::{SingleObjectiveProblem, TravellingSalespersonProblem},
};

/// Parameters for [ant_system].
pub struct ASParameters {
    num_ants: usize,
    alpha: f64,
    beta: f64,
    default_pheromones: f64,
    evaporation: f64,
    decay_coefficient: f64,
}

/// Ant Colony Optimization - Ant System
/// Uses the [aco] component internally.
///
/// # References
/// [doi.org/10.1109/MCI.2006.329691](https://doi.org/10.1109/MCI.2006.329691)
pub fn ant_system<P>(
    params: ASParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: TravellingSalespersonProblem,
{
    let ASParameters {
        num_ants,
        alpha,
        beta,
        default_pheromones,
        evaporation,
        decay_coefficient,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::Empty::new())
        .do_(aco::<P, Global>(
            Parameters {
                generation: generative::AcoGeneration::new(
                    num_ants,
                    alpha,
                    beta,
                    default_pheromones,
                ),
                pheromone_update: generative::AsPheromoneUpdate::new(
                    evaporation,
                    decay_coefficient,
                ),
            },
            condition,
        ))
        .build())
}

/// Parameters for [max_min_ant_system].
pub struct MMASParameters {
    num_ants: usize,
    alpha: f64,
    beta: f64,
    default_pheromones: f64,
    evaporation: f64,
    max_pheromones: f64,
    min_pheromones: f64,
}

/// Ant Colony Optimization - MAX-MIN Ant System
/// Uses the [aco] component internally.
///
/// # References
/// [doi.org/10.1109/MCI.2006.329691](https://doi.org/10.1109/MCI.2006.329691)
pub fn max_min_ant_system<P>(
    params: MMASParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: TravellingSalespersonProblem,
{
    let MMASParameters {
        num_ants,
        alpha,
        beta,
        default_pheromones,
        evaporation,
        max_pheromones,
        min_pheromones,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::Empty::new())
        .evaluate::<Global>()
        .do_(aco::<P, Global>(
            Parameters {
                generation: generative::AcoGeneration::new(
                    num_ants,
                    alpha,
                    beta,
                    default_pheromones,
                ),
                pheromone_update: generative::MinMaxPheromoneUpdate::new(
                    evaporation,
                    max_pheromones,
                    min_pheromones,
                )
                .wrap_err("failed to construct the min max pheromone update")?,
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of Ant Colony Optimization.
pub struct Parameters<P> {
    generation: Box<dyn Component<P>>,
    pheromone_update: Box<dyn Component<P>>,
}

/// A generic single-objective Ant Colony Optimization template.
pub fn aco<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
{
    let Parameters {
        generation,
        pheromone_update,
    } = params;

    Configuration::builder()
        .while_(condition, |builder| {
            builder
                .do_(generation)
                .evaluate::<I>()
                .update_best_individual()
                .do_(pheromone_update)
                .do_(Logger::new())
        })
        .build_component()
}
