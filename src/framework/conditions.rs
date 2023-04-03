use derivative::Derivative;
use serde::Serialize;

/// The Condition trait and combinators.
use crate::{framework::components::AnyComponent, problems::Problem, state::State};

/// A condition for loops or branches.
///
/// Similar to [Component](crate::framework::components::Component),
/// but `evaluate` replaces `execute` and returns a `bool`.
///
/// These can be combined using binary AND and OR (`|` and `&`).
pub trait Condition<P: Problem>: AnyComponent {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State<P>) {}
    fn evaluate(&self, problem: &P, state: &mut State<P>) -> bool;
}
erased_serde::serialize_trait_object!(<P: Problem> Condition<P>);
dyn_clone::clone_trait_object!(<P: Problem> Condition<P>);

/// Multiple conditions must be true.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct And<P: Problem>(Vec<Box<dyn Condition<P>>>);
impl<P: Problem> And<P> {
    pub fn new(conditions: Vec<Box<dyn Condition<P>>>) -> Box<dyn Condition<P>> {
        Box::new(Self(conditions))
    }
}
impl<P: Problem> Condition<P> for And<P> {
    fn initialize(&self, problem: &P, state: &mut State<P>) {
        for condition in self.0.iter() {
            condition.initialize(problem, state);
        }
    }

    fn evaluate(&self, problem: &P, state: &mut State<P>) -> bool {
        self.0
            .iter()
            .all(|condition| condition.evaluate(problem, state))
    }
}
impl<P: Problem> std::ops::BitAnd for Box<dyn Condition<P>> {
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
impl<P: Problem> Or<P> {
    pub fn new(conditions: Vec<Box<dyn Condition<P>>>) -> Box<dyn Condition<P>> {
        Box::new(Self(conditions))
    }
}
impl<P: Problem> Condition<P> for Or<P> {
    fn initialize(&self, problem: &P, state: &mut State<P>) {
        for condition in self.0.iter() {
            condition.initialize(problem, state);
        }
    }

    fn evaluate(&self, problem: &P, state: &mut State<P>) -> bool {
        self.0
            .iter()
            .any(|condition| condition.evaluate(problem, state))
    }
}
impl<P: Problem> std::ops::BitOr for Box<dyn Condition<P>> {
    type Output = Box<dyn Condition<P>>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Or::new(vec![self, rhs])
    }
}

/// Inverses the condition.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Not<P: Problem>(Box<dyn Condition<P>>);
impl<P: Problem> Not<P> {
    pub fn new(condition: Box<dyn Condition<P>>) -> Box<dyn Condition<P>> {
        Box::new(Self(condition))
    }
}
impl<P: Problem> Condition<P> for Not<P> {
    fn initialize(&self, problem: &P, state: &mut State<P>) {
        self.0.initialize(problem, state);
    }

    fn evaluate(&self, problem: &P, state: &mut State<P>) -> bool {
        !self.0.evaluate(problem, state)
    }
}
impl<P: Problem> std::ops::Not for Box<dyn Condition<P>> {
    type Output = Box<dyn Condition<P>>;

    fn not(self) -> Self::Output {
        Not::new(self)
    }
}
