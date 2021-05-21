use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct CustomState {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl CustomState {
    pub(crate) fn new() -> Self {
        CustomState {
            map: HashMap::new(),
        }
    }

    pub fn insert<T: Any>(&mut self, state: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(state));
    }

    pub fn has<T: Any>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    pub fn get<T: Any>(&self) -> &T {
        self.map
            .get(&TypeId::of::<T>())
            .unwrap()
            .downcast_ref()
            .unwrap()
    }

    pub fn get_mut<T: Any>(&mut self) -> &mut T {
        self.map
            .get_mut(&TypeId::of::<T>())
            .unwrap()
            .downcast_mut()
            .unwrap()
    }
}
