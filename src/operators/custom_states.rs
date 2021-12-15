//! Custom states required for specific metaheuristics and evaluation procedures

use crate::framework::{CustomState, Individual};
use crate::tracking::log::CustomLog;

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
impl CustomState for DiversityState {
    fn iteration_log(&self) -> Vec<CustomLog> {
        vec![
            CustomLog {
                name: "diversity",
                value: Some(self.diversity),
                solutions: None,
            }]
    }
}

/// State for logging current population
pub struct PopulationState {
    pub current_pop: Vec<Vec<f64>>,
}
impl CustomState for PopulationState {
    fn iteration_log(&self) -> Vec<CustomLog> {
        vec![
            CustomLog {
                name: "population",
                value: None,
                solutions: Some(self.current_pop.clone()),
            }]
    }
}
