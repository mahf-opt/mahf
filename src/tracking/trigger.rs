use crate::state::{common::Iterations, CustomState, State};
use derive_deref::Deref;
use std::{
    any::Any,
    ops::{Deref, Sub},
};
use crate::problems::{HasKnownTarget, SingleObjectiveProblem};
use crate::state::common::Evaluations;

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

/// Triggers on final iteration.
///
/// Works only if termination uses FixedIterations.
#[derive(serde::Serialize)]
pub struct FinalIter(u32);

impl FinalIter {
    pub fn new<P>(final_iteration: u32) -> Box<dyn Trigger<P>> {
        Box::new(FinalIter(final_iteration))
    }
}

impl<P> Trigger<P> for FinalIter {
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<Iterations>();
    }

    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        state.iterations() == self.0 - 1
    }
}

/// Triggers on final evaluation.
///
/// Works only if termination uses FixedEvaluations.
#[derive(serde::Serialize)]
pub struct FinalEval(u32);

impl FinalEval {
    pub fn new<P>(final_evaluation: u32) -> Box<dyn Trigger<P>> {
        Box::new(FinalEval(final_evaluation))
    }
}

impl<P> Trigger<P> for FinalEval {
    fn initialize(&self, _problem: &P, state: &mut State) {
        state.require::<Evaluations>();
    }

    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        state.evaluations() >= self.0
    }
}

/// Triggers on target hit.
///
/// Works only if optimum has known target.
#[derive(serde::Serialize)]
pub struct TargetHit;

impl TargetHit {
    pub fn new<P: HasKnownTarget + SingleObjectiveProblem>() -> Box<dyn Trigger<P>> {
        Box::new(TargetHit)
    }
}

impl<P: HasKnownTarget + SingleObjectiveProblem> Trigger<P> for TargetHit {
    fn initialize(&self, _problem: &P, _state: &mut State) {

    }

    fn evaluate(&self, problem: &P, state: &mut State) -> bool {
        if let Some(fitness) = state.best_objective_value::<P>() {
            problem.target_hit(*fitness)
        } else {
            false
        }
    }
}

#[derive(Deref)]
struct Previous<S>(S);
impl<S: 'static + Send> CustomState for Previous<S> {}

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
    /// Create a new [Change] [Trigger] based on a threshold.
    ///
    /// Requires `S` to dereference to something that implements [Sub] and [Ord].
    pub fn new<P>(threshold: S::Target) -> Box<dyn Trigger<P>> {
        Box::new(Change {
            check: Box::new(move |old: &S, new: &S| {
                let old = old.deref();
                let new = new.deref();

                let min = Ord::min(old, new).clone();
                let max = Ord::max(old, new).clone();

                (max - min) >= threshold
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
