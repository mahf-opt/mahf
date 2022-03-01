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
impl None {
    pub fn new<P>() -> Box<dyn Postprocess<P>>
    where
        P: Problem,
    {
        Box::new(Self)
    }
}
impl<P> Postprocess<P> for None
where
    P: Problem,
{
    fn initialize(
        &self,
        _state: &mut State,
        _problem: &P,
        _rng: &mut Random,
        _population: &[Individual],
    ) {
    }

    fn postprocess(
        &self,
        _state: &mut State,
        _problem: &P,
        _rng: &mut Random,
        _population: &[Individual],
    ) {
    }
}

/// PsoPostprocess for PSO.
///
/// Updates best found solutions of particles and global best in PsoState.
#[derive(Debug, serde::Serialize)]
pub struct PsoPostprocess {
    pub v_max: f64,
}
impl PsoPostprocess {
    pub fn new<P>(v_max: f64) -> Box<dyn Postprocess<P>>
    where
        P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
    {
        Box::new(Self { v_max })
    }
}
impl<P> Postprocess<P> for PsoPostprocess
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn initialize(
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
///
/// Measures can be chosen between dimension-wise (DW), mean of pairwise distance between solutions (PW),
/// average standard deviation of each position (also "true diversity", TD), and distance to average point (DTAP).
/// All measures are normalized with the maximum diversity found so far.
#[derive(Clone, Debug, serde::Serialize)]
pub enum DiversityMeasure {
    DW,
    PW,
    TD,
    DTAP,
}

#[derive(Debug, serde::Serialize)]
pub struct FloatVectorDiversity {
    pub measure: DiversityMeasure,
}

impl<P> Postprocess<P> for FloatVectorDiversity
where
    P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64>,
{
    fn initialize(
        &self,
        state: &mut State,
        _problem: &P,
        _rng: &mut Random,
        _population: &[Individual],
    ) {
        state.custom.insert(DiversityState {
            diversity: 0.0,
            max_div: 0.0,
        });
    }

    fn postprocess(
        &self,
        state: &mut State,
        problem: &P,
        _rng: &mut Random,
        population: &[Individual],
    ) {
        let diversity_state = state.custom.get_mut::<DiversityState>();

        if population.is_empty() {
            diversity_state.diversity = 0.0;
            return;
        }

        let n = population.len() as f64;
        let d = problem.dimension();
        let iter_solutions = || population.iter().map(|i| i.solution::<Vec<f64>>());

        let selected_measure = self.measure.clone();
        match selected_measure {
            DiversityMeasure::DW => {
                diversity_state.diversity = (0..d)
                    .into_iter()
                    .map(|k| {
                        let xk = iter_solutions().map(|s| s[k]).sum::<f64>() / n;
                        iter_solutions().map(|s| (s[k] - xk).abs()).sum::<f64>() / n
                    })
                    .sum::<f64>()
                    / (d as f64)
            }
            DiversityMeasure::PW => {
                let mut sum = 0.0;
                let solutions: Vec<Vec<f64>> = iter_solutions().cloned().collect();
                for i in 1..n as usize {
                    for j in 0..=i - 1 {
                        sum += (0..d)
                            .into_iter()
                            .map(|k| (solutions[i][k] - solutions[j][k]).powi(2))
                            .sum::<f64>();
                        diversity_state.diversity += sum.sqrt();
                    }
                }
                diversity_state.diversity = diversity_state.diversity * 2.0 / (n * (n - 1.0));
            }
            DiversityMeasure::TD => {
                diversity_state.diversity = (0..d)
                    .into_iter()
                    .map(|k| {
                        let xk = iter_solutions().map(|s| s[k]).sum::<f64>() / n;
                        let sum = iter_solutions().map(|i| i[k].powi(2)).sum::<f64>() / n;
                        sum - xk.powi(2)
                    })
                    .sum::<f64>()
                    .sqrt()
                    / (d as f64)
            }
            DiversityMeasure::DTAP => {
                let mut sum = 0.0;
                for i in iter_solutions() {
                    sum += (0..d)
                        .into_iter()
                        .map(|k| {
                            let xk = iter_solutions().map(|s| s[k]).sum::<f64>() / n;
                            (i[k] - xk).powi(2)
                        })
                        .sum::<f64>()
                        .sqrt();
                }
                diversity_state.diversity = sum / n;
            }
        }

        // set new maximum diversity found so far
        if diversity_state.diversity > diversity_state.max_div {
            diversity_state.max_div = diversity_state.diversity
        }

        // normalize by division with maximum diversity
        diversity_state.diversity /= diversity_state.max_div;
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
    fn initialize(
        &self,
        state: &mut State,
        _problem: &P,
        _rng: &mut Random,
        _population: &[Individual],
    ) {
        state.custom.insert(PopulationState {
            current_pop: vec![],
        });
    }

    fn postprocess(
        &self,
        state: &mut State,
        _problem: &P,
        _rng: &mut Random,
        population: &[Individual],
    ) {
        let population_state = state.custom.get_mut::<PopulationState>();

        population_state.current_pop = population
            .iter()
            .map(|i| i.solution::<Vec<f64>>())
            .cloned()
            .collect();
    }
}
