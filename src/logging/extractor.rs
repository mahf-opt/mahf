use dyn_clone::DynClone;
use serde::Serialize;

use crate::{logging::log::Entry, state::lens::Lens, Problem, State};

pub trait EntryName {
    fn entry_name() -> &'static str;
}

/// Extracts some state and turns it into an [Entry].
pub trait EntryExtractor<P>: DynClone + Send {
    fn extract_entry(&self, state: &State<P>) -> Entry;
}

dyn_clone::clone_trait_object!(<P> EntryExtractor<P>);

impl<P, T> EntryExtractor<P> for T
where
    P: Problem,
    T: Lens<P> + EntryName + Clone + Send,
    T::Target: Serialize + Send + 'static,
{
    fn extract_entry(&self, state: &State<P>) -> Entry {
        Entry {
            name: T::entry_name(),
            value: Box::new(self.get(state).ok()),
        }
    }
}
