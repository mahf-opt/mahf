use std::ops::{Deref, Sub};

use crate::{
    framework::{common_state, components::Component, CustomState, State},
    problems::Problem,
    tracking::{log::Step, set::LogSet, trigger, Log},
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(bound = "")]
pub struct Logger<P: Problem> {
    #[serde(skip)]
    sets: Vec<LogSet<P>>,
}

impl<P: Problem + 'static> Logger<P> {
    pub fn builder() -> Self {
        Logger { sets: Vec::new() }
    }

    pub fn log_on_change<S>(self, delta: S::Target) -> Self
    where
        S: CustomState + Clone + Serialize + Deref,
        S::Target: Clone + Sub<Output = S::Target> + Ord + Send + Sync + 'static,
    {
        self.log_set(
            LogSet::new()
                .with_trigger(trigger::Change::<S>::new(delta))
                .with_auto_logger::<S>(),
        )
    }

    pub fn log_set(mut self, set: LogSet<P>) -> Self {
        self.sets.push(set);
        self
    }

    pub fn log_common_sets(self) -> Self {
        self.log_set(
            LogSet::new()
                .with_trigger(trigger::Iteration::new(10))
                .with_auto_logger::<common_state::Evaluations>()
                .with_auto_logger::<common_state::BestFitness>()
                .with_auto_logger::<common_state::Progress>(),
        )
    }

    pub fn build(self) -> Box<dyn Component<P>> {
        Box::new(self)
    }

    pub fn default() -> Box<dyn Component<P>> {
        Logger::builder().build()
    }
}

impl<P: Problem + 'static> Component<P> for Logger<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        for set in &self.sets {
            for criteria in &set.criteria {
                criteria.initialize(problem, state);
            }
        }
    }

    fn execute(&self, problem: &P, state: &mut State) {
        let mut step = Step::default();

        for set in &self.sets {
            set.execute(problem, state, &mut step);
        }

        if !step.entries().is_empty() {
            step.push_iteration(state);
            state.get_mut::<Log>().push(step);
        }
    }
}