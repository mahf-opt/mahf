use crate::{fitness::Fitness, problem::Problem, tracking::Log};
use std::convert::TryFrom;

pub mod components;

mod state;
pub use state::State;

mod individual;
pub use individual::Individual;

mod config;
pub use config::Configuration;

pub fn run<P: Problem>(problem: &P, logger: &mut Log, components: Configuration<P>) {
    // This could be an additional component,
    // supporting parallel or GPU evaluation.
    let mut evaluator = SimpleEvaluator;
    let Configuration {
        mut initialization,
        mut selection,
        mut generation,
        mut replacement,
        mut termination,
    } = components;

    let initial_population = &mut Vec::new();
    let population = &mut Vec::new();

    // Initialisation
    initialization.initialize(problem, initial_population);

    // State shared across components
    let state = &mut State::new(logger);

    // Initial evaluation
    evaluator.evaluate(state, problem, initial_population, population);

    // Loop until Termination
    loop {
        let parent_individuals = &mut Vec::new();
        let parents = &mut Vec::new();
        let offspring = &mut Vec::new();
        let evaluated_offspring = &mut Vec::new();

        // Selection
        selection.select(state, population, parent_individuals);
        parents.extend(
            parent_individuals
                .drain(..)
                .map(|i| i.solution::<P::Encoding>()),
        );

        // Generation
        generation.generate(state, problem, parents, offspring);

        // Evaluation
        evaluator.evaluate(state, problem, offspring, evaluated_offspring);

        // Replancement + Update
        replacement.replace(state, population, evaluated_offspring);

        state.log_iteration();

        if termination.terminate(state) {
            break;
        }
    }
}

trait Evaluator<P: Problem> {
    fn evaluate(
        &mut self,
        state: &mut State,
        problem: &P,
        offspring: &mut Vec<P::Encoding>,
        evaluated: &mut Vec<Individual>,
    );
}

struct SimpleEvaluator;
impl<P: Problem> Evaluator<P> for SimpleEvaluator {
    fn evaluate(
        &mut self,
        state: &mut State,
        problem: &P,
        offspring: &mut Vec<P::Encoding>,
        evaluated: &mut Vec<Individual>,
    ) {
        for solution in offspring.drain(..) {
            let fitness = Fitness::try_from(problem.evaluate(&solution)).unwrap();
            let solution = Box::new(solution);
            state.log_evaluation(fitness);
            evaluated.push(Individual::new(solution, fitness));
        }
    }
}
