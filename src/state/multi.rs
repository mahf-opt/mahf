use std::collections::HashSet;

use crate::{state::StateRegistry, CustomState, StateError};

pub trait MultiStateTuple<'a, 'b>: 'a {
    type References: 'a;

    fn validate() -> bool;

    fn try_fetch(state: &'a mut StateRegistry<'b>) -> Result<Self::References, StateError>;
}

macro_rules! impl_multi_state_tuple {
    (($($item:ident),+)) => {
        impl<'a, 'b, $($item: 'a),*> MultiStateTuple<'a, 'b> for ($($item),*)
        where
            $($item: CustomState<'b>),*
        {
            type References = ($(&'a mut $item),*);

            fn validate() -> bool {
                let mut set = HashSet::new();
                $(set.insert($item::id()))&&*
            }

            fn try_fetch(state: &'a mut StateRegistry<'b>) -> Result<Self::References, StateError> {
                if !Self::validate() {
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
