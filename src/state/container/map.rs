use std::{any::TypeId, collections::BTreeMap};

use better_any::TidExt;

use crate::state::CustomState;

/// Stores custom state.
///
/// Each custom state should be a [new type].
/// The states type also doubles as index in the map.
/// Custom state can be of any type as long as it implements the [CustomState] trait.
///
/// [new type]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
#[derive(Default)]
pub struct StateMap<'a> {
    map: BTreeMap<TypeId, Box<dyn CustomState<'a>>>,
}

impl<'a> StateMap<'a> {
    pub(crate) fn new() -> Self {
        StateMap {
            map: BTreeMap::new(),
        }
    }

    pub fn insert<T: CustomState<'a>>(&mut self, state: T) {
        self.map.insert(T::id(), Box::new(state));
    }

    pub fn has<T: CustomState<'a>>(&self) -> bool {
        self.map.contains_key(&T::id())
    }

    pub fn get<T: CustomState<'a>>(&self) -> &T {
        self.map
            .get(&T::id())
            .unwrap()
            .as_ref()
            .downcast_ref()
            .unwrap()
    }

    pub fn get_mut<T: CustomState<'a>>(&mut self) -> &mut T {
        self.map
            .get_mut(&T::id())
            .unwrap()
            .as_mut()
            .downcast_mut()
            .unwrap()
    }

    pub fn get_or_insert_default<T: CustomState<'a> + Default>(&mut self) -> &mut T {
        self.map
            .entry(T::id())
            .or_insert_with(|| Box::new(T::default()))
            .as_mut()
            .downcast_mut()
            .unwrap()
    }
}
