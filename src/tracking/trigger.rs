use std::ops::{Deref, Sub};

use crate::framework::{common_state::Iterations, components::Condition, CustomState, State};
use derive_deref::Deref;

#[derive(serde::Serialize)]
pub struct OnNthIteration(u32);

impl OnNthIteration {
    pub fn new<P>(iterations: u32) -> Box<dyn Condition<P>> {
        Box::new(OnNthIteration(iterations))
    }
}

impl<P> Condition<P> for OnNthIteration {
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
pub struct OnImprovement<S> {
    #[serde(skip)]
    check: Box<dyn Fn(&S, &S) -> bool + Send + Sync>,
}

impl<S> OnImprovement<S>
where
    S: CustomState + Clone,
{
    pub fn custom<P>(
        check: impl Fn(&S, &S) -> bool + Send + Sync + 'static,
    ) -> Box<dyn Condition<P>> {
        Box::new(OnImprovement {
            check: Box::new(check),
        })
    }
}

impl<S, I> OnImprovement<S>
where
    I: Clone + Sub<Output = I> + Ord + Send + Sync + 'static,
    S: CustomState + Clone + Deref<Target = I>,
{
    pub fn new<P>(threshhold: I) -> Box<dyn Condition<P>> {
        Box::new(OnImprovement {
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

impl<S, P> Condition<P> for OnImprovement<S>
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
