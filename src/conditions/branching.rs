//! Branching conditions

use crate::{framework::conditions::Condition, problems::Problem, state::State};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RandomChance {
    // Probability of the condition evaluating to `true`.
    p: f64,
}
impl RandomChance {
    pub fn new<P>(p: f64) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self { p })
    }
}
impl<P> Condition<P> for RandomChance
where
    P: Problem,
{
    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        state.random_mut().gen_bool(self.p)
    }
}
