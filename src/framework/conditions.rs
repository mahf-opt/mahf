/// The Condition trait and combinators.
use crate::{framework::components::AnyComponent, problems::Problem, state::State};
use derivative::Derivative;
use serde::Serialize;

/// A condition for loops or branches.
///
/// Similar to [Component](crate::framework::components::Component),
/// but `evaluate` replaces `execute` and returns a `bool`.
///
/// These can be combined using binary AND and OR (`|` and `&`).
pub trait Condition<P>: AnyComponent {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State) {}
    fn evaluate(&self, problem: &P, state: &mut State) -> bool;
}
erased_serde::serialize_trait_object!(<P: Problem> Condition<P>);
dyn_clone::clone_trait_object!(<P: Problem> Condition<P>);

/// Multiple conditions must be true.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
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

/// Any condition must be true.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
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
