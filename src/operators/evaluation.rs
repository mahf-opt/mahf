use serde::Serialize;

use crate::{
    framework::{
        components::Component,
        state::{common, State},
        Fitness,
    },
    problems::Problem,
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
        state.require::<common::Population>();
        state.insert(common::Evaluations(0));
        state.insert(common::BestFitness(Fitness::default()));
        state.insert(common::BestIndividual(None));
    }

    fn execute(&self, problem: &P, state: &mut State) {
        let mut mut_state = state.get_states_mut();

        // Evaluate population
        let population = mut_state.population_stack_mut();

        if population.is_empty() {
            return;
        }

        for individual in population.current_mut().iter_mut() {
            let solution = individual.solution::<P::Encoding>();
            let fitness = Fitness::try_from(problem.evaluate(solution)).unwrap();
            individual.evaluate(fitness);
        }

        // Update best fitness and individual
        let best_individual = population.best();

        if mut_state
            .get_mut::<common::BestIndividual>()
            .replace_if_better(best_individual)
        {
            mut_state.set_value::<common::BestFitness>(best_individual.fitness());
        }

        // Update evaluations
        *mut_state.get_value_mut::<common::Evaluations>() += population.current().len() as u32;
    }
}
