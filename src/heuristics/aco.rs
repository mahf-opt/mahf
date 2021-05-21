pub mod v1 {
    use crate::{
        fitness::Fitness,
        heuristic::{components::*, Configuration, Individual, State},
        operators::*,
        problems::tsp::{Route, SymmetricTsp},
        random::Random,
    };
    use rand::distributions::{weighted::WeightedIndex, Distribution};
    use std::{
        convert::TryFrom,
        ops::{Index, IndexMut, MulAssign},
    };

    pub fn aco(
        number_of_ants: usize,
        alpha: f64,
        beta: f64,
        default_pheromones: f64,
        evaporation: f64,
        heuristic_length: f64,
        max_iterations: u32,
    ) -> Configuration<SymmetricTsp> {
        Configuration::new(
            AcoInitialization { default_pheromones },
            AcoSelection,
            AcoGeneration {
                number_of_ants,
                alpha,
                beta,
            },
            AcoReplacement {
                evaporation,
                heuristic_length,
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
    pub struct AcoSelection;
    impl Selection for AcoSelection {
        fn select<'p>(
            &self,
            _state: &mut State,
            _rng: &mut Random,
            _population: &'p [Individual],
            _selection: &mut Vec<&'p Individual>,
        ) {
            // Nothing to do
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
            _parents: &mut Vec<&Route>,
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
                    let weights = pheromones
                        .zip(distances)
                        .map(|(m, d)| m.powf(self.alpha) * (1.0 / d).powf(self.beta));
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
    pub struct AcoReplacement {
        pub evaporation: f64,
        pub heuristic_length: f64,
    }
    impl Replacement for AcoReplacement {
        fn replace(
            &self,
            state: &mut State,
            _rng: &mut Random,
            population: &mut Vec<Individual>,
            offspring: &mut Vec<Individual>,
        ) {
            let pm = state.custom.get_mut::<PheromoneMatrix>();

            // Update pheromones for probabilistic routes
            for individual in offspring.iter().skip(1) {
                let fitness = individual.fitness().into();
                let route = individual.solution::<Route>();
                let delta = self.heuristic_length / fitness;
                for (&a, &b) in route.iter().zip(route.iter().skip(1)) {
                    pm[a][b] += delta;
                    pm[b][a] += delta;
                }
            }

            population.clear();
            population.extend(offspring.drain(..));
        }
    }
}
