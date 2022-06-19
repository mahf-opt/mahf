use crate::framework::{common_state, CustomState, State};
use erased_serde::Serialize as DynSerialize;
use serde::Serialize;
use std::any::type_name;

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct Log {
    steps: Vec<Step>,
}

impl CustomState for Log {}

impl Log {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, entry: Step) {
        self.steps.push(entry);
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct Step {
    entries: Vec<Entry>,
}

impl Step {
    pub fn push(&mut self, entry: Entry) {
        debug_assert!(
            !self.entries.iter().any(|e| e.name == entry.name),
            "entry with name {} already exists",
            entry.name
        );

        self.entries.push(entry);
    }

    pub fn push_iteration(&mut self, state: &State) {
        let name = type_name::<common_state::Iterations>();
        let value = Box::new(state.iterations());

        self.push(Entry { name, value });
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
}

#[derive(Serialize)]
pub struct Entry {
    pub name: &'static str,
    pub value: Box<dyn DynSerialize>,
}
