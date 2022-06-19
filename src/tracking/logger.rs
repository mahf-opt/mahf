use crate::{
    framework::{components::Component, State},
    problems::Problem,
    tracking::{log::LogEntry, logfn::LogSet, Log},
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

    pub fn with_set(mut self, set: LogSet<P>) -> Self {
        self.sets.push(set);
        self
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
        let mut entry = LogEntry::default();

        for set in &self.sets {
            set.execute(problem, state, &mut entry);
        }

        if !entry.state.is_empty() {
            state.get_mut::<Log>().push(entry);
        }
    }
}
