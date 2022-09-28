//! Ant Colony Optimization

use crate::{
    framework::{components::Component, conditions::Condition, Configuration},
    operators::*,
    problems::{tsp, SingleObjectiveProblem},
};

/// Parameters for [ant_system].
pub struct ASParameters {
    number_of_ants: usize,
    alpha: f64,
    beta: f64,
    default_pheromones: f64,
    evaporation: f64,
    decay_coefficient: f64,
}

/// Ant Colony Optimization - Ant System
/// Uses the [aco] component internally.
///
/// # References
/// [doi.org/10.1109/MCI.2006.329691](https://doi.org/10.1109/MCI.2006.329691)
pub fn ant_system(
    params: ASParameters,
    termination: Box<dyn Condition<tsp::SymmetricTsp>>,
    logger: Box<dyn Component<tsp::SymmetricTsp>>,
) -> Configuration<tsp::SymmetricTsp> {
    let ASParameters {
        number_of_ants,
        alpha,
        beta,
        default_pheromones,
        evaporation,
        decay_coefficient,
    } = params;

    Configuration::builder()
        .do_(initialization::Empty::new())
        .do_(aco(
            Parameters {
                generation: ant_ops::AcoGeneration::new(
                    number_of_ants,
                    alpha,
                    beta,
                    default_pheromones,
                ),
                pheromone_update: ant_ops::AsPheromoneUpdate::new(evaporation, decay_coefficient),
            },
            termination,
            logger,
        ))
        .build()
}

/// Parameters for [max_min_ant_system].
pub struct MMASParameters {
    number_of_ants: usize,
    alpha: f64,
    beta: f64,
    default_pheromones: f64,
    evaporation: f64,
    max_pheromones: f64,
    min_pheromones: f64,
}

/// Ant Colony Optimization - MAX-MIN Ant System
/// Uses the [aco] component internally.
///
/// # References
/// [doi.org/10.1109/MCI.2006.329691](https://doi.org/10.1109/MCI.2006.329691)
pub fn max_min_ant_system(
    params: MMASParameters,
    termination: Box<dyn Condition<tsp::SymmetricTsp>>,
    logger: Box<dyn Component<tsp::SymmetricTsp>>,
) -> Configuration<tsp::SymmetricTsp> {
    let MMASParameters {
        number_of_ants,
        alpha,
        beta,
        default_pheromones,
        evaporation,
        max_pheromones,
        min_pheromones,
    } = params;

    Configuration::builder()
        .do_(initialization::Empty::new())
        .do_(aco(
            Parameters {
                generation: ant_ops::AcoGeneration::new(
                    number_of_ants,
                    alpha,
                    beta,
                    default_pheromones,
                ),
                pheromone_update: ant_ops::MinMaxPheromoneUpdate::new(
                    evaporation,
                    max_pheromones,
                    min_pheromones,
                ),
            },
            termination,
            logger,
        ))
        .build()
}

/// Basic building blocks of Ant Colony Optimization.
pub struct Parameters<P> {
    generation: Box<dyn Component<P>>,
    pheromone_update: Box<dyn Component<P>>,
}

/// A generic single-objective Ant Colony Optimization template.
pub fn aco<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>> {
    let Parameters {
        generation,
        pheromone_update,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(generation)
                .evaluate_sequential()
                .update_best_individual()
                .do_(pheromone_update)
                .do_(logger)
        })
        .build_component()
}

mod ant_ops {
    use crate::operators::state::PheromoneMatrix;
    use crate::{
        framework::{components::*, state::State, Individual, Random, SingleObjective},
        problems::tsp::SymmetricTsp,
    };
    use rand::distributions::{Distribution, WeightedIndex};

    #[derive(serde::Serialize)]
    pub struct AcoGeneration {
        pub number_of_ants: usize,
        pub alpha: f64,
        pub beta: f64,
        pub default_pheromones: f64,
    }
    impl AcoGeneration {
        pub fn new(
            number_of_ants: usize,
            alpha: f64,
            beta: f64,
            default_pheromones: f64,
        ) -> Box<dyn Component<SymmetricTsp>> {
            Box::new(Self {
                number_of_ants,
                alpha,
                beta,
                default_pheromones,
            })
        }
    }
    impl Component<SymmetricTsp> for AcoGeneration {
        fn initialize(&self, problem: &SymmetricTsp, state: &mut State) {
            state.insert(PheromoneMatrix::new(
                problem.dimension,
                self.default_pheromones,
            ));
        }

