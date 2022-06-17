use std::{any::TypeId, collections::HashSet};

use crate::framework::{CustomState, State};

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
