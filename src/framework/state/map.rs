use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
};

use splitmut::{GetMuts, SplitMut};
use crate::framework::CustomState;

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

/// Wrapper struct for [splitmut::GetMuts].
/// Currently only possible by leaking the [TypeId].
#[allow(dead_code)]
pub struct GetMutStatesOld<'a>(
    GetMuts<'a, &'static TypeId, Box<dyn CustomState>, BTreeMap<TypeId, Box<dyn CustomState>>>,
);
impl<'a> GetMutStatesOld<'a> {
    pub fn get_mut<T: CustomState>(&mut self) -> &'a mut T {
        let result = self.0.at(Box::leak(Box::new(TypeId::of::<T>()))).unwrap();
        result.as_mut_any().downcast_mut().unwrap()
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub enum CustomStateError {
    /// No value was found for the specified key (like when get_mut would return None)
    NoValue,
    /// The same value has already been returned (earlier in the same tuple)
    SameValue,
}
impl std::error::Error for CustomStateError {}
impl std::fmt::Display for CustomStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            &CustomStateError::NoValue => "No value",
            &CustomStateError::SameValue => "Duplicate values",
        };
        write!(f, "{}", s)
    }
}

type InternalStateMap = BTreeMap<TypeId, Box<dyn CustomState>>;

/// Adaption of [splitmut::GetMuts] to the use case of obtaining multiple [CustomState] types from the [StateMap].
/// Requires the use of `unsafe`, although in the same (probably ok) way [splitmut] handles it.
pub struct GetMutStates<'a> {
    map: &'a mut InternalStateMap,
    borrowed: std::collections::HashSet<*mut Box<dyn CustomState>>,
}

impl<'a> GetMutStates<'a> {
    pub(self) fn new(map: &'a mut InternalStateMap) -> Self {
        Self {
            map,
            borrowed: std::collections::HashSet::new(),
        }
    }

    pub fn try_get_mut<T: CustomState>(&mut self) -> Result<&'a mut T, CustomStateError> {
        let p = self
            .map
            .get_mut(&TypeId::of::<T>())
            .map(|s| s as *mut Box<dyn CustomState>)
            .ok_or(CustomStateError::NoValue)?;
        if !self.borrowed.insert(p) {
            return Err(CustomStateError::SameValue);
        }
        let p = unsafe { &mut *p };
        Ok(p.as_mut_any().downcast_mut().unwrap())
    }

    pub fn get_mut<T: CustomState>(&mut self) -> &'a mut T {
        self.try_get_mut::<T>().unwrap()
    }

    pub fn get<T: CustomState>(&mut self) -> &'a T {
        self.get_mut()
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

    pub fn get2_mut<T1: CustomState, T2: CustomState>(&mut self) -> (&mut T1, &mut T2) {
        let (state1, state2) = self.map.get2_mut(&TypeId::of::<T1>(), &TypeId::of::<T2>());
        let state1 = state1.unwrap().as_mut_any().downcast_mut().unwrap();
        let state2 = state2.unwrap().as_mut_any().downcast_mut().unwrap();
        (state1, state2)
    }

    pub fn get_multiple_mut(&mut self) -> GetMutStates<'_> {
        GetMutStates::new(&mut self.map)
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
