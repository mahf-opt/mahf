//! Custom states required for specific metaheuristics and evaluation procedures

use crate::framework::{CustomState, Individual, SingleObjective};

// Custom States for Specific Metaheuristics //

/// State required for PSO.
///
/// For preserving velocities of particles, own best values and global best particle.
pub struct PsoState {
    pub velocities: Vec<Vec<f64>>,
    pub bests: Vec<Individual>,
    pub global_best: Individual,
}
impl CustomState for PsoState {}

// Custom States for Operators //

/// State required for Elitism.
///
/// For preserving n elitist individuals.
pub struct ElitistArchiveState {
    elitists: Vec<Individual>,
    n_elitists: usize,
}
impl CustomState for ElitistArchiveState {}

impl ElitistArchiveState {
    pub fn new(n_elitists: usize) -> Self {
        Self {
            elitists: Vec::new(),
            n_elitists,
        }
    }

    pub fn update(&mut self, population: &[Individual]) {
        let mut pop = population.iter().collect::<Vec<_>>();
        pop.sort_unstable_by_key(|i| i.fitness());
        pop.truncate(self.n_elitists);
        self.elitists = pop.into_iter().cloned().collect();
    }

    pub fn elitists(&self) -> &[Individual] {
        &self.elitists
    }

    pub fn elitists_mut(&mut self) -> &mut [Individual] {
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
