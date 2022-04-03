use crate::tracking::log::CustomLog;
use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
};

/// Makes custom state trackable.
pub trait CustomState: AsAny {
    /// Called after each evaluation.
    fn evaluation_log(&self) -> Vec<CustomLog> {
        Vec::default()
    }

    /// Called after each iteration.
    fn iteration_log(&self) -> Vec<CustomLog> {
        Vec::default()
    }
}

/// Utility trait to upcast [CustomState] to [Any].
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
pub struct CustomStateMap {
    map: BTreeMap<TypeId, Box<dyn CustomState>>,
}

impl CustomStateMap {
    pub(crate) fn new() -> Self {
        CustomStateMap {
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

    pub(crate) fn collect_evaluation_log(&self) -> Vec<CustomLog> {
        let mut entries = Vec::new();
        for state in self.map.values() {
            entries.append(&mut state.evaluation_log());
        }
        entries
    }

    pub(crate) fn collect_iteration_log(&self) -> Vec<CustomLog> {
        let mut entries = Vec::new();
        for state in self.map.values() {
            entries.append(&mut state.iteration_log());
        }
        entries
    }
}
