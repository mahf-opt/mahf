use std::ops::{Deref, Sub};

use crate::framework::{common_state::Iterations, components::Condition, CustomState, State};
use derive_deref::Deref;

#[derive(serde::Serialize)]
pub struct Iteration(u32);

impl Iteration {
    pub fn new<P>(iterations: u32) -> Box<dyn Condition<P>> {
        Box::new(Iteration(iterations))
    }
}

impl<P> Condition<P> for Iteration {
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

#[derive(serde::Serialize)]
pub struct Change<S> {
    #[serde(skip)]
    check: Box<dyn Fn(&S, &S) -> bool + Send + Sync>,
}

impl<S> Change<S>
where
    S: CustomState + Clone,
{
    pub fn custom<P>(
        check: impl Fn(&S, &S) -> bool + Send + Sync + 'static,
    ) -> Box<dyn Condition<P>> {
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
    pub fn new<P>(threshhold: S::Target) -> Box<dyn Condition<P>> {
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

impl<S, P> Condition<P> for Change<S>
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
