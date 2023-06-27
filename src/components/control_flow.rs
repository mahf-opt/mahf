//! Meta-components for specifying control flow.

use derivative::Derivative;
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::Component,
    conditions::Condition,
    problems::Problem,
    state::{common, State, StateReq},
};

/// A block of components executed sequentially.
///
/// This is equivalent to sequential statements:
/// ```no_run
/// # fn statement1() {}
/// # fn statement2() {}
/// # fn statement3() {}
///
/// statement1();
/// statement2();
/// statement3();
/// ```
///
/// # Call propagation
///
/// Calling any of the `{init, require, execute}` methods on a block calls the specific
/// method sequentially in order on the inner components.
///
/// # Examples
///
/// A `Block` is usually only created implicitly by calling the [`do_`] method on [`Configuration::builder`]:
///
/// [`do_`]: crate::configuration::ConfigurationBuilder::do_
/// [`Configuration::builder`]: crate::Configuration::builder
///
/// ```no_run
/// # use mahf::Problem;
/// # fn component1<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component2<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component3<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// use mahf::Configuration;
/// # pub fn example<P: Problem>() -> Configuration<P> {
/// Configuration::builder()
///     .do_(component1())
///     .do_(component2())
///     .do_(component3())
///     .build()
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[serde(transparent)]
#[derivative(Clone(bound = ""))]
pub struct Block<P: Problem>(Vec<Box<dyn Component<P>>>);

impl<P: Problem> Block<P> {
    /// Creates a new `Block` from a collection of [`Component`]s.
    pub fn new(
        components: impl IntoIterator<Item = Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Self(components.into_iter().collect()))
    }
}

impl<P: Problem> Component<P> for Block<P> {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        for component in &self.0 {
            component.init(problem, state)?;
        }
        Ok(())
    }

    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        for component in &self.0 {
            component.require(problem, state_req)?;
        }
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        for component in &self.0 {
            component.execute(problem, state)?;
        }
        Ok(())
    }
}

impl<I: IntoIterator<Item = Box<dyn Component<P>>>, P: Problem> From<I> for Box<dyn Component<P>> {
    fn from(value: I) -> Self {
        Block::new(value)
    }
}

/// Loops the `body` while a `condition` is `true`.
///
/// This is equivalent to a `while` loop:
/// ```no_run
/// # fn condition() -> bool { true }
/// # fn body() {}
/// while condition() {
///     body()
/// }
/// ```
///
/// # State
///
/// This component inserts and updates the current number of [`Iterations`].
///
/// Note that a [`Scope`] is needed for nested loops, so that the inner `Loop` doesn't overwrite
/// the outer amount of [`Iterations`].
///
/// [`Iterations`]: common::Iterations
///
/// # Call propagation
///
/// Calling any of the `{init, require}` methods on a loop calls the specific
/// method once on the inner component and condition.
///
/// On calling the `execute` method, the `condition` is re-initialized, after which the
/// `body` is executed until the `condition` evaluates to `false`.
///
/// # Examples
///
/// A `Loop` is usually created by calling the [`while_`] method on [`Configuration::builder`]:
///
/// [`while_`]: crate::configuration::ConfigurationBuilder::while_
/// [`Configuration::builder`]: crate::Configuration::builder
///
/// ```no_run
/// # use mahf::Problem;
/// # fn condition<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
/// # fn component1<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component2<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component3<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// use mahf::Configuration;
/// # pub fn example<P: Problem>() -> Configuration<P> {
/// Configuration::builder()
///     .while_(condition(), |builder| {
///         builder
///             .do_(component1())
///             .do_(component2())
///             .do_(component3())
///     })
///     .build()
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Loop<P: Problem> {
    #[serde(rename = "while")]
    condition: Box<dyn Condition<P>>,
    #[serde(rename = "do")]
    body: Box<dyn Component<P>>,
}

