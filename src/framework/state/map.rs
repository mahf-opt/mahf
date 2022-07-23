use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
};

use crate::framework::state::CustomState;

/// Utility trait to upcast [CustomState](CustomState) to [Any].
pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}
impl<S: CustomState> AsAny for S {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// Stores custom state.
///
/// Each custom state should be a [new type].
/// The states type also doubles as index in the map.
/// Custom state can be of any type as long as it implements the [CustomState] trait.
///
/// [new type]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
#[derive(Default)]
pub struct StateMap {
    map: BTreeMap<TypeId, Box<dyn CustomState>>,
}

impl StateMap {
    pub(crate) fn new() -> Self {
        StateMap {
            map: BTreeMap::new(),
        }
    }

    pub fn insert<T: CustomState>(&mut self, state: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(state));
    }

    pub fn has<T: CustomState>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    pub fn get<T: CustomState>(&self) -> &T {
        self.map
            .get(&TypeId::of::<T>())
            .unwrap()
            .as_any()
            .downcast_ref()
            .unwrap()
    }

    pub fn get_mut<T: CustomState>(&mut self) -> &mut T {
        self.map
            .get_mut(&TypeId::of::<T>())
            .unwrap()
            .as_mut_any()
            .downcast_mut()
            .unwrap()
    }

    pub fn get_or_insert_default<T: CustomState + Default>(&mut self) -> &mut T {
        self.map
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(T::default()))
            .as_mut_any()
            .downcast_mut()
            .unwrap()
    }
}
