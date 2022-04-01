//! Scheduling methods

use crate::{framework::components::*, problems::Problem};

/// Schedules all operators once and in order.
#[derive(serde::Serialize)]
pub struct AllInOrder;
impl AllInOrder {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Schedule(Self))
    }
}
impl Scheduler for AllInOrder {
    fn schedule(
        &self,
        _state: &mut crate::framework::State,
        _rng: &mut crate::random::Random,
        choices: usize,
        _population: &[crate::framework::Individual],
        _parents: &[&crate::framework::Individual],
        schedule: &mut Vec<usize>,
    ) {
        schedule.extend((0..choices).into_iter())
    }
}
