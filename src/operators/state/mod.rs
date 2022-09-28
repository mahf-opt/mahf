//! Custom States and corresponding Operators

pub mod archive;
pub mod custom_state;
pub mod diversity;

use crate::operators::state::custom_state::PsoState;
use crate::problems::SingleObjectiveProblem;
use crate::{
    framework::{components::*, state::State, Individual},
    problems::{LimitedVectorProblem, Problem},
};
use rand::Rng;

/// State initialization for PSO.
///
/// Initializes velocities, best found solutions of particles and global best in [PsoState].
#[allow(clippy::new_ret_no_self)]
#[derive(Debug, serde::Serialize)]
pub struct PsoStateInitialization {
    v_max: f64,
}
impl PsoStateInitialization {
    pub fn new<P: Problem>(v_max: f64) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
    {
        Box::new(Self { v_max })
    }
}
impl<P> Component<P> for PsoStateInitialization
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        // Initialize with empty state to satisfy `state.require()` statements
        state.insert(PsoState {
            velocities: vec![],
            bests: vec![],
            global_best: Individual::<P>::new_unevaluated(Vec::new()),
        })
    }

    fn execute(&self, problem: &P, state: &mut State) {
        let population = state.population_stack_mut::<P>().pop();
        let rng = state.random_mut();

        let velocities = population
            .iter()
            .map(|_| {
                (0..problem.dimension())
                    .into_iter()
                    .map(|_| rng.gen_range(-self.v_max..=self.v_max))
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>();

        let bests = population.to_vec();

        let global_best = bests
            .iter()
            .min_by_key(|i| Individual::objective(i))
            .cloned()
            .unwrap();

        state.population_stack_mut().push(population);

        state.insert(PsoState {
            velocities,
            bests,
            global_best,
        });
    }
}

/// State update for PSO.
///
/// Updates best found solutions of particles and global best in [PsoState].
#[derive(Debug, serde::Serialize)]
pub struct PsoStateUpdate;
impl PsoStateUpdate {
    pub fn new<P: Problem>() -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
    {
        Box::new(Self)
    }
}
impl<P> Component<P> for PsoStateUpdate
where
    P: Problem<Encoding = Vec<f64>> + LimitedVectorProblem<T = f64>,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<PsoState<P>>();
    }

    fn execute(&self, _problem: &P, state: &mut State) {
        let population = state.population_stack_mut().pop();
        let mut pso_state = state.get_mut::<PsoState<P>>();

        for (i, individual) in population.iter().enumerate() {
            if pso_state.bests[i].objective() > individual.objective() {
                pso_state.bests[i] = individual.clone();

                if pso_state.global_best.objective() > individual.objective() {
                    pso_state.global_best = individual.clone();
                }
            }
        }

        state.population_stack_mut().push(population);
    }
}
