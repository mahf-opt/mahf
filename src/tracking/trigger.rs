use crate::state::{common::Iterations, CustomState, State};
use derive_deref::Deref;
use std::{
    any::Any,
    ops::{Deref, Sub},
};

/// Like [Condition](crate::framework::conditions::Condition) but non-serializable.
pub trait Trigger<P>: Any + Send + Sync {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State) {}
    fn evaluate(&self, problem: &P, state: &mut State) -> bool;
}

/// Triggers every `n` iterations.
#[derive(serde::Serialize)]
pub struct Iteration(u32);

impl Iteration {
    pub fn new<P>(iterations: u32) -> Box<dyn Trigger<P>> {
        Box::new(Iteration(iterations))
    }
}

impl<P> Trigger<P> for Iteration {
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<Iterations>();
    }

    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        state.iterations() % self.0 == 0
    }
}

#[derive(Deref)]
struct Previous<S>(S);
impl<S: 'static> CustomState for Previous<S> {}

/// Triggers when `S` changes base on a predicate.
#[derive(serde::Serialize)]
pub struct Change<S> {
    #[serde(skip)]
    check: Box<dyn Fn(&S, &S) -> bool + Send + Sync>,
}

impl<S> Change<S>
where
    S: CustomState + Clone,
{
    /// Create a new [Change] [Trigger] with a custom predicate.
    ///
    /// Will trigger when the predicate evaluates to `true`.
    pub fn custom<P>(
        check: impl Fn(&S, &S) -> bool + Send + Sync + 'static,
    ) -> Box<dyn Trigger<P>> {
        Box::new(Change {
            check: Box::new(check),
        })
    }
}

impl<S> Change<S>
where
    S: CustomState + Clone + Deref,
    S::Target: Clone + Sub<Output = S::Target> + Ord + Send + Sync + 'static,
{
    /// Create a new [Change] [Trigger] based on a threshhold.
    ///
    /// Requires `S` to dereference to something that implements [Sub] and [Ord].
    pub fn new<P>(threshhold: S::Target) -> Box<dyn Trigger<P>> {
        Box::new(Change {
            check: Box::new(move |old: &S, new: &S| {
                let old = old.deref();
                let new = new.deref();

                let min = Ord::min(old, new).clone();
                let max = Ord::max(old, new).clone();

                (max - min) >= threshhold
            }),
        })
    }
}

impl<S, P> Trigger<P> for Change<S>
where
    S: CustomState + Clone,
{
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<S>();
        let current = state.get::<S>().clone();
        state.insert(Previous(current));
    }

    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        let previous = state.get::<Previous<S>>();
        let current = state.get::<S>();
        let changed = (self.check)(previous, current);

        if changed {
            let new = current.clone();
            *state.get_mut::<S>() = new;
        }

        changed
    }
}
