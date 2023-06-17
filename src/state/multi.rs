use std::collections::HashSet;

use crate::{state::StateRegistry, CustomState, StateError};

/// Allows borrowing up to eight `&mut T: `[`CustomState`] from [State] at the same time.
///
/// Note that this makes it possible to retrieve mutable references `&mut T` directly,
/// while [`StateRegistry::borrow`] and similar return a [`RefMut<T>`].
///
/// [`RefMut<T>`]: std::cell::RefMut
///
/// This trait is implemented for type tuples with size up to eight.
///
/// # Panics
///
/// Panics on type duplicates in the tuple.
///
/// # Examples
///
/// TODO
pub trait MultiStateTuple<'a, 'b>: 'a {
    type References: 'a;

    fn distinct() -> bool;

    fn try_fetch(state: &'a mut StateRegistry<'b>) -> Result<Self::References, StateError>;
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

            fn try_fetch(state: &'a mut StateRegistry<'b>) -> Result<Self::References, StateError> {
                if !Self::distinct() {
                    return Err(StateError::multiple_borrow_conflict::<Self::References>())
                }

                let state = state as *mut StateRegistry;
                // SAFETY: TODO
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

    use crate::{state::multi::MultiStateTuple, CustomState};

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