impl<P: Problem> Loop<P> {
    /// Creates a new `Loop` with some `condition` and a body.
    pub fn new(
        condition: Box<dyn Condition<P>>,
        body: impl Into<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Loop {
            condition,
            body: body.into(),
        })
    }
}

impl<P: Problem> Component<P> for Loop<P> {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(common::Iterations(0));

        self.condition.init(problem, state)?;
        self.body.init(problem, state)?;

        Ok(())
    }

    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        self.condition.require(problem, state_req)?;
        self.body.require(problem, state_req)?;
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        self.condition.init(problem, state)?;
        while self.condition.evaluate(problem, state)? {
            self.body.execute(problem, state)?;
            *state.try_borrow_value_mut::<common::Iterations>()? += 1;
        }
        Ok(())
    }
}

/// Executes the `if` or `else` branch depending on the `condition`.
///
/// This is equivalent to an `if` statement with an optional `else`:
/// ```no_run
/// # fn condition() -> bool { true }
/// # fn if_body() {}
/// # fn else_body() {}
///
/// if condition() {
///     if_body();
/// }
///
/// if condition() {
///     if_body();
/// } else {
///     else_body();
/// }
/// ```
///
/// # Call propagation
///
/// Calling any of the `{init, require}` methods on a branch calls the specific
/// method once on the inner components and condition.
///
/// On calling the `execute` method, the `if_body` is executed if the `condition`
/// evaluates to `true`, and the optional `else_body` if executed if present, otherwise.
///
/// # Examples
///
/// A `Branch` is usually created by calling the [`if_`] method on [`Configuration::builder`]:
///
/// [`if_`]: crate::configuration::ConfigurationBuilder::if_
/// [`Configuration::builder`]: crate::Configuration::builder
///
/// ```no_run
/// # use mahf::Problem;
/// # fn condition<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
/// # fn component1<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component2<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component3<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// use mahf::Configuration;
/// # pub fn example<P: Problem>() -> Configuration<P> {
/// Configuration::builder()
///     .if_(condition(), |builder| {
///         builder
///             .do_(component1())
///             .do_(component2())
///             .do_(component3())
///     })
///     .build()
/// # }
/// ```
///
/// For also creating an `else` branch, use the [`if_else_`] method:
///
/// [`if_else_`]: crate::configuration::ConfigurationBuilder::if_else_
///
/// ```no_run
/// # use mahf::Problem;
/// # fn condition<P: Problem>() -> Box<dyn mahf::Condition<P>> { unimplemented!() }
/// # fn component1<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component2<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component3<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component4<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// use mahf::Configuration;
///
/// # pub fn example<P: Problem>() -> Configuration<P> {
/// Configuration::builder()
///     .if_else_(condition(),
///         |if_builder| {
///             if_builder
///                 .do_(component1())
///                 .do_(component2())
///         },
///         |else_builder| {
///             else_builder
///                 .do_(component3())
///                 .do_(component4())
///     })
///     .build()
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Branch<P: Problem> {
    condition: Box<dyn Condition<P>>,
    if_body: Box<dyn Component<P>>,
    else_body: Option<Box<dyn Component<P>>>,
}

impl<P: Problem> Branch<P> {
    /// Creates a new `Branch` with only an `if` branch:
    ///
    /// ```text
    /// if(condition) { body }
    /// ```
    pub fn new(
        condition: Box<dyn Condition<P>>,
        body: impl Into<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Branch {
            condition,
            if_body: body.into(),
            else_body: None,
        })
    }

    /// Creates a new `Branch` with an `if`-`else` branch:
    ///
    /// ```text
    /// if(condition) { if_body } else { else_body }
    /// ```
    pub fn new_with_else(
        condition: Box<dyn Condition<P>>,
        if_body: impl Into<Box<dyn Component<P>>>,
        else_body: impl Into<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Branch {
            condition,
            if_body: if_body.into(),
            else_body: Some(else_body.into()),
        })
    }
}

