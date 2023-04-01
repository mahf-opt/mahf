use std::ops::{Deref, DerefMut};
use std::{any::TypeId, collections::HashSet};

use crate::{
    problems::Problem,
    state::{CustomState, State},
};

/// Allows borrowing multiple [CustomState]'s mutable from [State] at the same time.
/// It is meant to significantly simplify the definition of [Component][crate::framework::components::Component]'s
/// with multiple [CustomState]'s that are modified.
///
/// Implements most of the methods on [State], including convenience methods like [State::random_mut].
///
/// # Panics
///
/// Panics on accessing the same [CustomState] twice, on both mutable and immutable access.
/// References going out of scope do not change this behaviour.
///
/// The only exception to this rule are [get_value][MutState::get_value] and [set_value][MutState::set_value],
/// which can be called repeatedly using the same [CustomState], given that no reference to it already exists.
pub struct MutState<'a, 's, P> {
    state: &'a mut State<'s, P>,
    borrowed: HashSet<TypeId>,
}
impl<'a, 's, P: Problem> MutState<'a, 's, P> {
    pub(super) fn new(state: &'a mut State<'s, P>) -> Self {
        Self {
            state,
            borrowed: HashSet::new(),
        }
    }

    pub fn get_mut<T: CustomState<'s>>(&mut self) -> &'a mut T {
        let custom = self.state.get_mut::<T>() as *mut T;
        assert!(
            self.borrowed.insert(T::id()),
            "each type can only be borrowed once"
        );
        unsafe { &mut *custom }
    }

    pub fn has<T: CustomState<'s>>(&self) -> bool {
        self.state.has::<T>()
    }

    pub fn get<T: CustomState<'s>>(&mut self) -> &'a T {
        self.get_mut()
    }

    pub fn get_value<T>(&self) -> T::Target
    where
        T: CustomState<'s> + Deref,
        T::Target: Sized + Copy,
    {
        assert!(!self.borrowed.contains(&T::id()));
        self.state.get_value::<T>()
    }

    pub fn set_value<T>(&mut self, value: T::Target)
    where
        T: CustomState<'s> + DerefMut,
        T::Target: Sized,
    {
        assert!(!self.borrowed.contains(&T::id()));
        self.state.set_value::<T>(value);
    }

    pub fn get_value_mut<T>(&mut self) -> &'a mut T::Target
    where
        T: CustomState<'s> + DerefMut,
        T::Target: Sized,
    {
        self.get_mut::<T>().deref_mut()
    }
}

/// Allows borrowing up to eight [CustomState]'s mutable from [State] at the same time.
///
/// This trait is implemented for type tuples with size up to eight.
///
/// # Panics
///
/// Panics on type duplicates in the tuple.
pub trait MultiStateTuple<'a, 's>: 'a {
    type References: 'a;

    fn validate() -> bool;

    #[track_caller]
    fn fetch<P: Problem>(state: &'a mut State<'s, P>) -> Self::References;
}

macro_rules! impl_multi_state_tuple {
    (($($item:ident),+)) => {
        impl<'a, 's, $($item: 'a),*> MultiStateTuple<'a, 's> for ($($item),*)
        where
            $($item: CustomState<'s>),*
        {
            type References = ($(&'a mut $item),*);

            fn validate() -> bool {
                let mut set = HashSet::new();
                $(set.insert($item::id()))&&*
            }

            fn fetch<P: Problem>(state: &'a mut State<'s, P>) -> Self::References {
                assert!(Self::validate(), "each type can only be borrowed once");

                let state = state as *mut State<P>;
                unsafe { ($((*state).get_mut::<$item>()),*) }
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
