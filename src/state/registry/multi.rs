use std::collections::HashSet;

use crate::{state::registry::StateRegistry, CustomState, StateError};

/// Allows borrowing up to eight `&mut T: `[`CustomState`] from a [`StateRegistry`] at the same time.
///
/// # Usage
///
/// This trait should only used indirectly through [`StateRegistry::get_multiple_mut`] and
/// [`StateRegistry::try_get_multiple_mut`].
///
/// Note that this makes it possible to retrieve mutable references `&mut T` directly,
/// while [`StateRegistry::borrow`] and similar return a [`RefMut<T>`].
///
/// A single `&mut T` can be retrieved using [`StateRegistry::get_mut`].
///
/// [`RefMut<T>`]: std::cell::RefMut
///
/// # Implementation
///
/// This trait is implemented for type tuples with size up to eight.
/// Own implementation is usually not necessary.
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
/// # #[derive(Debug, Deref, DerefMut, Tid)]
/// # pub struct B(usize);
/// # impl CustomState<'_> for B {}
///
/// let mut registry = StateRegistry::new();
/// registry.insert(A(10));
/// registry.insert(B(20));
///
/// // References are borrow-checked at compile time.
/// let (a, b): (&mut A, &mut B) = registry.get_multiple_mut::<(A, B)>();
/// a.0 += 1;
/// b.0 += 1;
///
/// assert_eq!(registry.get_value::<A>(), 11);
/// assert_eq!(registry.get_value::<B>(), 21);
///
/// // Borrowing the same type multiple times at the same time is not possible.
/// assert!(registry.try_get_multiple_mut::<(A, A)>().is_err());
/// ```
pub trait MultiStateTuple<'a, 'b>: 'a {
    /// A collection of types to borrow mutable at the same time.
    type References: 'a;

    /// Checks if the types within `References` are distinct.
    fn distinct() -> bool;

    /// Tries to borrow all types contained within `References` mutably.
    ///
    /// The soundness if `References` should be checked with [`distinct`] before any borrow occurs.
    ///
    /// [`distinct`]: Self::distinct
    fn try_get_mut(state: &'a mut StateRegistry<'b>) -> Result<Self::References, StateError>;
}

macro_rules! impl_multi_state_tuple {
    (($($item:ident),+)) => {
        impl<'a, 'b, $($item: 'a),*> MultiStateTuple<'a, 'b> for ($($item),*)
        where
            $($item: CustomState<'b>),*
        {
            type References = ($(&'a mut $item),*);

            fn distinct() -> bool {
                let mut set = HashSet::new();
                $(set.insert($item::id()))&&*
            }

            fn try_get_mut(state: &'a mut StateRegistry<'b>) -> Result<Self::References, StateError> {
                if !Self::distinct() {
                    return Err(StateError::multiple_borrow_conflict::<Self::References>())
                }

                let state = state as *mut StateRegistry;
                // SAFETY: All type ids were checked to be distinct beforehand.
                // The registry is mutably borrowed, so no new mutable references
                // to already borrowed types are possible until all references are dropped.
                unsafe { Ok(($((*state).get_mut::<$item>().ok_or_else(StateError::not_found::<$item>)?),*)) }
            }
        }
    };
}

impl_multi_state_tuple!((T1, T2));
impl_multi_state_tuple!((T1, T2, T3));
impl_multi_state_tuple!((T1, T2, T3, T4));
impl_multi_state_tuple!((T1, T2, T3, T4, T5));
impl_multi_state_tuple!((T1, T2, T3, T4, T5, T6));
impl_multi_state_tuple!((T1, T2, T3, T4, T5, T6, T7));
impl_multi_state_tuple!((T1, T2, T3, T4, T5, T6, T7, T8));
// impl_multi_state_tuple!((T1, T2, T3, T4, T5, T6, T7, T8, T9));
// impl_multi_state_tuple!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10));
// impl_multi_state_tuple!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11));
// impl_multi_state_tuple!((T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12));

#[cfg(test)]
mod tests {
    use better_any::{Tid, TidAble};

    use crate::{state::registry::MultiStateTuple, CustomState};

    macro_rules! make_type {
        ($ty:ident) => {
            #[derive(Tid)]
            struct $ty;
            impl CustomState<'_> for $ty {}
        };
    }

    make_type!(T1);
    make_type!(T2);
    make_type!(T3);
    make_type!(T4);

    #[test]
    fn distinct_returns_true_for_distinct() {
        assert!(<(T1, T2) as MultiStateTuple>::distinct());
        assert!(<(T1, T2, T3) as MultiStateTuple>::distinct());
        assert!(<(T1, T2, T3, T4) as MultiStateTuple>::distinct());
    }

    #[test]
    fn distinct_returns_false_for_non_distinct() {
        assert!(!<(T1, T1) as MultiStateTuple>::distinct());
        assert!(!<(T1, T2, T1) as MultiStateTuple>::distinct());
        assert!(!<(T1, T2, T3, T4, T1) as MultiStateTuple>::distinct());
        assert!(!<(T1, T2, T2, T1) as MultiStateTuple>::distinct());
    }
}
