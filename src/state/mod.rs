use std::{
    cell::{Ref, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use better_any::TidAble;
use derive_more::{Deref, DerefMut};

use crate::{
    component::ExecResult,
    logging::log::Log,
    problems::{MultiObjectiveProblem, SingleObjectiveProblem},
    Individual, Problem, SingleObjective,
};

pub mod common;
pub mod lens;
pub mod random;
pub mod registry;
mod require;

pub use random::Random;
pub use registry::{CustomState, StateError, StateRegistry, StateResult};
pub use require::StateReq;

#[derive(Default, Deref, DerefMut)]
pub struct State<'a, P> {
    #[deref]
    #[deref_mut]
    registry: StateRegistry<'a>,
    marker: PhantomData<P>,
}

impl<'a, P> State<'a, P> {
    pub fn new() -> Self {
        Self::from(StateRegistry::new())
    }

    pub fn requirements(&self) -> StateReq<'_, 'a, P> {
        StateReq::new(self)
    }

    pub fn with_inner_state<F>(&mut self, f: F) -> ExecResult<()>
    where
        F: FnOnce(&mut Self) -> ExecResult<()>,
    {
        let registry = std::mem::take(&mut self.registry);
        let mut state = registry.into_child().into();
        f(&mut state)?;
        let (registry, _) = StateRegistry::from(state).into_parent();
        self.registry = registry.unwrap();
        Ok(())
    }

    pub fn holding<T>(
        &mut self,
        f: impl FnOnce(&mut T, &mut Self) -> ExecResult<()>,
    ) -> ExecResult<()>
    where
        T: CustomState<'a> + TidAble<'a>,
    {
        #[derive(better_any::Tid)]
        struct Marker<T>(PhantomData<fn() -> T>);
        impl<'a, T: TidAble<'a>> CustomState<'a> for Marker<T> {}

        let registry_with_t = self.find_mut::<T>()?;
        registry_with_t.insert(Marker::<T>(PhantomData));
        let mut t = registry_with_t.remove::<T>()?;
        f(&mut t, self)?;

        let state_with_t = self.find_mut::<Marker<T>>()?;
        state_with_t.insert(t);
        state_with_t.remove::<Marker<T>>()?;

        Ok(())
    }
}

impl<'a, P> From<StateRegistry<'a>> for State<'a, P> {
    fn from(value: StateRegistry<'a>) -> Self {
        Self {
            registry: value,
            marker: PhantomData,
        }
    }
}

impl<'a, P> From<State<'a, P>> for StateRegistry<'a> {
    fn from(value: State<'a, P>) -> Self {
        value.registry
    }
}

impl<P> State<'_, P>
where
    P: Problem,
{
    /// Returns [Iterations](common::Iterations) state.
    pub fn iterations(&self) -> u32 {
        self.get_value::<common::Iterations>()
    }

    /// Returns [Evaluations](common::Evaluations) state.
    pub fn evaluations(&self) -> u32 {
        self.get_value::<common::Evaluations>()
    }

    /// Returns [Population](common::Populations) state.
    pub fn populations(&self) -> Ref<'_, common::Populations<P>> {
        self.borrow::<common::Populations<P>>()
    }

    /// Returns mutable [Population](common::Populations) state.
    pub fn populations_mut(&self) -> RefMut<'_, common::Populations<P>> {
        self.borrow_mut::<common::Populations<P>>()
    }

    /// Returns the mutable random number generator [Random].
    pub fn random_mut(&self) -> RefMut<'_, Random> {
        self.borrow_mut::<Random>()
    }

    /// Returns the [Log].
    pub fn log(&self) -> Ref<'_, Log> {
        self.borrow::<Log>()
    }
}

impl<P> State<'_, P>
where
    P: SingleObjectiveProblem,
{
    /// Returns [BestIndividual](common::BestIndividual) state.
    pub fn best_individual(&self) -> Option<Ref<'_, Individual<P>>> {
        let r = self.try_borrow::<common::BestIndividual<P>>().ok()?;
        Ref::filter_map(r, |r| r.deref().as_ref()).ok()
    }

    /// Returns [BestIndividual](common::BestIndividual) state.
    pub fn best_individual_mut(&self) -> Option<RefMut<'_, Individual<P>>> {
        let r = self.try_borrow_mut::<common::BestIndividual<P>>().ok()?;
        RefMut::filter_map(r, |r| r.deref_mut().as_mut()).ok()
    }

    /// Returns the objective value of the [BestIndividual](common::BestIndividual).
    pub fn best_objective_value(&self) -> Option<SingleObjective> {
        self.best_individual().map(|i| *i.objective())
    }
}

impl<P> State<'_, P>
where
    P: MultiObjectiveProblem,
{
    /// Returns [ParetoFront](common::ParetoFront) state.
    pub fn pareto_front(&self) -> Ref<'_, common::ParetoFront<P>> {
        self.borrow::<common::ParetoFront<P>>()
    }

    /// Returns [ParetoFront](common::ParetoFront) state.
    pub fn pareto_front_mut(&self) -> RefMut<'_, common::ParetoFront<P>> {
        self.borrow_mut::<common::ParetoFront<P>>()
    }
}
