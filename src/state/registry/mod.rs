//! A registry for arbitrary state.

use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use better_any::TidExt;

mod custom;
pub mod entry;
pub mod error;
pub mod multi;

pub use custom::CustomState;
pub use entry::{Entry, OccupiedEntry, VacantEntry};
pub use error::{StateError, StateResult};
pub use multi::MultiStateTuple;

pub type StateMap<'a> = HashMap<TypeId, RefCell<Box<dyn CustomState<'a>>>>;

/// A [`CustomState`] container, which provides methods to insert, access and manage the
/// contained custom state.
///
/// It is essentially a set of types, where distinct types can be accessed simultaneously using
/// the usual borrowing rules of Rust (multiple reads xor one write).
///
/// Note that multiple mutable borrows of distinct state is possible through interior mutability.
///
///
/// # Stack
///
/// The registry allows shadowing state though the use of an internal owned stack of state registries.
///
/// See [`insert`] for more information.
///
/// [`insert`]: Self::insert
#[derive(Default)]
pub struct StateRegistry<'a> {
    parent: Option<Box<StateRegistry<'a>>>,
    map: StateMap<'a>,
}

impl<'a> StateRegistry<'a> {
    /// Creates an empty `StateRegistry`.
    ///
    /// The registry is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use mahf::StateRegistry;
    /// let mut registry = StateRegistry::new();
    /// ```
    pub fn new() -> Self {
        Self {
            parent: None,
            map: HashMap::new(),
        }
    }

    /// Returns the parent registry, and `None` if this is the only registry on the stack.
    pub fn parent(&self) -> Option<&Self> {
        self.parent.as_deref()
    }

    /// Returns the parent registry mutable, and `None` if this is the only registry on the stack.
    pub fn parent_mut(&mut self) -> Option<&mut Self> {
        self.parent.as_deref_mut()
    }

    /// Pushes a new registry on the stack and returns it, taking ownership of the old registry.
    pub fn into_child(self) -> Self {
        Self {
            parent: Some(Box::new(self)),
            map: HashMap::new(),
        }
    }

    /// Pops the current registry from the stack and returns it along with the parent registry.
    pub fn into_parent(self) -> (Option<Self>, StateMap<'a>) {
        (self.parent.map(|parent| *parent), self.map)
    }

    /// Returns a reference to the first registry which contains `T`.
    pub fn find<T>(&self) -> StateResult<&Self>
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

    /// Returns a mutable reference to the first registry which contains `T`.
    pub fn find_mut<T>(&mut self) -> StateResult<&mut Self>
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

    /// Gets the entry of `T` in the registry for in-place manipulation.
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

    /// Inserts state into the top-most registry on the stack.
    ///
    /// If the registry did not have this state present, `None` is returned.
    ///
    /// If the registry did have this type present, the state is updated, and the old
    /// state is returned.
    ///
    /// # Shadowing state
    ///
    /// Note that `T` is inserted into the top-most registry on the stack.
    /// This means that any `T` that may be present in any registry below is now
    /// *shadowed* by the newly inserted `t`, and accessing `T` using [`borrow`] etc.
    /// will yield `t` until it is explicitly removed using [`remove`], or the
    /// entire registry is popped using [`into_parent`].
    ///
    /// [`borrow`]: Self::borrow
    /// [`remove`]: Self::remove
    /// [`into_parent`]: Self::into_parent
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// assert_eq!(registry.insert(A(10)), None);
    /// assert!(registry.contains::<A>());
    ///
    /// registry.insert(A(20));
    /// assert_eq!(registry.insert(A(30)), Some(A(20)));
    /// assert_eq!(registry.get_value::<A>(), 30);
    /// ```
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

    /// Removes the state of type `T` from the first registry which contains `T` and returns its
    /// ownership to the caller. In case there is no such state in any
    /// registry, an `Err` will be returned.
    ///
    /// Use this method with caution; in general, there is no good reason
    /// to remove state from the registry, other than having explicitly shadowed existing
    /// state beforehand, as other components might assume this state still exists.
    ///
    /// Thus, only use this if you're sure that:
    /// - the `T` you removed has shadowed other `T` (see [`insert`]),
    /// - no component will try to access this state after you removed it,
    /// - components that access are designed to handle missing state.
    ///
    /// [`insert`]: Self::insert
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// assert_eq!(registry.remove::<A>().unwrap(), A(10));
    /// assert!(registry.remove::<A>().is_err());
    /// ```
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

