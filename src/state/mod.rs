use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use better_any::{TidAble, TidExt};
use derive_more::{Deref, DerefMut};

use crate::{
    component::ExecResult,
    logging::log::Log,
    problems::{MultiObjectiveProblem, SingleObjectiveProblem},
    state::{entry::Entry, error::StateResult, multi::MultiStateTuple},
    Individual, Problem, SingleObjective,
};

pub mod common;
mod custom;
pub mod entry;
pub mod error;
pub mod extract;
pub mod multi;
pub mod random;
mod require;

pub use custom::CustomState;
pub use error::StateError;
pub use random::Random;
pub use require::StateReq;

pub type StateMap<'a> = HashMap<TypeId, RefCell<Box<dyn CustomState<'a>>>>;

#[derive(Default)]
pub struct StateRegistry<'a> {
    parent: Option<Box<StateRegistry<'a>>>,
    map: StateMap<'a>,
}

impl<'a> StateRegistry<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            map: HashMap::new(),
        }
    }

    pub fn parent(&self) -> Option<&Self> {
        self.parent.as_deref()
    }

    pub fn parent_mut(&mut self) -> Option<&mut Self> {
        self.parent.as_deref_mut()
    }

    fn find<T>(&self) -> StateResult<&Self>
    where
        T: CustomState<'a>,
    {
        if self.contains::<T>() {
            Ok(self)
        } else {
            self.parent()
                .ok_or_else(StateError::not_found::<T>)
                .and_then(Self::find::<T>)
        }
    }

    fn find_mut<T>(&mut self) -> StateResult<&mut Self>
    where
        T: CustomState<'a>,
    {
        if self.contains::<T>() {
            Ok(self)
        } else {
            self.parent_mut()
                .ok_or_else(StateError::not_found::<T>)
                .and_then(Self::find_mut::<T>)
        }
    }

    pub fn entry<T>(&mut self) -> Entry<'_, 'a, T>
    where
        T: CustomState<'a>,
    {
        if self.has::<T>() {
            // This is necessary because `self` can't be borrowed mutably by both
            // `self.find_mut::<T>` and as default.
            // The nice solution `self.find_mut::<T>().unwrap_or(self)` is therefore not possible.
            // The `unwrap()` is guaranteed to never fail, because we checked this with `has()`.
            // TODO: Revisit if there is no better solution, this currently means traversing the parents twice.
            Entry::new(self.find_mut::<T>().unwrap().map.entry(T::id()))
        } else {
            Entry::new(self.map.entry(T::id()))
        }
    }

    pub fn requirements(&self) -> StateReq<'_, 'a> {
        StateReq::new(self)
    }

    pub fn insert<T>(&mut self, t: T) -> Option<T>
    where
        T: CustomState<'a>,
    {
        self.map
            .insert(T::id(), RefCell::new(Box::new(t)))
            .map(|x| x.into_inner())
            .and_then(|x| x.downcast_box().ok())
            .map(|x| *x)
    }

    pub fn insert_default<T>(&mut self) -> Option<T>
    where
        T: CustomState<'a> + Default,
    {
        self.map
            .insert(T::id(), RefCell::new(Box::<T>::default()))
            .map(|x| x.into_inner())
            .and_then(|x| x.downcast_box().ok())
            .map(|x| *x)
    }

    pub fn remove<T>(&mut self) -> StateResult<T>
    where
        T: CustomState<'a>,
    {
        self.find_mut::<T>()?
            .map
            .remove(&T::id())
            .map(|x| x.into_inner())
            .and_then(|x| x.downcast_box().ok())
            .map(|x| *x)
            .ok_or_else(StateError::not_found::<T>)
    }

    #[track_caller]
    pub fn take<T>(&mut self) -> T
    where
        T: CustomState<'a>,
    {
        self.remove().unwrap_or_else(StateError::panic)
    }

    pub fn contains<T>(&self) -> bool
    where
        T: CustomState<'a>,
    {
        self.map.contains_key(&T::id())
    }

    pub fn has<T>(&self) -> bool
    where
        T: CustomState<'a>,
    {
        self.find::<T>().is_ok()
    }

    #[track_caller]
    pub fn borrow<T>(&self) -> Ref<'_, T>
    where
        T: CustomState<'a>,
    {
        self.try_borrow::<T>().unwrap_or_else(StateError::panic)
    }

    pub fn try_borrow<T>(&self) -> StateResult<Ref<'_, T>>
    where
        T: CustomState<'a>,
    {
        let r = self
            .find::<T>()?
            .map
            .get(&T::id())
            .ok_or_else(StateError::not_found::<T>)?
            .try_borrow()
            .map_err(|e| StateError::borrow_conflict::<T>(e))?;

        Ok(Ref::map(r, |x| x.as_ref().downcast_ref().unwrap()))
    }

    #[track_caller]
    pub fn borrow_mut<T>(&self) -> RefMut<'_, T>
    where
        T: CustomState<'a>,
    {
        self.try_borrow_mut::<T>().unwrap_or_else(StateError::panic)
    }

    pub fn try_borrow_mut<T>(&self) -> StateResult<RefMut<'_, T>>
    where
        T: CustomState<'a>,
    {
        let r = self
            .find::<T>()?
            .map
            .get(&T::id())
            .ok_or_else(StateError::not_found::<T>)?
            .try_borrow_mut()
            .map_err(|e| StateError::borrow_conflict_mut::<T>(e))?;

        Ok(RefMut::map(r, |x| x.as_mut().downcast_mut().unwrap()))
    }

    #[track_caller]
    pub fn get_value<T>(&self) -> T::Target
    where
        T: CustomState<'a> + Deref,
        T::Target: Sized + Clone,
    {
        self.try_get_value::<T>().unwrap_or_else(StateError::panic)
    }

    pub fn try_get_value<T>(&self) -> StateResult<T::Target>
    where
        T: CustomState<'a> + Deref,
        T::Target: Sized + Clone,
    {
        let r = self.try_borrow::<T>()?;
        Ok(r.clone())
    }

    #[track_caller]
    pub fn borrow_value<T>(&self) -> Ref<T::Target>
    where
        T: CustomState<'a> + Deref,
        T::Target: Sized,
    {
        self.try_borrow_value::<T>()
            .unwrap_or_else(StateError::panic)
    }

    pub fn try_borrow_value<T>(&self) -> StateResult<Ref<T::Target>>
    where
        T: CustomState<'a> + Deref,
        T::Target: Sized,
    {
        let r = self.try_borrow::<T>()?;
        Ok(Ref::map(r, |x| x.deref()))
    }

    #[track_caller]
    pub fn borrow_value_mut<T>(&self) -> RefMut<T::Target>
    where
        T: CustomState<'a> + DerefMut,
        T::Target: Sized,
    {
        self.try_borrow_value_mut::<T>()
            .unwrap_or_else(StateError::panic)
    }

    pub fn try_borrow_value_mut<T>(&self) -> StateResult<RefMut<T::Target>>
    where
        T: CustomState<'a> + DerefMut,
        T::Target: Sized,
    {
        let r = self.try_borrow_mut::<T>()?;
        Ok(RefMut::map(r, |x| x.deref_mut()))
    }

    pub fn set_value<T>(&self, mut value: T::Target) -> Option<T::Target>
    where
        T: CustomState<'a> + DerefMut,
        T::Target: Sized,
    {
        if let Ok(mut r) = self.try_borrow_value_mut::<T>() {
            std::mem::swap(r.deref_mut(), &mut value);
            Some(value)
        } else {
            None
        }
    }

    pub fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: CustomState<'a>,
    {
        self.find_mut::<T>()
            .ok()
            .and_then(|state| state.map.get_mut(&T::id()))
            .map(|cell| cell.get_mut())
            .and_then(|x| x.as_mut().downcast_mut())
    }

    #[track_caller]
    pub fn get_multiple_mut<'b, T: MultiStateTuple<'b, 'a>>(&'b mut self) -> T::References {
        self.try_get_multiple_mut::<T>()
            .unwrap_or_else(StateError::panic)
    }

    pub fn try_get_multiple_mut<'b, T: MultiStateTuple<'b, 'a>>(
        &'b mut self,
    ) -> StateResult<T::References> {
        T::try_fetch(self)
    }
}

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

    pub fn with_inner_state<F>(&mut self, f: F) -> ExecResult<()>
    where
        F: FnOnce(&mut Self) -> ExecResult<()>,
    {
        let child = StateRegistry {
            parent: Some(Box::new(std::mem::take(&mut self.registry))),
            map: HashMap::new(),
        };
        let mut state = Self::from(child);
        f(&mut state)?;
        self.registry = *StateRegistry::from(state).parent.unwrap();
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
