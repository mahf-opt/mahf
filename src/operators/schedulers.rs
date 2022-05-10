//! Scheduling methods

use crate::{
    framework::{components::*, legacy::components::*, Individual, State},
    problems::Problem,
    random::Random,
};

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
        _state: &mut State,
        _rng: &mut Random,
        choices: usize,
        _population: &[Individual],
        _parents: &[&Individual],
        schedule: &mut Vec<usize>,
    ) {
        schedule.extend((0..choices).into_iter())
    }
}
