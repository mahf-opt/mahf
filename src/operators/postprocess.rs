//! Postprocess variants
//!

use crate::operators::custom_state::{DiversityState, PopulationState, PsoState};
use crate::problems::VectorProblem;
use crate::{
    framework::{components::Postprocess, Individual, State},
    problems::{LimitedVectorProblem, Problem},
    random::Random,
};
use rand::Rng;

#[derive(Debug, serde::Serialize)]
pub struct None;
impl<P> Postprocess<P> for None
where
    P: Problem,
{
    fn postprocess(
        &self,
        _state: &mut State,
        _problem: &P,
        _rng: &mut Random,
        _population: &[Individual],
    ) {
    }
}

// Post-Initialisation Strategies //

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
            .map(Individual::clone)
            .collect::<Vec<Individual>>();

        let global_best = bests
            .iter()
            .min_by_key(|i| Individual::fitness(i))
            .map(Individual::clone)
            .unwrap();

        state.custom.insert(PsoState {
            velocities,
            bests,
            global_best,
        });
    }
}

// Post-Replacement Strategies //

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
                pso_state.bests[i] = individual.clone();

                if pso_state.global_best.fitness() > individual.fitness() {
                    pso_state.global_best = individual.clone();
                }
            }
        }
    }
}

// General post-processes //

/// Postprocess procedure for tracking population diversity
///
/// Currently only for VectorProblem
#[derive(Debug, serde::Serialize)]
pub struct FloatVectorDiversity;

impl<P> Postprocess<P> for FloatVectorDiversity
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>,
{
    fn postprocess(
        &self,
        state: &mut State,
        problem: &P,
        _rng: &mut Random,
        population: &[Individual],
    ) {
        if !state.custom.has::<DiversityState>() {
            state.custom.insert(DiversityState { diversity: 0.0 });
        }

        let diversity_state = state.custom.get_mut::<DiversityState>();

        if population.is_empty() {
            diversity_state.diversity = 0.0;
            return;
        }

        let m = population.len() as f64;
        let d = problem.dimension();
        let iter_solutions = || population.iter().map(|i| i.solution::<Vec<f64>>());

        //TODO: implement different diversity metrics
        diversity_state.diversity = (0..d)
            .into_iter()
            .map(|j| {
                let xj = iter_solutions().map(|s| s[j]).sum::<f64>() / m;
                iter_solutions().map(|s| (s[j] - xj).abs()).sum::<f64>() / m
            })
            .sum::<f64>()
            / (d as f64);
    }
}

/// Postprocess procedure for tracking all individuals' solutions
///
/// Currently only for VectorProblem
//TODO Independent of problem type
#[derive(Debug, serde::Serialize)]
pub struct FloatPopulation;

impl<P> Postprocess<P> for FloatPopulation
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>,
{
    fn postprocess(
        &self,
        state: &mut State,
        _problem: &P,
        _rng: &mut Random,
        population: &[Individual],
    ) {
        if !state.custom.has::<PopulationState>() {
            state.custom.insert(PopulationState {
                current_pop: vec![],
            });
        }
        let population_state = state.custom.get_mut::<PopulationState>();

        population_state.current_pop = population
            .iter()
            .map(|i| i.solution::<Vec<f64>>())
            .cloned()
            .collect();
    }
}
