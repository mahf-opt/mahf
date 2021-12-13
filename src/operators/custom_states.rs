//! Custom states required for specific metaheuristics

use crate::framework::{CustomState, Individual};

/// State required for PSO.
///
/// For preserving velocities of particles, own best values and global best particle.
pub struct PsoState {
    pub(crate) velocities: Vec<Vec<f64>>,
    pub(crate) bests: Vec<Individual>,
    pub(crate) global_best: Individual,
}
impl CustomState for PsoState {}
