use std::{
    marker::PhantomData,
    ops::{Deref, Sub},
};
use crate::problems::{HasKnownTarget, SingleObjectiveProblem};
use crate::state::common::Evaluations;

use crate::{
    problems::Problem,
    state::{common::Iterations, CustomState, State},
};
use better_any::{Tid, TidAble};
use derive_more::Deref;
use dyn_clone::DynClone;

/// Like [Condition](crate::framework::conditions::Condition) but non-serializable.
pub trait Trigger<'a, P>: DynClone + Send {
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State<'a, P>) {}
    fn evaluate(&self, problem: &P, state: &mut State<'a, P>) -> bool;
}
dyn_clone::clone_trait_object!(<'a, P> Trigger<'a, P>);

/// Triggers every `n` iterations.
#[derive(Clone, serde::Serialize)]
pub struct Iteration(u32);

impl Iteration {
    pub fn new<'a, P: Problem + 'static>(iterations: u32) -> Box<dyn Trigger<'a, P>> {
        Box::new(Iteration(iterations))
    }
}

impl<'a, P: Problem + 'static> Trigger<'a, P> for Iteration {
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.require::<Self, Iterations>();
    }

    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> bool {
        state.iterations() % self.0 == 0
    }
}

/// Triggers on final iteration.
///
/// Works only if termination uses FixedIterations.
#[derive(Clone, serde::Serialize)]
pub struct FinalIter(u32);

impl FinalIter {
    pub fn new<P>(final_iteration: u32) -> Box<dyn Trigger<P>> {
        Box::new(FinalIter(final_iteration))
    }
}

impl<P> Trigger<P> for FinalIter {
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.require::<Iterations, T>();
    }

    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> bool {
        state.iterations() == self.0 - 1
    }
}

/// Triggers on final evaluation.
///
/// Works only if termination uses FixedEvaluations.
#[derive(Clone, serde::Serialize)]
pub struct FinalEval(u32);

impl FinalEval {
    pub fn new<P>(final_evaluation: u32) -> Box<dyn Trigger<P>> {
        Box::new(FinalEval(final_evaluation))
    }
}

impl<P> Trigger<P> for FinalEval {
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.require::<Evaluations, T>();
    }

    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> bool {
        state.evaluations() >= self.0
    }
}

/// Triggers on target hit.
///
/// Works only if optimum has known target.
#[derive(Clone, serde::Serialize)]
pub struct TargetHit;

impl TargetHit {
    pub fn new<P: HasKnownTarget + SingleObjectiveProblem>() -> Box<dyn Trigger<P>> {
        Box::new(TargetHit)
    }
}

impl<P: HasKnownTarget + SingleObjectiveProblem> Trigger<P> for TargetHit {
    fn initialize(&self, _problem: &P, _state: &mut State<P>) {

    }

    fn evaluate(&self, problem: &P, state: &mut State<P>) -> bool {
        if let Some(fitness) = state.best_objective_value() {
            problem.target_hit(*fitness)
        } else {
            false
        }
    }
}

#[derive(Deref, Tid)]
struct Previous<'a, S: TidAble<'a> + Send> {
    #[deref]
    pub inner: S,
    _phantom: PhantomData<&'a ()>,
}
impl<'a, S: TidAble<'a> + Send> Previous<'a, S> {
    pub fn new(inner: S) -> Self {
        Previous {
            inner,
            _phantom: PhantomData::default(),
        }
    }
}

impl<'a, S: TidAble<'a> + Send> CustomState<'a> for Previous<'a, S> {}

/// Triggers when `S` changes base on a predicate.
#[derive(Tid, Clone)]
pub struct Change<'a, S> {
    check: Box<dyn Comparator<S> + 'a>,
}

pub trait Comparator<S>: Send + Sync + DynClone {
    fn compare(&self, a: &S, b: &S) -> bool;
}
dyn_clone::clone_trait_object!(<S> Comparator<S>);

impl<S, F> Comparator<S> for F
where
    F: Fn(&S, &S) -> bool + Send + Sync + Clone,
{
    fn compare(&self, a: &S, b: &S) -> bool {
        self(a, b)
    }
}

impl<'s, S> Change<'s, S>
where
    S: CustomState<'s> + TidAble<'s> + Clone,
{
    /// Create a new [Change] [Trigger] with a custom predicate.
    ///
    /// Will trigger when the predicate evaluates to `true`.
    pub fn custom<P: Problem>(check: impl Comparator<S> + 's) -> Box<dyn Trigger<'s, P> + 's> {
        Box::new(Change {
            check: Box::new(check),
        })
    }
}

impl<'s, S> Change<'s, S>
where
    S: CustomState<'s> + TidAble<'s> + Clone + Deref,
    S::Target: Clone + Sub<Output = S::Target> + PartialOrd + Send + Sync,
{
    /// Create a new [Change] [Trigger] based on a threshold.
    ///
    /// Requires `S` to dereference to something that implements [Sub] and [Ord].
    pub fn new<P: Problem>(threshhold: S::Target) -> Box<dyn Trigger<'s, P> + 's> {
        Box::new(Change {
            check: Box::new(move |old: &S, new: &S| {
                let old = old.deref();
                let new = new.deref();

                let min = min(old, new).clone();
                let max = max(old, new).clone();

                (max - min) >= threshold
            }),
        })
    }
}

impl<'s, S, P> Trigger<'s, P> for Change<'s, S>
where
    S: CustomState<'s> + TidAble<'s> + Clone,
    P: Problem + 's,
{
    fn initialize(&self, _problem: &P, state: &mut State<'s, P>) {
        state.require::<Self, S>();
        let current = state.get::<S>().clone();
        state.insert(Previous::new(current));
    }

    fn evaluate(&self, _problem: &P, state: &mut State<'s, P>) -> bool {
        let previous = state.get::<Previous<S>>();
        let current = state.get::<S>();
        let changed = self.check.compare(previous, current);

        if changed {
            let new = current.clone();
            *state.get_mut::<S>() = new;
        }

        changed
    }
}

#[inline]
fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
