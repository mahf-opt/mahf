//! Traits for extracting data from the [`State`] and logging it.

use dyn_clone::DynClone;
use serde::Serialize;

use crate::{lens::Lens, logging::log::Entry, Problem, State};

/// Trait to specify a name for the [`Entry`].
///
/// This is the string that will be used as key in the serialized representation.
pub trait EntryName {
    /// Returns the name of the value stored within [`Entry`].
    fn entry_name() -> &'static str;
}

/// Trait to extract some state and convert it into an [`Entry`].
///
/// Used inside a [`LogConfig`].
///
/// This trait is auto-implemented for types that implement [`Lens`], which is preferred
/// over implementing this trait directly.
///
/// [`LogConfig`]: crate::logging::LogConfig
pub trait EntryExtractor<P>: DynClone + Send {
    /// Extracts some state and converts it into an [`Entry`].
    fn extract_entry(&self, problem: &P, state: &State<P>) -> Entry;
}

dyn_clone::clone_trait_object!(<P> EntryExtractor<P>);

impl<P, T> EntryExtractor<P> for T
where
    P: Problem,
    T: Lens<P> + EntryName + Clone + Send,
    T::Target: Serialize + Send + 'static,
{
    fn extract_entry(&self, problem: &P, state: &State<P>) -> Entry {
        Entry {
            name: T::entry_name(),
            value: Box::new(self.get(problem, state).ok()),
        }
    }
}
