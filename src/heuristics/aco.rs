//! Ant Colony Optimization

use serde::Serialize;

use crate::{
    framework::{state::CustomState, Configuration},
    operators::*,
    problems::tsp::SymmetricTsp,
};

/// Ant Colony Optimization - Ant System
///
/// # References
/// Dorigo, Marco & Birattari, Mauro & Stützle, Thomas. (2006). Ant Colony Optimization. Computational Intelligence Magazine, IEEE. 1. 28-39. 10.1109/MCI.2006.329691.
pub fn ant_system(
    number_of_ants: usize,
    alpha: f64,
    beta: f64,
    default_pheromones: f64,
    evaporation: f64,
    decay_coefficient: f64,
    max_iterations: u32,
) -> Configuration<SymmetricTsp> {
    Configuration::builder()
        .do_(initialization::Empty::new())
        .while_(
            termination::FixedIterations::new(max_iterations),
            |builder| {
                builder
                    .do_(ant_ops::AcoGeneration::new(
                        number_of_ants,
                        alpha,
                        beta,
                        default_pheromones,
                    ))
                    .do_(evaluation::SerialEvaluator::new())
                    .do_(ant_ops::AsPheromoneUpdate::new(
                        evaporation,
                        decay_coefficient,
                    ))
            },
        )
        .build()
}

/// Ant Colony Optimization - Ant System
///
/// # References
/// Dorigo, Marco & Birattari, Mauro & Stützle, Thomas. (2006). Ant Colony Optimization. Computational Intelligence Magazine, IEEE. 1. 28-39. 10.1109/MCI.2006.329691.
pub fn min_max_ant_system(
    number_of_ants: usize,
    alpha: f64,
    beta: f64,
    default_pheromones: f64,
    evaporation: f64,
    max_pheromones: f64,
    min_pheromones: f64,
    max_iterations: u32,
) -> Configuration<SymmetricTsp> {
    assert!(
        min_pheromones < max_pheromones,
        "min_pheromones must be less than max_pheromones"
    );
    Configuration::builder()
        .do_(initialization::Empty::new())
        .while_(
            termination::FixedIterations::new(max_iterations),
            |builder| {
                builder
                    .do_(ant_ops::AcoGeneration::new(
                        number_of_ants,
                        alpha,
                        beta,
                        default_pheromones,
                    ))
                    .do_(evaluation::SerialEvaluator::new())
                    .do_(ant_ops::MinMaxPheromoneUpdate::new(
                        evaporation,
                        max_pheromones,
                        min_pheromones,
                    ))
            },
        )
        .build()
}

#[derive(Clone, Serialize)]
struct PheromoneMatrix {
    dimension: usize,
    inner: Vec<f64>,
}
impl CustomState for PheromoneMatrix {}
impl PheromoneMatrix {
    pub fn new(dimension: usize, initial_value: f64) -> Self {
        PheromoneMatrix {
            dimension,
            inner: vec![initial_value; dimension * dimension],
        }
    }
}
impl std::ops::Index<usize> for PheromoneMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.dimension);
        let start = index * self.dimension;
        let end = start + self.dimension;
        &self.inner[start..end]
    }
}
impl std::ops::IndexMut<usize> for PheromoneMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.dimension);
        let start = index * self.dimension;
        let end = start + self.dimension;
        &mut self.inner[start..end]
    }
}
impl std::ops::MulAssign<f64> for PheromoneMatrix {
    fn mul_assign(&mut self, rhs: f64) {
        for x in &mut self.inner {
            *x *= rhs;
        }
    }
}

mod ant_ops {
    use rand::distributions::{Distribution, WeightedIndex};

    use crate::{
        framework::{components::*, state::State, Fitness, Individual, Random},
        problems::tsp::{Route, SymmetricTsp},
    };

    use super::PheromoneMatrix;

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
                        .max_by_key(|(_, f)| Fitness::try_from(*f).unwrap())
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
                .map(Individual::new_unevaluated)
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
            let population = mut_state.population_stack().current();

            // Evaporation
            *pm *= 1.0 - self.evaporation;

            // Update pheromones for probabilistic routes
            for individual in population.iter().skip(1) {
                let fitness = individual.fitness().into();
                let route = individual.solution::<Route>();
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
            let population = state.population_stack_mut().pop();
            let pm = state.get_mut::<PheromoneMatrix>();

            // Evaporation
            *pm *= 1.0 - self.evaporation;

            // Update pheromones for probabilistic routes
            let individual = population
                .iter()
                .skip(1)
                .min_by_key(|i| i.fitness())
                .unwrap();

            let fitness = individual.fitness().into();
            let route = individual.solution::<Route>();
            let delta = 1.0 / fitness;
            for (&a, &b) in route.iter().zip(route.iter().skip(1)) {
                pm[a][b] = (pm[a][b] + delta).clamp(self.min_pheromones, self.max_pheromones);
                pm[b][a] = (pm[b][a] + delta).clamp(self.min_pheromones, self.max_pheromones);
            }

            state.population_stack_mut().push(population);
        }
    }
}
