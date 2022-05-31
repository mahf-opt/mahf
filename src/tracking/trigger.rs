use crate::framework::{components::Condition, State};

#[derive(serde::Serialize)]
pub struct OnNthIteration(u32);

impl OnNthIteration {
    pub fn new<P>(iterations: u32) -> Box<dyn Condition<P>> {
        Box::new(OnNthIteration(iterations))
    }
}

impl<P> Condition<P> for OnNthIteration {
    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        state.iterations() % self.0 == 0
    }
}

pub struct OnImprovement {}
