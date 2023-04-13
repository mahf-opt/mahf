use derivative::Derivative;
use serde::Serialize;

use crate::{
    components::Component,
    conditions::Condition,
    problems::Problem,
    state::{common, State},
};

/// A sequential block of components.
/// The child components are initialized and executed sequentially.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[serde(transparent)]
#[derivative(Clone(bound = ""))]
pub struct Block<P: Problem>(Vec<Box<dyn Component<P>>>);

impl<P: Problem> Block<P> {
    /// Creates a new [Block] from a collection of [Component]s.
    pub fn new(
        components: impl IntoIterator<Item = Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Block(components.into_iter().collect()))
    }
}

impl<P: Problem> Component<P> for Block<P> {
    fn initialize(&self, problem: &P, state: &mut State<P>) {
        for component in &self.0 {
            component.initialize(problem, state);
        }
    }

    fn execute(&self, problem: &P, state: &mut State<P>) {
        for component in &self.0 {
            component.execute(problem, state);
        }
    }
}

impl<I: IntoIterator<Item = Box<dyn Component<P>>>, P: Problem> From<I> for Box<dyn Component<P>> {
    fn from(value: I) -> Self {
        Block::new(value)
    }
}

/// Executes the child component as long as the condition is true.
///
/// This corresponds to a `while` loop.
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
    /// Creates a new [Loop].
    ///
    /// Accepts a single or multiple components through the [Into] implementation of multiple components.
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
    fn initialize(&self, problem: &P, state: &mut State<P>) {
        state.insert(common::Iterations(0));

        self.condition.initialize(problem, state);
        self.body.initialize(problem, state);
    }

    fn execute(&self, problem: &P, state: &mut State<P>) {
        self.condition.initialize(problem, state);
        while self.condition.evaluate(problem, state) {
            self.body.execute(problem, state);
            *state.get_value_mut::<common::Iterations>() += 1;
        }
    }
}

/// Executes either the `if` or `else` branch depending on a condition.
///
/// Both `if` and `if`-`else` constructs are possible.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Branch<P: Problem> {
    condition: Box<dyn Condition<P>>,
    if_body: Box<dyn Component<P>>,
    else_body: Option<Box<dyn Component<P>>>,
}

impl<P: Problem> Branch<P> {
    /// Creates a new [Branch] component.
    ///
    /// This corresponds to an `if` construct:
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

    /// Creates a new [Branch] component.
    ///
    /// This corresponds to an `if`-`else` construct:
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
    fn initialize(&self, problem: &P, state: &mut State<P>) {
        self.condition.initialize(problem, state);
        self.if_body.initialize(problem, state);
        if let Some(else_body) = &self.else_body {
            else_body.initialize(problem, state);
        }
    }

    fn execute(&self, problem: &P, state: &mut State<P>) {
        if self.condition.evaluate(problem, state) {
            self.if_body.execute(problem, state);
        } else if let Some(else_body) = &self.else_body {
            else_body.execute(problem, state);
        }
    }
}

/// Encapsulates state of child components.
///
/// When child components add new state,
/// that state will be lost after the execution
/// of this components.
///
/// All children will be re-initialized on every call
/// of the `execute` function.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Scope<P: Problem> {
    body: Box<dyn Component<P>>,
    #[serde(skip)]
    init: fn(&mut State<P>),
}

impl<P: Problem> Scope<P> {
    /// Creates a new [Scope].
    ///
    /// If you want to override some state,
    /// you can use [Scope::new_with].
    pub fn new(body: Vec<Box<dyn Component<P>>>) -> Box<dyn Component<P>> {
        Self::new_with(|_| {}, body)
    }

    /// Creates a new [Scope] while overriding some state.
    pub fn new_with(
        init: fn(&mut State<P>),
        body: impl Into<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        Box::new(Scope {
            body: body.into(),
            init,
        })
    }
}

impl<P: Problem> Component<P> for Scope<P> {
    fn execute(&self, problem: &P, state: &mut State<P>) {
        state.with_substate(|state| {
            (self.init)(state);
            self.body.initialize(problem, state);
            self.body.execute(problem, state);
        });
    }
}
