//! Custom states required for specific metaheuristics and evaluation procedures

use crate::framework::{CustomState, Individual};

// Custom States for Metaheuristics //

/// State required for PSO.
///
/// For preserving velocities of particles, own best values and global best particle.
pub struct PsoState {
    pub velocities: Vec<Vec<f64>>,
    pub bests: Vec<Individual>,
    pub global_best: Individual,
}
impl CustomState for PsoState {}

// Custom States for Metrics and Logging //

/// State for logging/tracking population diversity
pub struct DiversityState {
    pub diversity: f64,
}
impl CustomState for DiversityState {}

/// State for logging current population
pub struct PopulationState {
    pub current_pop: Vec<Individual>,
}
impl CustomState for PopulationState {}