    /// Removes the state of type `T` from the first registry which contains `T` and returns its
    /// ownership to the caller.
    ///
    /// Use this method with caution; in general, there is no good reason
    /// to remove state from the registry, other than having explicitly shadowed existing
    /// state beforehand, as other components might assume this state still exists.
    ///
    /// Thus, only use this if you're sure that:
    /// - the `T` you removed has shadowed other `T` (see [`insert`]),
    /// - no component will try to access this state after you removed it.
    ///
    /// For a non-panicking version, see [`remove`].
    ///
    /// [`insert`]: Self::insert
    /// [`remove`]: Self::remove
    ///
    ///
    /// # Panics
    ///
    /// Panics when `T` is not present in the registry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// assert_eq!(registry.take::<A>(), A(10));
    /// // registry.remove::<A>() panics.
    /// ```
    #[track_caller]
    pub fn take<T>(&mut self) -> T
    where
        T: CustomState<'a>,
    {
        self.remove().unwrap_or_else(StateError::panic)
    }

    /// Returns `true` if the current registry contains state of type `T`.
    ///
    /// Note that this method only searches the top-most registry on the stack.
    /// For searching the entire stack, see [`has`].
    ///
    /// [`has`]: Self::has
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// assert!(registry.contains::<A>());
    ///
    /// let mut registry = registry.into_child();
    /// // Only the parent registry contains `A`.
    /// assert!(!registry.contains::<A>());
    ///
    /// let (parent, _) = registry.into_parent();
    /// let registry = parent.unwrap();
    /// assert!(registry.contains::<A>())
    /// ```
    pub fn contains<T>(&self) -> bool
    where
        T: CustomState<'a>,
    {
        self.map.contains_key(&T::id())
    }

    /// Returns `true` if any registry on the stack contains state of type `T`.
    ///
    /// Note that this method searches all registries on the stack.
    /// For searching only the top-most registry, see [`contains`].
    ///
    /// [`contains`]: Self::contains
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// assert!(registry.has::<A>());
    ///
    /// let mut registry = registry.into_child();
    /// // `has` also checks any parent registries.
    /// assert!(registry.has::<A>());
    /// ```
    pub fn has<T>(&self) -> bool
    where
        T: CustomState<'a>,
    {
        self.find::<T>().is_ok()
    }

    /// Returns a reference to the first state `T` in the registry.
    ///
    /// For a non-panicking version, see [`try_borrow`].
    ///
    /// [`try_borrow`]: Self::try_borrow
    ///
    /// # Panics
    ///
    /// Panics if the state doesn't exist or if the state is already being accessed mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// assert_eq!(&*registry.borrow::<A>(), &A(10));
    /// ```
    #[track_caller]
    pub fn borrow<T>(&self) -> Ref<'_, T>
    where
        T: CustomState<'a>,
    {
        self.try_borrow::<T>().unwrap_or_else(StateError::panic)
    }

    /// Returns a reference to the first state `T` in the registry if it exists and is not
    /// currently mutably borrowed, and `Err` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// assert_eq!(&*registry.try_borrow::<A>().unwrap(), &A(10));
    ///
    /// let mut_a = registry.borrow_mut::<A>();
    /// // Trying to access `A` while it is mutably borrowed results in an `Err`.
    /// assert!(registry.try_borrow::<A>().is_err());
    ///
    /// // Note that `mut_a` must be explicitly dropped to end the mutable borrow.
    /// drop(mut_a);
    /// assert!(registry.try_borrow::<A>().is_ok());
    /// ```
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

