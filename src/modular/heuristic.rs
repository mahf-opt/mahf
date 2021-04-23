use crate::{
    fitness::Fitness,
    functions::Problem,
    modular::{components::*, Individual, Solution, State},
    tracking::Log,
};
use std::convert::TryFrom;

pub fn run(
    problem: &Problem,
    logger: &mut Log,

    mut initialization: impl Initialization,
    mut selection: impl Selection,
    mut generation: impl Generation,
    mut replacement: impl Replacement,
    mut termination: impl Termination,
) {
    // This could be an additional component,
    // supporting parallel or GPU evaluation.
    let mut evaluator = SimpleEvaluator;

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
        let parents = &mut Vec::new();
        let offspring = &mut Vec::new();
        let evaluated_offspring = &mut Vec::new();

        // Selection
        selection.select(state, population, parents);

        // Generation
        generation.generate(state, problem, parents, offspring);

        // Evaluation
        evaluator.evaluate(state, problem, offspring, evaluated_offspring);

        // Replancement + Update
        replacement.replace(state, population, evaluated_offspring);

        state.log_iteration(calculate_diversity(population));

        if termination.terminate(state) {
            break;
        }
    }
}

fn calculate_diversity(x: &[Individual]) -> f64 {
    if x.is_empty() {
        return 0.0;
    }

    let m = x.len() as f64;
    let d = x[0].solution.len();

    (0..d)
        .into_iter()
        .map(|j| {
            let xj = x.iter().map(|i| i.solution[j]).sum::<f64>() / m;
            x.iter().map(|i| (i.solution[j] - xj).abs()).sum::<f64>() / m
        })
        .sum::<f64>()
        / (d as f64)
}

trait Evaluator {
    fn evaluate(
        &mut self,
        state: &mut State,
        problem: &Problem,
        offspring: &mut Vec<Solution>,
        evaluated: &mut Vec<Individual>,
    );
}

struct SimpleEvaluator;
impl Evaluator for SimpleEvaluator {
    fn evaluate(
        &mut self,
        state: &mut State,
        problem: &Problem,
        offspring: &mut Vec<Solution>,
        evaluated: &mut Vec<Individual>,
    ) {
        for solution in offspring.drain(..) {
            let fitness = Fitness::try_from((problem.function)(solution.as_slice())).unwrap();
            state.log_evaluation(fitness);
            evaluated.push(Individual { fitness, solution });
        }
    }
}
