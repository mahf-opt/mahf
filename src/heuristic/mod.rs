//! Framework for modular heuristics.

use crate::{fitness::Fitness, problem::Problem, random::Random, tracking::Log};
use std::convert::TryFrom;

pub mod components;

mod state;
pub use state::State;

mod individual;
pub use individual::Individual;

mod config;
pub use config::Configuration;

/// Run the provided [Configuration] in the framework.
///
/// Returns the best solution encountered during the entire run.
pub fn run<P: Problem>(
    problem: &P,
    logger: &mut Log,
    components: &Configuration<P>,
    rng: Option<Random>,
    evaluator: Option<Box<dyn Evaluator<P>>>,
) -> P::Encoding {
    let evaluator = &mut evaluator.unwrap_or_else(|| Box::new(SimpleEvaluator));
    let rng = &mut rng.unwrap_or_default();
    let Configuration {
        initialization,
        selection,
        generation,
        replacement,
        termination,
    } = components;

    let initial_population = &mut Vec::new();
    let population = &mut Vec::new();

    // State shared across components
    let state = &mut State::new();

    // Initialisation
    initialization.initialize(state, problem, rng, initial_population);

    // Initial evaluation
    evaluator.evaluate(state, problem, initial_population, population);
    for evaluated in population.iter() {
        state.log_evaluation(logger, evaluated.fitness());
    }

    let mut best = find_best(population).clone::<P::Encoding>();

    // Loop until Termination
    loop {
        let parent_individuals = &mut Vec::new();
        let parents = &mut Vec::new();
        let offspring = &mut Vec::new();
        let evaluated_offspring = &mut Vec::new();

        // Selection
        selection.select(state, rng, population, parent_individuals);
        parents.extend(
            parent_individuals
                .drain(..)
                .map(|i| i.solution::<P::Encoding>()),
        );

        // Generation
        generation.generate(state, problem, rng, parents, offspring);

        // Evaluation
        evaluator.evaluate(state, problem, offspring, evaluated_offspring);
        for evaluated in evaluated_offspring.iter() {
            if evaluated.fitness() < best.fitness() {
                best = evaluated.clone::<P::Encoding>();
            }
            state.log_evaluation(logger, evaluated.fitness());
        }

        // Replancement + Update
        replacement.replace(state, rng, population, evaluated_offspring);

        state.log_iteration(logger);
        if termination.terminate(state) {
            break;
        }
    }

    logger.finalize();

    best.into_solution()
}

fn find_best(population: &[Individual]) -> &Individual {
    population.iter().min_by_key(|i| i.fitness()).unwrap()
}

pub trait Evaluator<P: Problem> {
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
        _state: &mut State,
        problem: &P,
        offspring: &mut Vec<P::Encoding>,
        evaluated: &mut Vec<Individual>,
    ) {
        for solution in offspring.drain(..) {
            let fitness = Fitness::try_from(problem.evaluate(&solution)).unwrap();
            let solution = Box::new(solution);
            evaluated.push(Individual::new(solution, fitness));
        }
    }
}
