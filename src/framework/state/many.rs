use std::ops::{Deref, DerefMut};
use std::{any::TypeId, collections::HashSet};

use crate::framework::{CustomState, State};

pub struct MutState<'a> {
    state: &'a mut State,
    borrowed: HashSet<TypeId>,
}
impl<'a> MutState<'a> {
    pub fn new(state: &'a mut State) -> Self {
        Self {
            state,
            borrowed: HashSet::new(),
        }
    }

    pub fn get_mut<T: CustomState>(&mut self) -> &'a mut T {
        let custom = self.state.get_mut::<T>() as *mut T;
        assert!(
            self.borrowed.insert(TypeId::of::<T>()),
            "each type can only be borrowed once"
        );
        unsafe { &mut *custom }
    }

    pub fn has<T: CustomState>(&self) -> bool {
        self.state.has::<T>()
    }

    pub fn get<T: CustomState>(&mut self) -> &'a T {
        self.get_mut()
    }

    pub fn get_value<T>(&mut self) -> T::Target
    where
        T: CustomState + Deref,
        T::Target: Sized + Copy,
    {
        assert!(!self.borrowed.contains(&TypeId::of::<T>()));
        self.state.get_value::<T>()
    }

    pub fn set_value<T>(&mut self, value: T::Target)
    where
        T: CustomState + DerefMut,
        T::Target: Sized,
    {
        assert!(!self.borrowed.contains(&TypeId::of::<T>()));
        self.state.set_value::<T>(value);
    }

    pub fn get_value_mut<T>(&mut self) -> &'a mut T::Target
    where
        T: CustomState + DerefMut,
        T::Target: Sized,
    {
        self.get_mut::<T>().deref_mut()
    }
}

pub trait MultiStateTuple<'a>: 'a {
    type References: 'a;

    fn validate() -> bool;

    #[track_caller]
    fn fetch(state: &'a mut State) -> Self::References;
}

macro_rules! impl_multi_state_tuple {
    (($($item:ident),+)) => {
        impl<'a, $($item),*> MultiStateTuple<'a> for ($($item),*)
        where
            $($item: CustomState),*
        {
            type References = ($(&'a mut $item),*);

            fn validate() -> bool {
                let mut set = HashSet::new();
                $(set.insert(TypeId::of::<$item>()))&&*
            }

            fn fetch(state: &'a mut State) -> Self::References {
                assert!(Self::validate(), "each type can only be borrowed once");

                let state = state as *mut State;
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
