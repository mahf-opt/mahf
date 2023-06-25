//! State requirements.

use crate::{state::StateResult, CustomState, State, StateError};

/// Helper struct to check if specific custom state is present in the state.
///
/// This `struct` is constructed from the [`requirements`] method on [`State`].
///
/// [`requirements`]: State::requirements
pub struct StateReq<'a, 'b, P>(&'a State<'b, P>);

impl<'a, 'b, P> StateReq<'a, 'b, P> {
    pub fn new(state: &'a State<'b, P>) -> Self {
        Self(state)
    }

    /// Checks whether `T`, which is required by `Source`, is present in the state.
    ///
    /// This method can be called in [`Component::require`] to ensure that all required
    /// state is available.
    ///
    /// [`Component::require`]: crate::Component::require
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use better_any::{Tid, TidAble};
    /// use serde::Serialize;
    /// use mahf::prelude::*;
    /// # #[derive(Tid)]
    /// # struct RequiredCustomState;
    /// # impl CustomState<'_> for RequiredCustomState {}
    ///
    /// #[derive(Clone, Serialize)]
    /// struct ExampleComponent;
    ///
    /// impl<P: Problem> Component<P> for ExampleComponent {
    ///     fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
    ///         // Returns an error if `RequiredCustomState` is not present.
    ///         state_req.require::<Self, RequiredCustomState>()?;
    ///         Ok(())
    ///     }
    ///
    ///     /* ... */
    /// #    fn execute(&self, problem: &P, state: &mut State<P>)-> ExecResult<()> {
    /// #        unimplemented!()
    /// #    }
    /// }
    /// ```
    pub fn require<Source, T>(&self) -> StateResult<()>
    where
        T: CustomState<'b>,
    {
        self.0
            .has::<T>()
            .then_some(())
            .ok_or_else(StateError::required_missing::<Source, T>)
    }
}
