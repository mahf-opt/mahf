//! Scheduling methods

use crate::heuristic::components::Scheduler;

/// Schedules all operators once and in order.
#[derive(serde::Serialize)]
pub struct AllInOrder;
impl Scheduler for AllInOrder {
    fn schedule(
        &self,
        _state: &mut crate::heuristic::State,
        _rng: &mut crate::random::Random,
        choices: usize,
        _population: &[crate::heuristic::Individual],
        _parents: &[&crate::heuristic::Individual],
        schedule: &mut Vec<usize>,
    ) {
        schedule.extend((0..choices).into_iter())
    }
}
