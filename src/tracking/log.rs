use crate::framework::CustomState;
use erased_serde::Serialize as DynSerialize;
use serde::Serialize;

#[derive(Default, Serialize)]
#[serde(transparent)]
pub struct Log {
    entries: Vec<LogEntry>,
}

impl CustomState for Log {}

impl Log {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, entry: LogEntry) {
        self.entries.push(entry);
    }
}

#[derive(Default, Serialize)]
#[serde(transparent)]
pub(crate) struct LogEntry {
    pub state: Vec<LoggedState>,
}

#[derive(Serialize)]
pub struct LoggedState {
    pub name: &'static str,
    pub value: Box<dyn DynSerialize>,
}