    /// Returns a mutable reference to the first state `T` in the registry.
    ///
    /// For a non-panicking version, see [`try_borrow_mut`].
    ///
    /// [`try_borrow_mut`]: Self::try_borrow_mut
    ///
    /// # Panics
    ///
    /// Panics if the state doesn't exist or if the state is already being accessed mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// registry.borrow_mut::<A>().0 += 1;
    /// assert_eq!(registry.get_value::<A>(), 11);
    /// ```
    #[track_caller]
    pub fn borrow_mut<T>(&self) -> RefMut<'_, T>
    where
        T: CustomState<'a>,
    {
        self.try_borrow_mut::<T>().unwrap_or_else(StateError::panic)
    }

    /// Returns a mutable reference to the first state `T` in the registry if it exists and is not
    /// currently mutably borrowed, and `Err` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// # fn main() -> mahf::state::StateResult<()> {
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// registry.try_borrow_mut::<A>()?.0 += 1;
    /// assert_eq!(registry.get_value::<A>(), 11);
    ///
    /// let mut_a = registry.borrow_mut::<A>();
    /// // Trying to access `A` while it is mutably borrowed results in an `Err`.
    /// assert!(registry.try_borrow::<A>().is_err());
    /// assert!(registry.try_borrow_mut::<A>().is_err());
    ///
    /// // Note that `mut_a` must be explicitly dropped to end the mutable borrow.
    /// drop(mut_a);
    /// assert!(registry.try_borrow::<A>().is_ok());
    /// assert!(registry.try_borrow_mut::<A>().is_ok());
    /// # Ok(())
    /// # }
    /// ```
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

    /// Returns the cloned `T::Target` of the first state `T` in the registry.
    ///
    /// This is especially handy for state which derefs to some primitive like `u32` or `f64`.
    ///
    /// For a non-panicking version, see [`try_get_value`].
    ///
    /// [`try_get_value`]: Self::try_get_value
    ///
    /// # Panics
    ///
    /// Panics if the state doesn't exist or if the state is already being accessed mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// assert_eq!(registry.get_value::<A>(), 10);
    /// ```
    #[track_caller]
    pub fn get_value<T>(&self) -> T::Target
    where
        T: CustomState<'a> + Deref,
        T::Target: Sized + Clone,
    {
        self.try_get_value::<T>().unwrap_or_else(StateError::panic)
    }

    /// Returns the cloned `T::Target` of the first state `T` in the registry if it exists and is not
    /// currently mutably borrowed, and `Err` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// assert_eq!(registry.try_get_value::<A>().unwrap(), 10);
    ///
    /// let mut_a = registry.borrow_mut::<A>();
    /// // Trying to access `A` while it is mutably borrowed results in an `Err`.
    /// assert!(registry.try_get_value::<A>().is_err());
    ///
    /// // Note that `mut_a` must be explicitly dropped to end the mutable borrow.
    /// drop(mut_a);
    /// assert!(registry.try_get_value::<A>().is_ok());
    /// ```
    pub fn try_get_value<T>(&self) -> StateResult<T::Target>
    where
        T: CustomState<'a> + Deref,
        T::Target: Sized + Clone,
    {
        self.try_borrow::<T>().map(|t| t.clone())
    }

    /// Returns a reference to `T::Target` of the first state `T` in the registry.
    ///
    /// For a non-panicking version, see [`try_borrow_value`].
    ///
    /// [`try_borrow_value`]: Self::try_borrow_value
    ///
    /// # Panics
    ///
    /// Panics if the state doesn't exist or if the state is already being accessed mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// assert_eq!(*registry.borrow_value::<A>(), 10);
    /// ```
    #[track_caller]
    pub fn borrow_value<T>(&self) -> Ref<T::Target>
    where
        T: CustomState<'a> + Deref,
        T::Target: Sized,
    {
        self.try_borrow_value::<T>()
            .unwrap_or_else(StateError::panic)
    }

