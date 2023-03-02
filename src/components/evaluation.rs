//! Evaluation methods

use serde::Serialize;

use crate::{
    framework::components::Component,
    problems::{EvaluatorState, MultiObjectiveProblem, Problem, SingleObjectiveProblem},
    state::{common, State},
};

#[derive(Serialize, Clone)]
pub struct SequentialEvaluator;

impl SequentialEvaluator {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: Problem> Component<P> for SequentialEvaluator {
    fn initialize(&self, problem: &P, state: &mut State) {
        state.require::<common::Population<P>>();
        state.insert(common::Evaluations(0));

        if !state.has::<EvaluatorState<P>>() {
            state.insert(EvaluatorState::from(problem.default_evaluator()));
        }
    }

    fn execute(&self, problem: &P, state: &mut State) {
        if let Some(mut population) = state.population_stack_mut().try_pop() {
            state.holding::<EvaluatorState<P>>(|evaluator_state, state| {
                evaluator_state
                    .evaluator
                    .evaluate(problem, state, &mut population);
            });

            *state.get_value_mut::<common::Evaluations>() += population.len() as u32;
            state.population_stack_mut().push(population);
        }
    }
}

/// Inserts and updates the [common::BestIndividual] state.
#[derive(Serialize, Clone)]
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
#[derive(Serialize, Clone)]
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
