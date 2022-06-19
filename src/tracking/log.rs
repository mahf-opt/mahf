use crate::framework::CustomState;
use erased_serde::Serialize as DynSerialize;
use serde::Serialize;

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
            self.entries.iter().find(|e| e.name == entry.name).is_none(),
            "entry with name {} already exists",
            entry.name
        );

        self.entries.push(entry);
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
