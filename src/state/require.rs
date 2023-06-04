use crate::{
    state::{error::StateResult, StateRegistry},
    CustomState, StateError,
};

pub struct StateReq<'a, 'b>(&'a StateRegistry<'b>);

impl<'a, 'b> StateReq<'a, 'b> {
    pub fn new(state: &'a StateRegistry<'b>) -> Self {
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
