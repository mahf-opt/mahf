//! Meta-conditions for combining conditions using boolean operators.

use std::ops;

use derivative::Derivative;
use serde::Serialize;

use crate::{component::ExecResult, conditions::Condition, state::StateReq, Problem, State};

/// Boolean `AND` operator (`&`) for [`Condition`]s.
///
/// # Examples
///
/// Requiring both `condition1` and `condition2` to be `true`:
///
/// ```
/// # use mahf::{Problem, ExecResult};
/// # fn condition1<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
/// # fn condition2<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
/// use mahf::Configuration;
/// use mahf::conditions::OptimumReached;
///
/// # fn example<P: Problem>() -> ExecResult<Configuration<P>> {
/// # Ok(
/// Configuration::builder()
///     .while_(condition1() & condition2(), |builder| {
///         /* ... */
///         # builder
///     })
///     .build()
/// # )
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct And<P: Problem>(Vec<Box<dyn Condition<P>>>);

impl<P: Problem> And<P> {
    /// Constructs a new `And` using the provided `conditions`.
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

/// Boolean `OR` operator (`|`) for [`Condition`]s.
///
/// # Examples
///
/// Requiring `condition1` or `condition2` to be `true`:
///
/// ```
/// # use mahf::{Problem, ExecResult};
/// # fn condition1<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
/// # fn condition2<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
/// use mahf::Configuration;
/// use mahf::conditions::OptimumReached;
///
/// # fn example<P: Problem>() -> ExecResult<Configuration<P>> {
/// # Ok(
/// Configuration::builder()
///     .while_(condition1() | condition2(), |builder| {
///         /* ... */
///         # builder
///     })
///     .build()
/// # )
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Or<P: Problem>(Vec<Box<dyn Condition<P>>>);

impl<P: Problem> Or<P> {
    /// Constructs a new `Or` using the provided `conditions`.
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

/// Boolean `NOT` operator (`!`) for [`Condition`]s.
///
/// # Examples
///
/// Inverting a `condition`:
///
/// ```
/// # use mahf::{Problem, ExecResult};
/// # fn condition<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
/// use mahf::Configuration;
/// use mahf::conditions::OptimumReached;
///
/// # fn example<P: Problem>() -> ExecResult<Configuration<P>> {
/// # Ok(
/// Configuration::builder()
///     .while_(!condition(), |builder| {
///         /* ... */
///         # builder
///     })
///     .build()
/// # )
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Not<P: Problem>(Box<dyn Condition<P>>);

impl<P: Problem> Not<P> {
    /// Constructs a new `Not` using the provided `condition`.
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
