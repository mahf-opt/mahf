#![allow(unused_variables)]
#![allow(clippy::new_ret_no_self)]

//! Framework components.

use crate::{framework::state::State, problems::Problem};
use std::any::Any;
use trait_set::trait_set;

trait_set! {
    pub trait AnyComponent = Any + Send + Sync;
}

/// Defines the traits required by any component.
///
/// This will be implemented automatically for all structs satisfying the requirements.
///
/// # Any
/// All components must allow downcasting and thus require [Any].
///
/// # Serialize
/// [DynSerialize] allows serializing dynamic components for the purpose of logging.
///
/// # Send
/// Most of the time, execution should be multi threaded and having
/// components implement [Send] makes this much easier.
///
pub trait Component<P>: AnyComponent {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State) {}
    fn execute(&self, problem: &P, state: &mut State);
}
erased_serde::serialize_trait_object!(<P: Problem> Component<P>);

pub trait Condition<P>: AnyComponent {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State) {}
    fn evaluate(&self, problem: &P, state: &mut State) -> bool;
}
erased_serde::serialize_trait_object!(<P: Problem> Condition<P>);

pub type Configuration<P> = Box<dyn Component<P>>;

pub struct Scope<P> {
    body: Box<dyn Component<P>>,
    init: fn(&mut State),
}

impl<P> Component<P> for Scope<P>
where
    P: Problem + 'static,
{
    fn execute(&self, problem: &P, state: &mut State) {
        state.with_substate(|state| {
            (self.init)(state);
            self.body.initialize(problem, state);
            self.body.execute(problem, state);
        });
    }
}

impl<P: Problem + 'static> Scope<P> {
    pub fn new(body: Vec<Box<dyn Component<P>>>) -> Box<dyn Component<P>> {
        Self::new_with(|_| {}, body)
    }

    pub fn new_with(
        init: fn(&mut State),
        body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let body = Block::new(body);
        Box::new(Scope { body, init })
    }
}

pub struct Block<P>(Vec<Box<dyn Component<P>>>);

impl<P> Component<P> for Block<P>
where
    P: Problem + 'static,
{
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

impl<P: Problem + 'static> Block<P> {
    pub fn new(components: Vec<Box<dyn Component<P>>>) -> Box<dyn Component<P>> {
        Box::new(Block(components))
    }
}

pub struct Loop<P> {
    condition: Box<dyn Condition<P>>,
    body: Box<dyn Component<P>>,
}

impl<P> Component<P> for Loop<P>
where
    P: Problem + 'static,
{
    fn initialize(&self, problem: &P, state: &mut State) {
        self.condition.initialize(problem, state);
        self.body.initialize(problem, state);
    }

    fn execute(&self, problem: &P, state: &mut State) {
        self.condition.initialize(problem, state);
        while self.condition.evaluate(problem, state) {
            self.body.execute(problem, state);
        }
    }
}

impl<P: Problem + 'static> Loop<P> {
    pub fn new(
        condition: Box<dyn Condition<P>>,
        body: Vec<Box<dyn Component<P>>>,
    ) -> Box<dyn Component<P>> {
        let body = Block::new(body);
        Box::new(Loop { condition, body })
    }
}
