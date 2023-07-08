//! Ant Colony Optimization (ACO).
//!
//! This module currently defines ACO only in its original domain, i.e. for [TSP].
//!
//! [TSP]: TravellingSalespersonProblem
//!
//! # References
//!
//! \[1\] Marco Dorigo, Mauro Birattari, and Thomas Stützle. 2006.
//! Ant colony optimization. IEEE Computational Intelligence Magazine 1, 4 (November 2006), 28–39.
//! DOI:<https://doi.org/10/dq339r>
//!
//! \[2\] Marco Dorigo and Gianni Di Caro. 1999.
//! Ant colony optimization: a new meta-heuristic.
//! In Proceedings of the 1999 Congress on Evolutionary Computation-CEC99 (Cat. No. 99TH8406), 1470-1477 Vol. 2.
//! DOI:<https://doi.org/10/b5h9xz>

use eyre::WrapErr;

use crate::{
    components::{generative, initialization},
    identifier::{Global, Identifier},
    logging::Logger,
    problems::TravellingSalespersonProblem,
    Component, Condition, Configuration, ExecResult, SingleObjectiveProblem,
};

/// Parameters for [`ant_system`].
pub struct ASParameters {
    /// The number of ants in the colony, i.e. how many individuals should be sampled.
    num_ants: usize,
    /// Relative importance of pheromones (usually called τ).
    alpha: f64,
    /// Relative importance of heuristic information (usually called η).
    beta: f64,
    /// Initial pheromone values in the matrix.
    default_pheromones: f64,
    /// Pheromone evaporation rate.
    evaporation: f64,
    /// Constant for scaling the pheromone update.
    decay_coefficient: f64,
}

/// Ant System (AS).
///
/// Uses the [`aco`] component internally.
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

/// Parameters for [`max_min_ant_system`].
pub struct MMASParameters {
    /// The number of ants in the colony, i.e. how many individuals should be sampled.
    num_ants: usize,
    /// Relative importance of pheromones (usually called τ).
    alpha: f64,
    /// Relative importance of heuristic information (usually called η).
    beta: f64,
    /// Initial pheromone values in the matrix.
    default_pheromones: f64,
    /// Pheromone evaporation rate.
    evaporation: f64,
    /// Maximal allowed pheromone value.
    max_pheromones: f64,
    /// Minimal allowed pheromone value.
    min_pheromones: f64,
}

/// MAX-MIN Ant System (MMAS).
///
/// Uses the [`aco`] component internally.
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

/// Basic building blocks of [`aco`].
pub struct Parameters<P> {
    /// Generates a population using the pheromone matrix.
    generation: Box<dyn Component<P>>,
    /// Updates the pheromone matrix using the objective values of the sampled population.
    pheromone_update: Box<dyn Component<P>>,
}

/// A generic single-objective Ant Colony Optimization (ACO) template.
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
                .evaluate_with::<I>()
                .update_best_individual()
                .do_(pheromone_update)
                .do_(Logger::new())
        })
        .build_component()
}
