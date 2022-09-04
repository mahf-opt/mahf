//! Custom state types
//!
//! A collection of custom state types required by specific metaheuristics and evaluation procedures.

use crate::{
    framework::{state::CustomState, Individual, SingleObjective},
    problems::{Problem, SingleObjectiveProblem},
};
use serde::Serialize;

// Custom States for Specific Metaheuristics //

/// State required for PSO.
///
/// For preserving velocities of particles, own best values and global best particle.
pub struct PsoState<P: Problem> {
    pub velocities: Vec<Vec<f64>>,
    pub bests: Vec<Individual<P>>,
    pub global_best: Individual<P>,
}
impl<P: Problem> CustomState for PsoState<P> {}

#[derive(Clone, Serialize)]
pub struct PheromoneMatrix {
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
impl CustomState for PheromoneMatrix {}

// Custom States for Operators //

/// State required for Elitism.
///
/// For preserving n elitist individuals.
pub struct ElitistArchiveState<P: SingleObjectiveProblem> {
    elitists: Vec<Individual<P>>,
    n_elitists: usize,
}
impl<P: SingleObjectiveProblem> CustomState for ElitistArchiveState<P> {}

impl<P: SingleObjectiveProblem> ElitistArchiveState<P> {
    pub fn new(n_elitists: usize) -> Self {
        Self {
            elitists: Vec::new(),
            n_elitists,
        }
    }

    pub fn update(&mut self, population: &[Individual<P>]) {
        let mut pop = population.iter().collect::<Vec<_>>();
        pop.sort_unstable_by_key(|i| i.objective());
        pop.truncate(self.n_elitists);
        self.elitists = pop.into_iter().cloned().collect();
    }

    pub fn elitists(&self) -> &[Individual<P>] {
        &self.elitists
    }

    pub fn elitists_mut(&mut self) -> &mut [Individual<P>] {
        &mut self.elitists
    }
}

/// State required for Termination by Steps without Improvement.
///
/// For preserving current number of steps without improvement and corresponding fitness value.
pub struct FitnessImprovementState {
    pub current_steps: usize,
    pub current_objective: SingleObjective,
}
impl FitnessImprovementState {
    pub fn update(&mut self, objective: &SingleObjective) -> bool {
        if objective >= &self.current_objective {
            self.current_steps += 1;
            false
        } else {
            self.current_objective = *objective;
            self.current_steps = 0;
            true
        }
    }
}
impl CustomState for FitnessImprovementState {}

// Custom States for Metrics and Logging //

/// State for logging/tracking population diversity
pub struct DiversityState {
    pub diversity: f64,
    pub max_div: f64,
}
impl CustomState for DiversityState {}

/// State for logging current population
pub struct PopulationState {
    pub current_pop: Vec<Vec<f64>>,
}
impl CustomState for PopulationState {}
