//! TODO

use std::{
    cell::{Ref, RefMut},
    marker::PhantomData,
    ops::Deref,
};

use better_any::TidAble;
use derive_more::{Deref, DerefMut};

use crate::{
    component::ExecResult,
    logging,
    logging::log::Log,
    problems::{MultiObjectiveProblem, SingleObjectiveProblem},
    Individual, Problem, SingleObjective,
};

pub mod common;
pub mod random;
pub mod registry;
mod require;

pub use random::Random;
pub use registry::{CustomState, StateError, StateRegistry, StateResult};
pub use require::StateReq;

/// Runtime state of a (metaheuristic) [`Configuration`].
///
/// The `State` acts as a shared variable storage (blackboard), which supports inserting,
/// reading, writing, and removing [`CustomState`].
///
/// This is a thin wrapper around [`StateRegistry`], which defines additional methods
/// to access often used (so-called "common") state.
///
/// [`Configuration`]: crate::Configuration
#[derive(Deref, DerefMut)]
pub struct State<'a, P> {
    #[deref]
    #[deref_mut]
    registry: StateRegistry<'a>,
    marker: PhantomData<P>,
}

impl<'a, P> State<'a, P> {
    /// Creates a new `State`.
    ///
    /// The state is automatically created by the [`Configuration`],
    /// and therefore, it usually does not need to be created manually.
    ///
    /// [`Configuration`]: crate::Configuration
    pub fn new() -> Self {
        Self::from(StateRegistry::new())
    }

    /// Enables checking if specific custom state is present in the state.
    pub(crate) fn requirements(&self) -> StateReq<'_, 'a, P> {
        StateReq::new(self)
    }

    /// Calls `f` with a child state, which is discarded afterwards.
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

    /// Borrows `T` and the remaining state mutably at the same time.
    ///
    /// To make this possible, `T` is removed temporarily.
    ///
    /// This makes passing the state to another function while `T` is borrowed possible.
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

impl<P> Default for State<'_, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P> State<'_, P>
where
    P: Problem,
{
    /// Returns the current number of [`Iterations`].
    ///
    /// [`Iterations`]: common::Iterations
    pub fn iterations(&self) -> u32 {
        self.get_value::<common::Iterations>()
    }

    /// Returns the current number of objective function [`Evaluations`].
    ///
    /// [`Evaluations`]: common::Evaluations
    pub fn evaluations(&self) -> u32 {
        self.get_value::<common::Evaluations>()
    }

    /// Returns a reference to the stack of [`Populations`].
    ///
    /// [`Populations`]: common::Populations
    pub fn populations(&self) -> Ref<'_, common::Populations<P>> {
        self.borrow::<common::Populations<P>>()
    }

    /// Returns a mutable reference to the stack of [`Populations`].
    ///
    /// [`Populations`]: common::Populations
    pub fn populations_mut(&self) -> RefMut<'_, common::Populations<P>> {
        self.borrow_mut::<common::Populations<P>>()
    }

    /// Returns a mutable reference to the random number generator [Random].
    pub fn random_mut(&self) -> RefMut<'_, Random> {
        self.borrow_mut::<Random>()
    }

    /// Returns a reference to the [Log].
    pub fn log(&self) -> Ref<'_, Log> {
        self.borrow::<Log>()
    }

    pub fn configure_log<F>(&mut self, f: F) -> ExecResult<()>
    where
        F: FnOnce(&mut logging::LogConfig<P>) -> ExecResult<()>,
    {
        let mut config = self.entry::<logging::LogConfig<P>>().or_default();
        f(&mut *config)
    }
}

impl<P> State<'_, P>
where
    P: SingleObjectiveProblem,
{
    /// Returns a reference to the [`BestIndividual`] yet found.
    ///
    /// [`BestIndividual`]: common::BestIndividual
    pub fn best_individual(&self) -> Option<Ref<'_, Individual<P>>> {
        let r = self.try_borrow::<common::BestIndividual<P>>().ok()?;
        Ref::filter_map(r, |r| r.deref().as_ref()).ok()
    }

    /// Returns the objective value of the [`BestIndividual`] yet found.
    ///
    /// [`BestIndividual`]: common::BestIndividual
    pub fn best_objective_value(&self) -> Option<SingleObjective> {
        self.best_individual().map(|i| *i.objective())
    }
}

impl<P> State<'_, P>
where
    P: MultiObjectiveProblem,
{
    /// Returns the current approximation of the [`ParetoFront`].
    ///
    /// [`ParetoFront`]: common::ParetoFront
    pub fn pareto_front(&self) -> Ref<'_, common::ParetoFront<P>> {
        self.borrow::<common::ParetoFront<P>>()
    }
}