    /// Returns a reference to `T::Target` of the first state `T` in the registry if it exists and is not
    /// currently mutably borrowed, and `Err` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// assert_eq!(*registry.try_borrow_value::<A>().unwrap(), 10);
    ///
    /// let mut_a = registry.borrow_mut::<A>();
    /// // Trying to access `A` while it is mutably borrowed results in an `Err`.
    /// assert!(registry.try_borrow_value::<A>().is_err());
    ///
    /// // Note that `mut_a` must be explicitly dropped to end the mutable borrow.
    /// drop(mut_a);
    /// assert!(registry.try_borrow_value::<A>().is_ok());
    /// ```
    pub fn try_borrow_value<T>(&self) -> StateResult<Ref<T::Target>>
    where
        T: CustomState<'a> + Deref,
        T::Target: Sized,
    {
        let r = self.try_borrow::<T>()?;
        Ok(Ref::map(r, |x| x.deref()))
    }

    /// Returns a mutable reference to `T::Target` of the first state `T` in the registry.
    ///
    /// For a non-panicking version, see [`try_borrow_value_mut`].
    ///
    /// [`try_borrow_value_mut`]: Self::try_borrow_value_mut
    ///
    /// # Panics
    ///
    /// Panics if the state doesn't exist or if the state is already being accessed mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// *registry.borrow_value_mut::<A>() += 1;
    /// assert_eq!(registry.get_value::<A>(), 11);
    /// ```
    #[track_caller]
    pub fn borrow_value_mut<T>(&self) -> RefMut<T::Target>
    where
        T: CustomState<'a> + DerefMut,
        T::Target: Sized,
    {
        self.try_borrow_value_mut::<T>()
            .unwrap_or_else(StateError::panic)
    }

    /// Returns a mutable reference to `T::Target` of the first state `T` in the registry if it exists and is not
    /// currently mutably borrowed, and `Err` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// # fn main() -> mahf::state::StateResult<()> {
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// *registry.try_borrow_value_mut::<A>()? += 1;
    /// assert_eq!(registry.get_value::<A>(), 11);
    ///
    /// let mut_a = registry.borrow_mut::<A>();
    /// // Trying to access `A` while it is mutably borrowed results in an `Err`.
    /// assert!(registry.try_borrow_value::<A>().is_err());
    /// assert!(registry.try_borrow_value_mut::<A>().is_err());
    ///
    /// // Note that `mut_a` must be explicitly dropped to end the mutable borrow.
    /// drop(mut_a);
    /// assert!(registry.try_borrow_value::<A>().is_ok());
    /// assert!(registry.try_borrow_value_mut::<A>().is_ok());
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_borrow_value_mut<T>(&self) -> StateResult<RefMut<T::Target>>
    where
        T: CustomState<'a> + DerefMut,
        T::Target: Sized,
    {
        let r = self.try_borrow_mut::<T>()?;
        Ok(RefMut::map(r, |x| x.deref_mut()))
    }

    /// Sets the `T::Target` of the first state `T` in the registry using `value`, and returns
    /// the previous `T::Target` if it exists, and `None` otherwise.
    ///
    /// This is especially handy for state which derefs to some primitive like `u32` or `f64`.
    ///
    /// # Missing `T`
    ///
    /// Note that if `T` is not present, it is **not** inserted by this method and `value` is discarded.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// assert_eq!(registry.set_value::<A>(20), None);
    /// // `A` is **not** inserted here.
    /// assert!(!registry.contains::<A>());
    ///
    /// registry.insert(A(10));
    /// assert_eq!(registry.set_value::<A>(20), Some(10));
    /// assert_eq!(registry.get_value::<A>(), 20);
    /// ```
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

    /// Returns a mutable reference to the first state `T` in the registry, borrowing the
    /// registry mutably.
    ///
    /// Note that method is only useful if only one state needs to be accessed at a time
    /// because of the mutable borrow.
    ///
    /// If multiple states need to be accessed, see [`borrow`] and similar or [`get_multiple_mut`].
    ///
    /// [`borrow`]: Self::borrow
    /// [`get_multiple_mut`]: Self::get_multiple_mut
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    ///
    /// let a: &mut A = registry.get_mut::<A>().unwrap();
    /// a.0 += 1;
    /// // `registry` is mutably borrowed, the following is a compile-time error:
    /// // registry.try_borrow::<B>()
    /// a.0 += 1;
    ///
    /// // `a` is not used anymore after this, so `registry` can be borrowed again.
    /// assert_eq!(registry.get_value::<A>(), 12);
    /// ```
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

