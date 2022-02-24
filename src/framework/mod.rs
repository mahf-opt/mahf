#![doc = embed_doc_image::embed_image!("module_system", "docs/MAHF-module-system.svg")]
#![doc = include_str!("../../docs/heuristic.md")]

use crate::{problems::Problem, random::Random, tracking::Log};
use std::mem;

pub mod components;

mod fitness;
pub use fitness::{Fitness, IllegalFitness};

mod state;
pub use state::State;

mod custom_state;
pub use custom_state::{CustomState, CustomStateMap};

mod individual;
pub use individual::Individual;

mod config;
pub use config::{Configuration};

/// Run the provided [Configuration] in the framework.
///
/// Returns the best solution encountered during the entire run.
/// More detailed information can be obtained from the [Log].
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
        generation_scheduler,
        replacement,
        archiving,
        post_replacement,
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

    if let Some(archiving) = archiving {
        archiving.archive(state, rng, problem, population, &mut Vec::new());
    }

    if let Some(post_replacement) = post_replacement {
        post_replacement.initialize(state, problem, rng, population);
        post_replacement.postprocess(state, problem, rng, population);
    }

    let mut best: Option<Individual> = find_best(population).map(Individual::clone);

    // Loop until Termination
    loop {
        let generation_schedule = &mut Vec::new();
        let parent_individuals = &mut Vec::new();
        let mut parents = &mut Vec::new();
        let mut offspring = &mut Vec::new();
        let evaluated_offspring = &mut Vec::new();

        // Selection
        selection.select(state, rng, population, parent_individuals);

        // Generation
        generation_scheduler.schedule(
            state,
            rng,
            generation.len(),
            population,
            parent_individuals,
            generation_schedule,
        );
        parents.extend(
            parent_individuals
                .drain(..)
                .map(|i| i.solution::<P::Encoding>().clone()),
        );
        for generator in generation_schedule.iter().map(|&i| &generation[i]) {
            generator.generate(state, problem, rng, parents, offspring);

            parents.clear();
            mem::swap(&mut parents, &mut offspring);
        }
        mem::swap(&mut parents, &mut offspring);

        // Evaluation
        evaluator.evaluate(state, problem, offspring, evaluated_offspring);
        for evaluated in evaluated_offspring.iter() {
            if evaluated.fitness() < best.as_ref().map(Individual::fitness).unwrap_or_default() {
                best = Some(evaluated.clone());
            }
            state.log_evaluation(logger, evaluated.fitness());
        }

        // Replancement + Update
        replacement.replace(state, rng, population, evaluated_offspring);

        // Archiving
        archiving.archive(state, rng, problem, population, evaluated_offspring);

        // Postprocessing
        post_replacement.postprocess(state, problem, rng, population);

        state.log_iteration(logger);
        if termination.terminate(state) {
            break;
        }
    }

    logger.finalize();

    best.unwrap().into_solution()
}

fn find_best(population: &[Individual]) -> Option<&Individual> {
    population.iter().min_by_key(|i| i.fitness())
}

/// Evaluates solutions.
///
/// Can be used to customize how solutions should be evaluated.
/// One use case for this would be GPU evaluation.
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
            evaluated.push(Individual::new::<P::Encoding>(solution, fitness));
        }
    }
}