        fn execute(&self, problem: &SymmetricTsp, state: &mut State) {
            let (pm, rng) = state.get_multiple_mut::<(PheromoneMatrix, Random)>();
            let mut routes = Vec::new();

            // Greedy route
            {
                let mut remaining = (1..problem.dimension).into_iter().collect::<Vec<usize>>();
                let mut route = Vec::with_capacity(problem.dimension);
                route.push(0);
                while !remaining.is_empty() {
                    let last = *route.last().unwrap();
                    let pheromones = remaining.iter().map(|&r| pm[last][r]);
                    let next_index = pheromones
                        .enumerate()
                        .max_by_key(|(_, f)| SingleObjective::try_from(*f).unwrap())
                        .unwrap()
                        .0;
                    let next = remaining.remove(next_index);
                    route.push(next);
                }
                routes.push(route);
            }

            // Probabilistic routes
            for _ in 0..self.number_of_ants {
                let mut remaining = (1..problem.dimension).into_iter().collect::<Vec<usize>>();
                let mut route = Vec::with_capacity(problem.dimension);
                route.push(0);
                while !remaining.is_empty() {
                    let last = *route.last().unwrap();
                    let distances = remaining
                        .iter()
                        .map(|&r| problem.distance((last, r)) as f64);
                    let pheromones = remaining.iter().map(|&r| pm[last][r]);
                    let weights = pheromones.zip(distances).map(|(m, d)| {
                        // TODO: This should not be zero.
                        m.powf(self.alpha) * (1.0 / d).powf(self.beta) + 1e-15
                    });
                    let dist = WeightedIndex::new(weights).unwrap();
                    let next_index = dist.sample(rng);
                    let next = remaining.remove(next_index);
                    route.push(next);
                }
                routes.push(route);
            }

            let population = routes
                .into_iter()
                .map(Individual::<SymmetricTsp>::new_unevaluated)
                .collect();
            *state.population_stack_mut().current_mut() = population;
        }
    }

    #[derive(serde::Serialize)]
    pub struct AsPheromoneUpdate {
        pub evaporation: f64,
        pub decay_coefficient: f64,
    }
    impl AsPheromoneUpdate {
        pub fn new(evaporation: f64, decay_coefficient: f64) -> Box<dyn Component<SymmetricTsp>> {
            Box::new(Self {
                evaporation,
                decay_coefficient,
            })
        }
    }
    impl Component<SymmetricTsp> for AsPheromoneUpdate {
        fn initialize(&self, _problem: &SymmetricTsp, state: &mut State) {
            state.require::<PheromoneMatrix>();
        }

        fn execute(&self, _problem: &SymmetricTsp, state: &mut State) {
            let mut mut_state = state.get_states_mut();
            let pm = mut_state.get_mut::<PheromoneMatrix>();
            let population = mut_state.population_stack::<SymmetricTsp>().current();

            // Evaporation
            *pm *= 1.0 - self.evaporation;

            // Update pheromones for probabilistic routes
            for individual in population.iter().skip(1) {
                let fitness = individual.objective().value();
                let route = individual.solution();
                let delta = self.decay_coefficient / fitness;
                for (&a, &b) in route.iter().zip(route.iter().skip(1)) {
                    pm[a][b] += delta;
                    pm[b][a] += delta;
                }
            }
        }
    }

    #[derive(serde::Serialize)]
    pub struct MinMaxPheromoneUpdate {
        pub evaporation: f64,
        pub max_pheromones: f64,
        pub min_pheromones: f64,
    }
    impl MinMaxPheromoneUpdate {
        pub fn new(
            evaporation: f64,
            max_pheromones: f64,
            min_pheromones: f64,
        ) -> Box<dyn Component<SymmetricTsp>> {
            assert!(
                min_pheromones < max_pheromones,
                "min_pheromones must be less than max_pheromones"
            );
            Box::new(Self {
                evaporation,
                max_pheromones,
                min_pheromones,
            })
        }
    }
    impl Component<SymmetricTsp> for MinMaxPheromoneUpdate {
        fn initialize(&self, _problem: &SymmetricTsp, state: &mut State) {
            state.require::<PheromoneMatrix>();
        }

        fn execute(&self, _problem: &SymmetricTsp, state: &mut State) {
            let mut mut_state = state.get_states_mut();
            let pm = mut_state.get_mut::<PheromoneMatrix>();
            let population = mut_state.population_stack::<SymmetricTsp>().current();

            // Evaporation
            *pm *= 1.0 - self.evaporation;

            // Update pheromones for probabilistic routes
            let individual = population
                .iter()
                .skip(1)
                .min_by_key(|i| i.objective())
                .unwrap();

            let fitness = individual.objective().value();
            let route = individual.solution();
            let delta = 1.0 / fitness;
            for (&a, &b) in route.iter().zip(route.iter().skip(1)) {
                pm[a][b] = (pm[a][b] + delta).clamp(self.min_pheromones, self.max_pheromones);
                pm[b][a] = (pm[b][a] + delta).clamp(self.min_pheromones, self.max_pheromones);
            }
        }
    }
}