    /// Returns mutable references to the first states `T` in the registry, borrowing the
    /// registry mutably.
    ///
    /// `T` can be any tuple of `(T1, T2, ...)` up to size eight.
    ///
    /// Note that `T` needs to include all states that should be read or written to, as
    /// the registry is mutably borrowed.
    ///
    /// For interleaved borrows, see [`borrow`] and similar.
    ///
    /// For accessing only exactly one single state mutably, see [`get_mut`].
    ///
    /// For a non-panicking version, see [`try_get_multiple_mut`].
    ///
    /// [`borrow`]: Self::borrow
    /// [`get_mut`]: Self::get_mut
    /// [`try_get_multiple_mut`]: Self::try_get_multiple_mut
    ///
    /// # Panics
    ///
    /// Panics when `T` contains type duplicates, e.g. `(T1, T2, T1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct B(usize);
    /// # impl CustomState<'_> for B {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// registry.insert(B(20));
    ///
    /// let (a, b): (&mut A, &mut B) = registry.get_multiple_mut::<(A, B)>();
    /// a.0 += 1;
    /// b.0 += 1;
    /// // `registry` is mutably borrowed, the following is a compile-time error:
    /// // registry.try_borrow::<B>()
    /// a.0 += 1;
    /// b.0 += 1;
    ///
    /// // `a` and `b` are not used anymore after this, so `registry` can be borrowed again.
    /// assert_eq!(registry.get_value::<A>(), 12);
    /// assert_eq!(registry.get_value::<B>(), 22);
    ///
    /// // registry.get_multiple_mut::<(A, A)>() panics because of type duplicates.
    /// ```
    #[track_caller]
    pub fn get_multiple_mut<'b, T: MultiStateTuple<'b, 'a>>(&'b mut self) -> T::References {
        self.try_get_multiple_mut::<T>()
            .unwrap_or_else(StateError::panic)
    }

    /// Returns mutable references to the first states `T` in the registry, borrowing the
    /// registry mutably when `T` contains only distinct types, and `Err` otherwise.
    ///
    /// `T` can be any tuple of `(T1, T2, ...)` up to size eight.
    ///
    /// Note that `T` needs to include all states that should be read or written to, as
    /// the registry is mutably borrowed.
    ///
    /// For interleaved borrows, see [`borrow`] and similar.
    ///
    /// For accessing only exactly one single state mutably, see [`get_mut`].
    ///
    /// [`borrow`]: Self::borrow
    /// [`get_mut`]: Self::get_mut
    ///
    /// # Examples
    ///
    /// ```
    /// # use better_any::{Tid, TidAble};
    /// # use derive_more::{Deref, DerefMut};
    /// # use mahf::CustomState;
    /// use mahf::StateRegistry;
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct A(usize);
    /// # impl CustomState<'_> for A {}
    /// # #[derive(Debug, PartialEq, Deref, DerefMut, Tid)]
    /// # pub struct B(usize);
    /// # impl CustomState<'_> for B {}
    ///
    /// let mut registry = StateRegistry::new();
    /// registry.insert(A(10));
    /// registry.insert(B(20));
    ///
    /// let (a, b): (&mut A, &mut B) = registry.try_get_multiple_mut::<(A, B)>().unwrap();
    /// a.0 += 1;
    /// b.0 += 1;
    /// // `registry` is mutably borrowed, the following is a compile-time error:
    /// // registry.try_borrow::<B>()
    /// a.0 += 1;
    /// b.0 += 1;
    ///
    /// // `a` and `b` are not used anymore after this, so `registry` can be borrowed again.
    /// assert_eq!(registry.get_value::<A>(), 12);
    /// assert_eq!(registry.get_value::<B>(), 22);
    ///
    /// // The following errors because of type duplicates:
    /// assert!(registry.try_get_multiple_mut::<(A, A)>().is_err());
    /// ```
    pub fn try_get_multiple_mut<'b, T: MultiStateTuple<'b, 'a>>(
        &'b mut self,
    ) -> StateResult<T::References> {
        T::try_get_mut(self)
    }
}
