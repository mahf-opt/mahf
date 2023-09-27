use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use downcast_rs::Downcast;
use dyn_clone::DynClone;
use trait_set::trait_set;

use crate::{state::StateResult, StateError};

trait_set! {
    pub trait LocalCustomState = Downcast + DynClone + Send;
}

downcast_rs::impl_downcast!(LocalCustomState);
dyn_clone::clone_trait_object!(LocalCustomState);

#[derive(Default, Clone)]
pub struct LocalStateRegistry {
    map: HashMap<TypeId, RefCell<Box<dyn LocalCustomState>>>,
}

impl LocalStateRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T>(&mut self, t: T) -> Option<T>
    where
        T: LocalCustomState,
    {
        self.map
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(t)))
            .map(|x| x.into_inner())
            .and_then(|x| x.into_any().downcast().ok())
            .map(|x| *x)
    }

    pub fn remove<T>(&mut self) -> StateResult<T>
    where
        T: LocalCustomState,
    {
        self.map
            .remove(&TypeId::of::<T>())
            .map(|x| x.into_inner())
            .and_then(|x| x.into_any().downcast().ok())
            .map(|x| *x)
            .ok_or_else(StateError::not_found::<T>)
    }

    #[track_caller]
    pub fn take<T>(&mut self) -> T
    where
        T: LocalCustomState,
    {
        self.remove().unwrap_or_else(StateError::panic)
    }

    pub fn contains<T>(&self) -> bool
    where
        T: LocalCustomState,
    {
        self.map.contains_key(&TypeId::of::<T>())
    }

    #[track_caller]
    pub fn borrow<T>(&self) -> Ref<'_, T>
    where
        T: LocalCustomState,
    {
        self.try_borrow::<T>().unwrap_or_else(StateError::panic)
    }

    pub fn try_borrow<T>(&self) -> StateResult<Ref<'_, T>>
    where
        T: LocalCustomState,
    {
        let r = self
            .map
            .get(&TypeId::of::<T>())
            .ok_or_else(StateError::not_found::<T>)?
            .try_borrow()
            .map_err(|e| StateError::borrow_conflict::<T>(e))?;

        Ok(Ref::map(r, |x| x.as_ref().as_any().downcast_ref().unwrap()))
    }

    #[track_caller]
    pub fn borrow_mut<T>(&self) -> RefMut<'_, T>
    where
        T: LocalCustomState,
    {
        self.try_borrow_mut::<T>().unwrap_or_else(StateError::panic)
    }

    pub fn try_borrow_mut<T>(&self) -> StateResult<RefMut<'_, T>>
    where
        T: LocalCustomState,
    {
        let r = self
            .map
            .get(&TypeId::of::<T>())
            .ok_or_else(StateError::not_found::<T>)?
            .try_borrow_mut()
            .map_err(|e| StateError::borrow_conflict_mut::<T>(e))?;

        Ok(RefMut::map(r, |x| {
            x.as_mut().as_any_mut().downcast_mut().unwrap()
        }))
    }

    #[track_caller]
    pub fn get_value<T>(&self) -> T::Target
    where
        T: LocalCustomState + Deref,
        T::Target: Sized + Clone,
    {
        self.try_get_value::<T>().unwrap_or_else(StateError::panic)
    }

    pub fn try_get_value<T>(&self) -> StateResult<T::Target>
    where
        T: LocalCustomState + Deref,
        T::Target: Sized + Clone,
    {
        self.try_borrow::<T>().map(|t| t.clone())
    }

    #[track_caller]
    pub fn borrow_value<T>(&self) -> Ref<T::Target>
    where
        T: LocalCustomState + Deref,
        T::Target: Sized,
    {
        self.try_borrow_value::<T>()
            .unwrap_or_else(StateError::panic)
    }

    pub fn try_borrow_value<T>(&self) -> StateResult<Ref<T::Target>>
    where
        T: LocalCustomState + Deref,
        T::Target: Sized,
    {
        let r = self.try_borrow::<T>()?;
        Ok(Ref::map(r, |x| x.deref()))
    }

    #[track_caller]
    pub fn borrow_value_mut<T>(&self) -> RefMut<T::Target>
    where
        T: LocalCustomState + DerefMut,
        T::Target: Sized,
    {
        self.try_borrow_value_mut::<T>()
            .unwrap_or_else(StateError::panic)
    }

    pub fn try_borrow_value_mut<T>(&self) -> StateResult<RefMut<T::Target>>
    where
        T: LocalCustomState + DerefMut,
        T::Target: Sized,
    {
        let r = self.try_borrow_mut::<T>()?;
        Ok(RefMut::map(r, |x| x.deref_mut()))
    }

    pub fn set_value<T>(&self, mut value: T::Target) -> Option<T::Target>
    where
        T: LocalCustomState + DerefMut,
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
        T: LocalCustomState,
    {
        self.map
            .get_mut(&TypeId::of::<T>())
            .map(|cell| cell.get_mut())
            .and_then(|x| x.as_mut().as_any_mut().downcast_mut())
    }
}
