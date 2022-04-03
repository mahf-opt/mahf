use crate::{
    framework::{
        components::{Evaluator, SimpleEvaluator},
        legacy::Configuration,
    },
    problems::Problem,
    random::Random,
    tracking::Log,
};
use std::mem;

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

    #[allow(deprecated)]
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

    archiving.archive(state, rng, problem, population, &mut Vec::new());

    post_replacement.initialize(state, problem, rng, population);
    post_replacement.postprocess(state, problem, rng, population);

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
        if termination.terminate(state, problem) {
            break;
        }
    }

    logger.finalize();

    best.unwrap().into_solution()
}

fn find_best(population: &[Individual]) -> Option<&Individual> {
    population.iter().min_by_key(|i| i.fitness())
}
