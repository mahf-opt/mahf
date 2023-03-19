//! Evaluation methods

use serde::Serialize;

use crate::{
    framework::components::Component,
    problems::{MultiObjectiveProblem, Problem, SingleObjectiveProblem},
    state::{common, State},
};

/// Evaluates all individuals in the current population.
///
/// This component should be inserted after every generating component.
///
/// Only the head of the [common::Population] will be evaluated.
/// Requires either [common::EvaluatorInstance] to be present
/// in the [State] or [Problem::default_evaluator] to be implemented.
///
/// By inserting a custom [common::EvaluatorInstance] the evaluation
/// behavior can be customized.
#[derive(Serialize, Clone)]
pub struct Evaluator;

impl Evaluator {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}

impl<P: Problem> Component<P> for Evaluator {
    fn initialize(&self, problem: &P, state: &mut State) {
        state.require::<common::Population<P>>();
        state.insert(common::Evaluations(0));

        if !state.has::<common::EvaluatorInstance<P>>() {
            state.insert(common::EvaluatorInstance::from(problem.default_evaluator()));
        }
    }

    fn execute(&self, problem: &P, state: &mut State) {
        if let Some(mut population) = state.population_stack_mut().try_pop() {
            state.holding::<common::EvaluatorInstance<P>>(|evaluator_state, state| {
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
///
/// Should be inserted right after [Evaluator].
/// For [MultiObjectiveProblem]s see [UpdateParetoFront].
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
///
/// Should be inserted right after [Evaluator].
/// For [SingleObjectiveProblem]s see [UpdateBestIndividual].
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
