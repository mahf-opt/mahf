use std::ops;

use derivative::Derivative;
use serde::Serialize;

use crate::{component::ExecResult, conditions::Condition, state::StateReq, Problem, State};

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct And<P: Problem>(Vec<Box<dyn Condition<P>>>);

impl<P: Problem> And<P> {
    pub fn new(
        conditions: impl IntoIterator<Item = Box<dyn Condition<P>>>,
    ) -> Box<dyn Condition<P>> {
        Box::new(Self(conditions.into_iter().collect()))
    }
}

impl<P: Problem> Condition<P> for And<P> {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        for condition in self.0.iter() {
            condition.init(problem, state)?;
        }
        Ok(())
    }

    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        for condition in &self.0 {
            condition.require(problem, state_req)?;
        }
        Ok(())
    }

    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let evaluations: Result<Vec<_>, _> = self
            .0
            .iter()
            .map(|condition| condition.evaluate(problem, state))
            .collect();
        Ok(evaluations?.into_iter().all(|x| x))
    }
}

impl<P: Problem> ops::BitAnd for Box<dyn Condition<P>> {
    type Output = Box<dyn Condition<P>>;

    fn bitand(self, rhs: Self) -> Self::Output {
        And::new([self, rhs])
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Or<P: Problem>(Vec<Box<dyn Condition<P>>>);

impl<P: Problem> Or<P> {
    pub fn new(
        conditions: impl IntoIterator<Item = Box<dyn Condition<P>>>,
    ) -> Box<dyn Condition<P>> {
        Box::new(Self(conditions.into_iter().collect()))
    }
}

impl<P: Problem> Condition<P> for Or<P> {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        for condition in self.0.iter() {
            condition.init(problem, state)?;
        }
        Ok(())
    }

    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        for condition in &self.0 {
            condition.require(problem, state_req)?;
        }
        Ok(())
    }

    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let evaluations: Result<Vec<_>, _> = self
            .0
            .iter()
            .map(|condition| condition.evaluate(problem, state))
            .collect();
        Ok(evaluations?.into_iter().any(|x| x))
    }
}

impl<P: Problem> ops::BitOr for Box<dyn Condition<P>> {
    type Output = Box<dyn Condition<P>>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Or::new([self, rhs])
    }
}

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
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        self.0.init(problem, state)?;
        Ok(())
    }

    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        self.0.require(problem, state_req)?;
        Ok(())
    }

    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        Ok(!self.0.evaluate(problem, state)?)
    }
}

impl<P: Problem> ops::Not for Box<dyn Condition<P>> {
    type Output = Box<dyn Condition<P>>;

    fn not(self) -> Self::Output {
        Not::new(self)
    }
}
