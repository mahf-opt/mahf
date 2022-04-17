mod custom;
pub use custom::{CustomState, CustomStateMap};

pub mod common;

#[derive(Default)]
pub struct StateTree {
    parent: Option<Box<StateTree>>,
    map: CustomStateMap,
}

impl StateTree {
    pub fn new_root() -> Self {
        StateTree {
            parent: None,
            map: CustomStateMap::new(),
        }
    }

    pub fn with_substate(&mut self, fun: impl FnOnce(&mut StateTree)) {
        let mut substate: StateTree = Self {
            parent: Some(Box::new(std::mem::take(self))),
            map: CustomStateMap::new(),
        };
        fun(&mut substate);
        *self = *substate.parent.unwrap()
    }

    pub fn parent(&self) -> Option<&Self> {
        self.parent.as_deref()
    }

    pub fn parent_mut(&mut self) -> Option<&mut Self> {
        self.parent.as_deref_mut()
    }

    pub fn insert<T: CustomState>(&mut self, state: T) {
        self.map.insert(state);
    }

    pub fn has<T: CustomState>(&self) -> bool {
        self.map.has::<T>() || self.parent().map(|p| p.has::<T>()).unwrap_or_default()
    }

    pub fn get<T: CustomState>(&self) -> &T {
        if self.map.has::<T>() {
            self.map.get::<T>()
        } else {
            self.parent().unwrap().get::<T>()
        }
    }

    pub fn get_mut<T: CustomState>(&mut self) -> &mut T {
        if self.map.has::<T>() {
            self.map.get_mut::<T>()
        } else {
            self.parent_mut().unwrap().get_mut::<T>()
        }
    }
}
