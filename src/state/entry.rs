//! A reimplementation of the [`Entry`] API of [`HashMap`], but for [`StateRegistry`].
//!
//! [`Entry`]: std::collections::hash::map::Entry
//! [`HashMap`]: std::collections::HashMap
//! [`StateRegistry`]: crate::StateRegistry

use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::hash_map,
    marker::PhantomData,
};

use better_any::TidExt;

use crate::CustomState;

pub type HashMapEntry<'a, 'b> = hash_map::Entry<'a, TypeId, RefCell<Box<dyn CustomState<'b>>>>;

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`StateRegistry`].
///
/// [`entry`]: crate::StateRegistry::entry
/// [`StateRegistry`]: crate::StateRegistry
pub enum Entry<'a, 'b, T> {
    Occupied(OccupiedEntry<'a, 'b, T>),
    Vacant(VacantEntry<'a, 'b, T>),
}

/// A view into an occupied entry in a [`StateRegistry`].
///
/// It is part of the [`Entry`] enum.
///
/// [`StateRegistry`]: crate::StateRegistry
pub struct OccupiedEntry<'a, 'b, T> {
    base: hash_map::OccupiedEntry<'a, TypeId, RefCell<Box<dyn CustomState<'b>>>>,
    marker: PhantomData<T>,
}

impl<'a, 'b, T> OccupiedEntry<'a, 'b, T>
where
    T: CustomState<'b>,
{
    pub fn remove_entry(self) -> T {
        let cell = self.base.remove_entry().1;
        *cell
            .into_inner()
            .downcast_box()
            .unwrap_or_else(|_| unreachable!())
    }

    pub fn get(&self) -> Ref<'_, T> {
        let cell = self.base.get();
        Ref::map(cell.borrow(), |x| {
            x.as_ref().downcast_ref().unwrap_or_else(|| {
                unreachable!("`OccupiedEntry<T>` should only be constructed for valid `T`")
            })
        })
    }

    pub fn get_mut(&mut self) -> RefMut<'_, T> {
        let cell = self.base.get_mut();
        RefMut::map(cell.borrow_mut(), |x| {
            x.as_mut().downcast_mut().unwrap_or_else(|| {
                unreachable!("`OccupiedEntry<T>` should only be constructed for valid `T`")
            })
        })
    }

    pub fn into_mut(self) -> RefMut<'a, T> {
        let cell = self.base.into_mut();
        RefMut::map(cell.borrow_mut(), |x| {
            x.as_mut().downcast_mut().unwrap_or_else(|| {
                unreachable!("`OccupiedEntry<T>` should only be constructed for valid `T`")
            })
        })
    }

    pub fn insert(&mut self, value: T) -> T {
        let cell = RefCell::new(Box::new(value));
        *self
            .base
            .insert(cell)
            .into_inner()
            .downcast_box()
            .unwrap_or_else(|_| {
                unreachable!("`OccupiedEntry<T>` should only be constructed for valid `T`")
            })
    }

    pub fn remove(self) -> T {
        let cell = self.base.remove();
        *cell.into_inner().downcast_box().unwrap_or_else(|_| {
            unreachable!("`OccupiedEntry<T>` should only be constructed for valid `T`")
        })
    }
}

/// A view into an vacant entry in a [`StateRegistry`].
///
/// It is part of the [`Entry`] enum.
///
/// [`StateRegistry`]: crate::StateRegistry
pub struct VacantEntry<'a, 'b, T> {
    base: hash_map::VacantEntry<'a, TypeId, RefCell<Box<dyn CustomState<'b>>>>,
    marker: PhantomData<T>,
}

impl<'a, 'b, T> VacantEntry<'a, 'b, T>
where
    T: CustomState<'b>,
{
    pub fn insert(self, value: T) -> RefMut<'a, T> {
        let cell = RefCell::new(Box::new(value));
        RefMut::map(self.base.insert(cell).borrow_mut(), |x| {
            x.as_mut()
                .downcast_mut()
                .unwrap_or_else(|| unreachable!("`T` should have been inserted before this call"))
        })
    }
}

impl<'a, 'b, T> Entry<'a, 'b, T>
where
    T: CustomState<'b>,
{
    pub(crate) fn new(entry: HashMapEntry<'a, 'b>) -> Self {
        match entry {
            HashMapEntry::Occupied(entry) => Self::Occupied(OccupiedEntry {
                base: entry,
                marker: PhantomData,
            }),
            HashMapEntry::Vacant(entry) => Self::Vacant(VacantEntry {
                base: entry,
                marker: PhantomData,
            }),
        }
    }

    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(RefMut<T>),
    {
        match self {
            Self::Occupied(mut entry) => {
                f(entry.get_mut());
                Self::Occupied(entry)
            }
            Self::Vacant(entry) => Self::Vacant(entry),
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use better_any::{Tid, TidAble};
    /// use mahf::{CustomState, StateRegistry};
    /// # #[derive(PartialEq, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    ///
    /// registry.entry::<A>().or_insert(A(3));
    /// assert_eq!(registry.borrow::<A>(), A(3));
    ///
    /// *map.entry::<A>().or_insert(A(10)).0 *= 2;
    /// assert_eq!(registry.borrow::<A>(), A(3));
    /// ```
    #[inline]
    pub fn or_insert(self, default: T) -> RefMut<'a, T> {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(default),
        }
    }

    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> RefMut<'a, T> {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(default()),
        }
    }
}

impl<'a, 'b, T> Entry<'a, 'b, T>
where
    T: CustomState<'b> + Default,
{
    pub fn or_default(self) -> RefMut<'a, T> {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(Default::default()),
        }
    }
}
