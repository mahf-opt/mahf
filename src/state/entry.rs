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
    ops::DerefMut,
};

use better_any::TidExt;

use crate::CustomState;

pub type HashMapEntry<'a, 'b> = hash_map::Entry<'a, TypeId, RefCell<Box<dyn CustomState<'b>>>>;

/// A view into a single entry in a state registry, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`StateRegistry`].
///
/// [`entry`]: crate::StateRegistry::entry
/// [`StateRegistry`]: crate::StateRegistry
pub enum Entry<'a, 'b, T> {
    Occupied(OccupiedEntry<'a, 'b, T>),
    Vacant(VacantEntry<'a, 'b, T>),
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

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    ///
    /// registry.entry::<A>()
    ///    .and_modify(|mut a| { a.0 += 1 })
    ///    .or_insert(A(42));
    /// assert_eq!(registry.get_value::<A>(), 42);
    ///
    /// registry.entry::<A>()
    ///    .and_modify(|mut a| { a.0 += 1 })
    ///    .or_insert(A(42));
    /// assert_eq!(registry.get_value::<A>(), 43);
    /// ```
    #[inline]
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

    /// Provides in-place mutable access to the value of an occupied entry before any
    /// potential inserts into the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    ///
    /// registry.entry::<A>()
    ///    .and_modify_value(|a| { *a += 1 })
    ///    .or_insert(A(42));
    /// assert_eq!(registry.get_value::<A>(), 42);
    ///
    /// registry.entry::<A>()
    ///    .and_modify_value(|a| { *a += 1 })
    ///    .or_insert(A(42));
    /// assert_eq!(registry.get_value::<A>(), 43);
    /// ```
    #[inline]
    pub fn and_modify_value<F>(self, f: F) -> Self
    where
        T: DerefMut,
        F: FnOnce(&mut T::Target),
    {
        match self {
            Self::Occupied(mut entry) => {
                f(&mut *entry.get_mut());
                Self::Occupied(entry)
            }
            Self::Vacant(entry) => Self::Vacant(entry),
        }
    }

    /// Ensures a value is in the entry by inserting `default` if empty, and returns
    /// a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    ///
    /// registry.entry::<A>().or_insert(A(3));
    /// assert_eq!(registry.get_value::<A>(), 3);
    ///
    /// registry.entry::<A>().or_insert(A(10)).0 *= 2;
    /// assert_eq!(registry.get_value::<A>(), 6);
    /// ```
    #[inline]
    pub fn or_insert(self, default: T) -> RefMut<'a, T> {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the `default` function if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    ///
    /// registry.entry::<A>().or_insert_with(|| A(3));
    /// assert_eq!(registry.get_value::<A>(), 3);
    ///
    /// registry.entry::<A>().or_insert_with(|| A(10)).0 *= 2;
    /// assert_eq!(registry.get_value::<A>(), 6);
    /// ```
    #[inline]
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
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Default, Debug, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    ///
    /// registry.entry::<A>().or_default(); // A defaults to A(0)
    /// assert_eq!(registry.get_value::<A>(), 0);
    /// ```
    #[inline]
    pub fn or_default(self) -> RefMut<'a, T> {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(Default::default()),
        }
    }
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
    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// use mahf::state::entry::Entry;
    /// # #[derive(Default, Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.entry::<A>().or_insert(A(12));
    ///
    /// if let Entry::Occupied(o) = registry.entry::<A>() {
    ///     assert_eq!(&*o.get(), &A(12));
    /// }
    /// ```
    #[inline]
    pub fn get(&self) -> Ref<'_, T> {
        let cell = self.base.get();
        Ref::map(cell.borrow(), |x| {
            x.as_ref().downcast_ref().unwrap_or_else(|| {
                unreachable!("`OccupiedEntry<T>` should only be constructed for valid `T`")
            })
        })
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// If you need a reference to the `OccupiedEntry` which may outlive the
    /// destruction of the `Entry` value, see [`into_mut`].
    ///
    /// [`into_mut`]: Self::into_mut
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// use mahf::state::entry::Entry;
    /// # #[derive(Default, Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.entry::<A>().or_insert(A(12));
    ///
    /// assert_eq!(registry.get_value::<A>(), 12);
    /// if let Entry::Occupied(mut o) = registry.entry::<A>() {
    ///     o.get_mut().0 += 10;
    ///     assert_eq!(&*o.get(), &A(22));
    ///
    ///     // We can use the same Entry multiple times, given that the
    ///     // reference by the previous `get_mut` is dropped before.
    ///     o.get_mut().0 += 2;
    /// }
    ///
    /// assert_eq!(registry.get_value::<A>(), 24);
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> RefMut<'_, T> {
        let cell = self.base.get_mut();
        RefMut::map(cell.borrow_mut(), |x| {
            x.as_mut().downcast_mut().unwrap_or_else(|| {
                unreachable!("`OccupiedEntry<T>` should only be constructed for valid `T`")
            })
        })
    }

    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry
    /// with a lifetime bound to the registry itself.
    ///
    /// If you need multiple references to the `OccupiedEntry`, see [`get_mut`].
    ///
    /// [`get_mut`]: Self::get_mut
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// use mahf::state::entry::Entry;
    /// # #[derive(Default, Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.entry::<A>().or_insert(A(12));
    ///
    /// assert_eq!(registry.get_value::<A>(), 12);
    /// if let Entry::Occupied(o) = registry.entry::<A>() {
    ///     o.into_mut().0 += 10;
    /// }
    ///
    /// assert_eq!(registry.get_value::<A>(), 22);
    /// ```
    #[inline]
    pub fn into_mut(self) -> RefMut<'a, T> {
        let cell = self.base.into_mut();
        RefMut::map(cell.borrow_mut(), |x| {
            x.as_mut().downcast_mut().unwrap_or_else(|| {
                unreachable!("`OccupiedEntry<T>` should only be constructed for valid `T`")
            })
        })
    }

    /// Sets the value of the entry, and returns the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// use mahf::state::entry::Entry;
    /// # #[derive(Default, Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.entry::<A>().or_insert(A(12));
    ///
    /// if let Entry::Occupied(mut o) = registry.entry::<A>() {
    ///     assert_eq!(o.insert(A(15)), A(12));
    /// }
    ///
    /// assert_eq!(registry.get_value::<A>(), 15);
    /// ```
    #[inline]
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

    /// Takes the value out of the entry, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// use mahf::state::entry::Entry;
    /// # #[derive(Default, Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.entry::<A>().or_insert(A(12));
    ///
    /// if let Entry::Occupied(o) =  registry.entry::<A>()  {
    ///     assert_eq!(o.remove(), A(12));
    /// }
    ///
    /// assert_eq!(registry.contains::<A>(), false);
    /// ```
    #[inline]
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
    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns a mutable reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// use mahf::state::entry::Entry;
    /// # #[derive(Default, Debug, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    ///
    /// if let Entry::Vacant(o) = registry.entry::<A>() {
    ///     o.insert(A(37));
    /// }
    /// assert_eq!(registry.get_value::<A>(), 37);
    /// ```
    #[inline]
    pub fn insert(self, value: T) -> RefMut<'a, T> {
        let cell = RefCell::new(Box::new(value));
        RefMut::map(self.base.insert(cell).borrow_mut(), |x| {
            x.as_mut()
                .downcast_mut()
                .unwrap_or_else(|| unreachable!("`T` should have been inserted before this call"))
        })
    }
}