impl<P: Problem> Component<P> for Branch<P> {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        self.condition.init(problem, state)?;
        self.if_body.init(problem, state)?;
        if let Some(else_body) = &self.else_body {
            else_body.init(problem, state)?;
        }
        Ok(())
    }

    fn require(&self, problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        self.condition.require(problem, state_req)?;
        self.if_body.require(problem, state_req)?;
        if let Some(else_body) = &self.else_body {
            else_body.require(problem, state_req)?;
        }
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        if self.condition.evaluate(problem, state)? {
            self.if_body.execute(problem, state)?;
        } else if let Some(else_body) = &self.else_body {
            else_body.execute(problem, state)?;
        }
        Ok(())
    }
}

/// Executes the `body` in a new scope, where shadowing custom state is possible.
///
/// This is especially useful for nested heuristics with an inner loop.
///
/// It is equivalent to a scope:
/// ```no_run
/// # fn statement1() {}
/// # fn statement2() {}
/// # fn statement3() {}
/// {
///     statement1();
///     statement2();
///     statement3();
/// }
/// ```
///
/// # Call propagation
///
/// Calling any of the `{init, require}` methods on a scope **doesn't do anything**.
///
/// On calling the `execute` method, the `state` is first initialized using
/// the provided `state_init` function.
/// Afterwards, the `body` is executed as if it was an independent
/// [`Configuration`], e.g. the `{init, require, execute}` methods are called in order
/// in the body.
/// Finally, the provided `states_merge` function is called to merge the original state with
/// the newly created child state.
///
/// [`Configuration`]: crate::Configuration
///
/// # Examples
///
/// A `Scope` is usually only created implicitly by calling the [`scope_`] method on [`Configuration::builder`]:
///
/// [`scope_`]: crate::configuration::ConfigurationBuilder::do_
/// [`Configuration::builder`]: crate::Configuration::builder
///
/// ```no_run
/// # use mahf::Problem;
/// # fn component1<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component2<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// # fn component3<P: Problem>() -> Box<dyn mahf::Component<P>> { unimplemented!() }
/// use mahf::Configuration;
///
/// # pub fn example<P: Problem>() -> Configuration<P> {
/// Configuration::builder()
///     .scope_(|builder| {
///         builder
///             .do_(component1())
///             .do_(component2())
///             .do_(component3())
///     })
///     .build()
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Scope<P: Problem> {
    body: Box<dyn Component<P>>,
    #[serde(skip)]
    state_init: fn(&mut State<P>) -> ExecResult<()>,
    #[serde(skip)]
    states_merge: fn(&mut State<P>, State<P>) -> ExecResult<()>,
}

impl<P: Problem> Scope<P> {
    /// Creates a new `Scope` with the `body`.
    pub fn new(body: Vec<Box<dyn Component<P>>>) -> Box<dyn Component<P>> {
        Self::new_with(|_| Ok(()), body, |_, _| Ok(()))
    }

    /// Creates a new `Scope` with the `body`, and methods for initializing the [`State`] beforehand
    /// and merging the parent and child [`State`] afterwards.
    pub fn new_with(
        state_init: fn(&mut State<P>) -> ExecResult<()>,
        body: impl Into<Box<dyn Component<P>>>,
        states_merge: fn(&mut State<P>, State<P>) -> ExecResult<()>,
    ) -> Box<dyn Component<P>> {
        Box::new(Scope {
            body: body.into(),
            state_init,
            states_merge,
        })
    }
}

impl<P: Problem> Component<P> for Scope<P> {
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let inner = state.with_inner_state(|state| {
            (self.state_init)(state)?;
            self.body.init(problem, state)?;
            self.body.require(problem, &state.requirements())?;
            self.body.execute(problem, state)?;
            Ok(())
        })?;
        (self.states_merge)(state, inner)?;
        Ok(())
    }
}
