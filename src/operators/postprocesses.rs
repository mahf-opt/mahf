//! Postprocess variants

use crate::operators::custom_states::PsoState;
use crate::{
    framework::{components::Postprocess, Individual, State},
    problems::{LimitedVectorProblem, Problem},
    random::Random,
};
use rand::Rng;

/* Post-Initialisation Strategies */

/// PostInitialisation for PSO.
///
/// Provides initial PsoState by calculating initial velocities, setting the own best and global best.
#[derive(Debug, serde::Serialize)]
pub struct PsoPostInitialization {
    pub v_max: f64,
}

impl<P> Postprocess<P> for PsoPostInitialization
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn postprocess(
        &self,
        state: &mut State,
        problem: &P,
        rng: &mut Random,
        population: &[Individual],
    ) {
        let velocities = population
            .iter()
            .map(|_| {
                (0..problem.dimension())
                    .into_iter()
                    .map(|_| rng.gen_range(-self.v_max..=self.v_max))
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>();

        let bests = population
            .iter()
            .map(Individual::clone::<Vec<f64>>)
            .collect::<Vec<Individual>>();

        let global_best = bests
            .iter()
            .min_by_key(|i| Individual::fitness(i))
            .map(Individual::clone::<Vec<f64>>)
            .unwrap();

        state.custom.insert(PsoState {
            velocities,
            bests,
            global_best,
        });
    }
}

/* Post-Replacement Strategies */

/// PostReplacement for PSO.
///
/// Updates best found solutions of particles and global best in PsoState.
#[derive(Debug, serde::Serialize)]
pub struct PsoPostReplacement;

impl<P> Postprocess<P> for PsoPostReplacement
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn postprocess(
        &self,
        state: &mut State,
        _problem: &P,
        _rng: &mut Random,
        population: &[Individual],
    ) {
        let pso_state = state.custom.get_mut::<PsoState>();

        for (i, individual) in population.iter().enumerate() {
            if pso_state.bests[i].fitness() > individual.fitness() {
                pso_state.bests[i] = individual.clone::<Vec<f64>>();

                if pso_state.global_best.fitness() > individual.fitness() {
                    pso_state.global_best = individual.clone::<Vec<f64>>();
                }
            }
        }
    }
}
