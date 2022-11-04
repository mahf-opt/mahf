//! The Component trait and structural components.

use crate::{
    framework::conditions::Condition,
    problems::Problem,
    state::{common, State},
};
use derivative::Derivative;
use dyn_clone::DynClone;
use serde::Serialize;
use std::any::Any;
use trait_set::trait_set;

trait_set! {
    /// Collection of traits required by every component.
    pub trait AnyComponent = erased_serde::Serialize + Any + Send + Sync + DynClone;
}

/// Describes a component for use in the MAHF framework.
///
/// `initialize` can be used to check what state is available
/// or add custom state required by the component.
///
/// `execute` should contain the actual logic of the component.
///
/// Components are immutable, their properties describe them and can not
/// change during a run. All mutable state has to be stored in the [State].
pub trait Component<P: Problem>: AnyComponent {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State) {}
    fn execute(&self, problem: &P, state: &mut State);
}
erased_serde::serialize_trait_object!(<P: Problem> Component<P>);
dyn_clone::clone_trait_object!(<P: Problem> Component<P>);

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
    init: fn(&mut State),
}

impl<P: Problem> Component<P> for Scope<P> {
    fn execute(&self, problem: &P, state: &mut State) {
        state.with_substate(|state| {
            (self.init)(state);
            self.body.initialize(problem, state);
            self.body.execute(problem, state);
        });
    }
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
        init: fn(&mut State),
        body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let body = Block::new(body);
        Box::new(Scope { body, init })
    }
}

/// A sequential block of components.
///
/// Will execute child components sequentially.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[serde(transparent)]
#[derivative(Clone(bound = ""))]
pub struct Block<P: Problem>(Vec<Box<dyn Component<P>>>);

impl<P: Problem> Component<P> for Block<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        for component in &self.0 {
            component.initialize(problem, state);
        }
    }

    fn execute(&self, problem: &P, state: &mut State) {
        for component in &self.0 {
            component.execute(problem, state);
        }
    }
}

impl<P: Problem> Block<P> {
    pub fn new(components: Vec<Box<dyn Component<P>>>) -> Box<dyn Component<P>> {
        Box::new(Block(components))
    }
}

/// Executes its child as long as the condition is true.
///
/// Use a [Block] as body if you want to loop over
/// multiple components sequentially.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Loop<P: Problem> {
    #[serde(rename = "while")]
    condition: Box<dyn Condition<P>>,
    #[serde(rename = "do")]
    body: Box<dyn Component<P>>,
}

impl<P: Problem> Component<P> for Loop<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        state.insert(common::Iterations(0));

        self.condition.initialize(problem, state);
        self.body.initialize(problem, state);
    }

    fn execute(&self, problem: &P, state: &mut State) {
        self.condition.initialize(problem, state);
        while self.condition.evaluate(problem, state) {
            self.body.execute(problem, state);
            *state.get_value_mut::<common::Iterations>() += 1;
        }
    }
}

impl<P: Problem> Loop<P> {
    pub fn new(
        condition: Box<dyn Condition<P>>,
        body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let body = Block::new(body);
        Box::new(Loop { condition, body })
    }
}

/// An if-else branching component.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Branch<P: Problem> {
    condition: Box<dyn Condition<P>>,
    if_body: Box<dyn Component<P>>,
    else_body: Option<Box<dyn Component<P>>>,
}

impl<P: Problem> Component<P> for Branch<P> {
    fn initialize(&self, problem: &P, state: &mut State) {
        self.condition.initialize(problem, state);
        self.if_body.initialize(problem, state);
        if let Some(else_body) = &self.else_body {
            else_body.initialize(problem, state);
        }
    }

    fn execute(&self, problem: &P, state: &mut State) {
        if self.condition.evaluate(problem, state) {
            self.if_body.execute(problem, state);
        } else if let Some(else_body) = &self.else_body {
            else_body.execute(problem, state);
        }
    }
}

impl<P: Problem> Branch<P> {
    pub fn new(
        condition: Box<dyn Condition<P>>,
        body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let if_body = Block::new(body);
        Box::new(Branch {
            condition,
            if_body,
            else_body: None,
        })
    }

    pub fn new_with_else(
        condition: Box<dyn Condition<P>>,
        if_body: Vec<Box<dyn Component<P>>>,
        else_body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let if_body = Block::new(if_body);
        let else_body = Some(Block::new(else_body));
        Box::new(Branch {
            condition,
            if_body,
            else_body,
        })
    }
}
