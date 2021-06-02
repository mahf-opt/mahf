use crate::tracking::CustomLog;
use std::any::{Any, TypeId};
use std::collections::BTreeMap;

pub trait CustomState: AsAny {
    fn evaluation_log(&self) -> Vec<CustomLog> {
        Vec::default()
    }

    fn iteration_log(&self) -> Vec<CustomLog> {
        Vec::default()
    }
}

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

    pub fn collect_evaluation_log(&self) -> Vec<CustomLog> {
        let mut entries = Vec::new();
        for state in self.map.values() {
            entries.append(&mut state.evaluation_log());
        }
        entries
    }

    pub fn collect_iteration_log(&self) -> Vec<CustomLog> {
        let mut entries = Vec::new();
        for state in self.map.values() {
            entries.append(&mut state.iteration_log());
        }
        entries
    }
}
