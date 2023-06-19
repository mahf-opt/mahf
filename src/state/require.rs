use crate::{state::StateResult, CustomState, State, StateError};

pub struct StateReq<'a, 'b, P>(&'a State<'b, P>);

impl<'a, 'b, P> StateReq<'a, 'b, P> {
    pub fn new(state: &'a State<'b, P>) -> Self {
        Self(state)
    }

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
