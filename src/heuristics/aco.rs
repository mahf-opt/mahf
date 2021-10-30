//! Ant Colony Optimization

use crate::{
    fitness::Fitness,
    heuristic::{components::*, Configuration, CustomState, Individual, State},
    operators::*,
    problems::tsp::{Route, SymmetricTsp},
    random::Random,
    tracking::CustomLog,
};
use rand::distributions::{weighted::WeightedIndex, Distribution};
use std::ops::{Index, IndexMut, MulAssign};

/// Ant Colony Optimization - Ant System
///
/// # References
/// Dorigo, Marco & Birattari, Mauro & Stützle, Thomas. (2006). Ant Colony Optimization. Computational Intelligence Magazine, IEEE. 1. 28-39. 10.1109/MCI.2006.329691.
pub fn ant_stystem(
    number_of_ants: usize,
    alpha: f64,
    beta: f64,
    default_pheromones: f64,
    evaporation: f64,
    decay_coefficient: f64,
    max_iterations: u32,
) -> Configuration<SymmetricTsp> {
    Configuration::new(
        AcoInitialization { default_pheromones },
        selection::FullyRandom { offspring: 0 },
        AcoGeneration {
            number_of_ants,
            alpha,
            beta,
        },
        AsReplacement {
            evaporation,
            decay_coefficient,
        },
        termination::FixedIterations { max_iterations },
    )
}

/// Ant Colony Optimization - Ant System
///
/// # References
/// Dorigo, Marco & Birattari, Mauro & Stützle, Thomas. (2006). Ant Colony Optimization. Computational Intelligence Magazine, IEEE. 1. 28-39. 10.1109/MCI.2006.329691.
pub fn min_max_ant_stystem(
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

    Configuration::new(
        AcoInitialization { default_pheromones },
        selection::FullyRandom { offspring: 0 },
        AcoGeneration {
            number_of_ants,
            alpha,
            beta,
        },
        MinMaxReplacement {
            evaporation,
            max_pheromones,
            min_pheromones,
        },
        termination::FixedIterations { max_iterations },
    )
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
                value: avg,
            },
            CustomLog {
                name: "min_pheromone",
                value: min,
            },
            CustomLog {
                name: "max_pheromone",
                value: max,
            },
        ]
    }
}
impl Index<usize> for PheromoneMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.dimension);
        let start = index * self.dimension;
        let end = start + self.dimension;
        &self.inner[start..end]
    }
}
impl IndexMut<usize> for PheromoneMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.dimension);
        let start = index * self.dimension;
        let end = start + self.dimension;
        &mut self.inner[start..end]
    }
}
impl MulAssign<f64> for PheromoneMatrix {
    fn mul_assign(&mut self, rhs: f64) {
        for x in &mut self.inner {
            *x *= rhs;
        }
    }
}

#[derive(serde::Serialize)]
pub struct AcoInitialization {
    pub default_pheromones: f64,
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
