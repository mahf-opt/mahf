//! Ant Colony Optimization

use crate::{
    framework::{legacy, CustomState},
    operators::*,
    problems::tsp::SymmetricTsp,
    tracking::log::CustomLog,
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
) -> legacy::Configuration<SymmetricTsp> {
    legacy::Configuration {
        initialization: ant_ops::AcoInitialization::new(default_pheromones),
        selection: selection::FullyRandom::new(0),
        generation: vec![ant_ops::AcoGeneration::new(number_of_ants, alpha, beta)],
        replacement: ant_ops::AsReplacement::new(evaporation, decay_coefficient),
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
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
) -> legacy::Configuration<SymmetricTsp> {
    assert!(
        min_pheromones < max_pheromones,
        "min_pheromones must be less than max_pheromones"
    );

    legacy::Configuration {
        initialization: ant_ops::AcoInitialization::new(default_pheromones),
        selection: selection::FullyRandom::new(0),
        generation: vec![ant_ops::AcoGeneration::new(number_of_ants, alpha, beta)],
        replacement: ant_ops::MinMaxReplacement::new(evaporation, max_pheromones, min_pheromones),
        termination: termination::FixedIterations::new(max_iterations),
        ..Default::default()
    }
}

struct PheromoneMatrix {
    dimension: usize,
    inner: Vec<f64>,
}
impl PheromoneMatrix {
    pub fn new(dimension: usize, initial_value: f64) -> Self {
        PheromoneMatrix {
            dimension,
            inner: vec![initial_value; dimension * dimension],
        }
    }
}
impl CustomState for PheromoneMatrix {
    fn iteration_log(&self) -> Vec<CustomLog> {
        let mut min = self.inner[0];
        let mut max = self.inner[0];
        let mut sum = 0.0;

        for &x in &self.inner {
            min = f64::min(min, x);
            max = f64::max(max, x);
            sum += x;
        }
        let avg = sum / (self.inner.len() as f64);

        vec![
            CustomLog {
                name: "avg_pheromone",
                value: Some(avg),
                solutions: None,
            },
            CustomLog {
                name: "min_pheromone",
                value: Some(min),
                solutions: None,
            },
            CustomLog {
                name: "max_pheromone",
                value: Some(max),
                solutions: None,
            },
        ]
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

#[allow(clippy::new_ret_no_self)]
mod ant_ops {
    use super::PheromoneMatrix;
    use crate::{
        framework::{
            components::*,
            legacy::{components::*, State},
            Fitness, Individual,
        },
        problems::{
            tsp::{Route, SymmetricTsp},
            Problem,
        },
        random::Random,
    };
    use rand::distributions::{weighted::WeightedIndex, Distribution};

    #[derive(serde::Serialize)]
    pub struct AcoInitialization {
        pub default_pheromones: f64,
    }
    impl AcoInitialization {
        pub fn new(default_pheromones: f64) -> Box<dyn Component<SymmetricTsp>> {
            Box::new(Initializer(Self { default_pheromones }))
        }
    }
    impl Initialization<SymmetricTsp> for AcoInitialization {
        fn initialize(
            &self,
            state: &mut State,
            problem: &SymmetricTsp,
            _rng: &mut Random,
            _population: &mut Vec<Route>,
        ) {
            state.custom.insert(PheromoneMatrix::new(
                problem.dimension,
                self.default_pheromones,
            ));
        }
    }

    #[derive(serde::Serialize)]
    pub struct AcoGeneration {
        pub number_of_ants: usize,
        pub alpha: f64,
        pub beta: f64,
    }
    impl AcoGeneration {
        pub fn new(
            number_of_ants: usize,
            alpha: f64,
            beta: f64,
        ) -> Box<dyn Component<SymmetricTsp>> {
            Box::new(Generator(Self {
                number_of_ants,
                alpha,
                beta,
            }))
        }
    }
    impl Generation<SymmetricTsp> for AcoGeneration {
        fn generate(
            &self,
            state: &mut State,
            problem: &SymmetricTsp,
            rng: &mut Random,
            _parents: &mut Vec<Route>,
            offspring: &mut Vec<Route>,
        ) {
            let pm = state.custom.get_mut::<PheromoneMatrix>();

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
                offspring.push(route);
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
                        m.powf(self.alpha) * (1.0 / d).powf(self.beta) + 0.000000000000001
                    });
                    let dist = WeightedIndex::new(weights).unwrap();
                    let next_index = dist.sample(rng);
                    let next = remaining.remove(next_index);
                    route.push(next);
                }
                offspring.push(route);
            }
        }
    }

    #[derive(serde::Serialize)]
    pub struct AsReplacement {
        pub evaporation: f64,
        pub decay_coefficient: f64,
    }
    impl AsReplacement {
        pub fn new<P: Problem>(evaporation: f64, decay_coefficient: f64) -> Box<dyn Component<P>> {
            Box::new(Replacer(Self {
                evaporation,
                decay_coefficient,
            }))
        }
    }
    impl Replacement for AsReplacement {
        fn replace(
            &self,
            state: &mut State,
            _rng: &mut Random,
            _population: &mut Vec<Individual>,
            offspring: &mut Vec<Individual>,
        ) {
            let pm = state.custom.get_mut::<PheromoneMatrix>();

            // Evaporation
            *pm *= 1.0 - self.evaporation;

            // Update pheromones for probabilistic routes
            for individual in offspring.iter().skip(1) {
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
    pub struct MinMaxReplacement {
        pub evaporation: f64,
        pub max_pheromones: f64,
        pub min_pheromones: f64,
    }
    impl MinMaxReplacement {
        pub fn new<P: Problem>(
            evaporation: f64,
            max_pheromones: f64,
            min_pheromones: f64,
        ) -> Box<dyn Component<P>> {
            Box::new(Replacer(Self {
                evaporation,
                max_pheromones,
                min_pheromones,
            }))
        }
    }
    impl Replacement for MinMaxReplacement {
        fn replace(
            &self,
            state: &mut State,
            _rng: &mut Random,
            _population: &mut Vec<Individual>,
            offspring: &mut Vec<Individual>,
        ) {
            let pm = state.custom.get_mut::<PheromoneMatrix>();

            // Evaporation
            *pm *= 1.0 - self.evaporation;

            // Update pheromones for probabilistic routes
            let individual = offspring
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
        }
    }
}
