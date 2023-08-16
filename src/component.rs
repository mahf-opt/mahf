//! Base utilities for `components`, `conditions`, and `lens`es.

use trait_set::trait_set;

use crate::params::Parametrized;

trait_set! {
    /// Collection of traits required by every component.
    ///
    /// # Requirements
    ///
    /// The [`dyn_clone::DynClone`] and [`erased_serde::Serialize`] traits are automatically
    /// implemented for [`Component`]s and [`Condition`]s that implement [`Clone`] and
    /// [`serde::Serialize`].
    ///
    /// [`Component`]: crate::components::Component
    /// [`Condition`]: crate::conditions::Condition
    pub trait ComponentLike = dyn_clone::DynClone + erased_serde::Serialize + Parametrized + Send + Sync;
}

/// The result type for fallible execution.
///
/// Note that methods returning this type are considered 'application code', and the caller
/// is expected to only care about if it is an error or not, and not about handling different
/// errors in a different way.
/// This is also the reason an [`eyre::Result`] is used instead of a custom error type.
pub type ExecResult<T> = eyre::Result<T>;
