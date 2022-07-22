use crate::{
    framework::{components::AnyComponent, state::State},
    problems::Problem,
};
use serde::Serialize;

pub trait Condition<P>: AnyComponent {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State) {}
    fn evaluate(&self, problem: &P, state: &mut State) -> bool;
}
erased_serde::serialize_trait_object!(<P: Problem> Condition<P>);

#[derive(Serialize)]
#[serde(bound = "")]
pub struct And<P: Problem>(Vec<Box<dyn Condition<P>>>);
impl<P: Problem + 'static> And<P> {
    pub fn new(conditions: Vec<Box<dyn Condition<P>>>) -> Box<dyn Condition<P>> {
        Box::new(Self(conditions))
    }
}
impl<P: Problem + 'static> Condition<P> for And<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        for condition in self.0.iter() {
            condition.initialize(problem, state);
        }
    }

    fn evaluate(&self, problem: &P, state: &mut State) -> bool {
        self.0
            .iter()
            .all(|condition| condition.evaluate(problem, state))
    }
}
impl<P: Problem + 'static> std::ops::BitAnd for Box<dyn Condition<P>> {
    type Output = Box<dyn Condition<P>>;

    fn bitand(self, rhs: Self) -> Self::Output {
        And::new(vec![self, rhs])
    }
}

#[derive(Serialize)]
#[serde(bound = "")]
pub struct Or<P: Problem>(Vec<Box<dyn Condition<P>>>);
impl<P: Problem + 'static> Or<P> {
    pub fn new(conditions: Vec<Box<dyn Condition<P>>>) -> Box<dyn Condition<P>> {
        Box::new(Self(conditions))
    }
}
impl<P: Problem + 'static> Condition<P> for Or<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        for condition in self.0.iter() {
            condition.initialize(problem, state);
        }
    }

    fn evaluate(&self, problem: &P, state: &mut State) -> bool {
        self.0
            .iter()
            .any(|condition| condition.evaluate(problem, state))
    }
}
impl<P: Problem + 'static> std::ops::BitOr for Box<dyn Condition<P>> {
    type Output = Box<dyn Condition<P>>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Or::new(vec![self, rhs])
    }
}
