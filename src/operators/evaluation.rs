use serde::Serialize;

use crate::{
    framework::{
        components::Component,
        state::{common, State},
    },
    problems::{MultiObjectiveProblem, Problem, SingleObjectiveProblem},
};

#[derive(Serialize)]
pub struct SerialEvaluator;

impl SerialEvaluator {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: Problem> Component<P> for SerialEvaluator {
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<common::Population<P>>();
        state.insert(common::Evaluations(0));
    }

    fn execute(&self, problem: &P, state: &mut State) {
        let mut mut_state = state.get_states_mut();

        // Evaluate population
        let population = mut_state.population_stack_mut();

        if population.is_empty() {
            return;
        }

        for individual in population.current_mut().iter_mut() {
            if !individual.is_evaluated() {
                problem.evaluate(individual);
            }
        }

        // Update evaluations
        *mut_state.get_value_mut::<common::Evaluations>() += population.current().len() as u32;
    }
}

/// Inserts and updates the [common::BestIndividual] state.
#[derive(Serialize)]
pub struct UpdateBestIndividual;

impl UpdateBestIndividual {
    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: SingleObjectiveProblem> Component<P> for UpdateBestIndividual {
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.insert(common::BestIndividual::<P>(None));
    }

    fn execute(&self, _problem: &P, state: &mut State) {
        let mut mut_state = state.get_states_mut();

        let best_individual = mut_state
            .population_stack()
            .current()
            .iter()
            .min_by_key(|i| i.objective());

        if let Some(best_individual) = best_individual {
            mut_state
                .get_mut::<common::BestIndividual<P>>()
                .replace_if_better(best_individual);
        }
    }
}

/// Inserts and updates the [common::ParetoFront] state.
#[derive(Serialize)]
pub struct UpdateParetoFront;

impl UpdateParetoFront {
    pub fn new<P: MultiObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: MultiObjectiveProblem> Component<P> for UpdateParetoFront {
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.insert(common::ParetoFront::<P>::new());
    }

    fn execute(&self, _problem: &P, state: &mut State) {
        let mut mut_state = state.get_states_mut();

        let front = mut_state.get_mut::<common::ParetoFront<P>>();
        front.update_multiple(mut_state.population_stack().current());
    }
}
