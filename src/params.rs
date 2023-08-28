use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter},
};

use downcast_rs::Downcast;
use eyre::ContextCompat;
use itertools::Itertools;
#[cfg(feature = "macros")]
pub use mahf_derive::{Parametrized, TryFromParams};
#[cfg(not(feature = "macros"))]
pub(crate) use mahf_derive::{Parametrized, TryFromParams};
use trait_set::trait_set;

use crate::{Component, ExecResult, Problem};

trait_set! {
    /// A type-erased parameter.
    pub trait Parameter = Debug + dyn_clone::DynClone + Downcast + Send;
}

downcast_rs::impl_downcast!(Parameter);
dyn_clone::clone_trait_object!(Parameter);

#[derive(Clone)]
pub struct Param {
    inner: Box<dyn Parameter>,
}

impl Debug for Param {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Param {
    pub fn new<T: Parameter>(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }

    pub fn is<T: Parameter>(&self) -> bool {
        self.inner.as_ref().as_any().is::<T>()
    }

    pub fn as_ref<T: Parameter>(&self) -> Option<&T> {
        self.inner.as_any().downcast_ref()
    }

    pub fn as_mut<T: Parameter>(&mut self) -> Option<&mut T> {
        self.inner.as_any_mut().downcast_mut()
    }

    pub fn into_inner<T: Parameter>(self) -> Option<T> {
        self.inner.into_any().downcast().ok().map(|value| *value)
    }
}

/// A set of named parameters.
#[derive(Default, Clone)]
pub struct Params {
    params: HashMap<String, Param>,
}

impl Params {
    pub fn new() -> Self {
        Self {
            params: Default::default(),
        }
    }

    pub fn insert_raw(&mut self, name: impl Into<String>, value: Param) {
        let name = name.into();
        assert!(
            !name.contains('.'),
            "dots (.) are not allowed in parameter names"
        );
        self.params.insert(name, value);
    }

    pub fn insert<T: Parameter>(&mut self, name: impl Into<String>, value: T) {
        self.insert_raw(name, Param::new(value));
    }

    pub fn with<T: Parameter>(mut self, name: impl Into<String>, value: T) -> Self {
        self.insert(name, value);
        self
    }

    pub fn with_real(self, name: impl Into<String>, value: impl Into<f64>) -> Self {
        self.with(name, value.into())
    }

    pub fn with_int(self, name: impl Into<String>, value: impl Into<u32>) -> Self {
        self.with(name, value.into())
    }

    pub fn contains<T: Parameter>(&self, name: &str) -> bool {
        self.params
            .get(name)
            .and_then(|param| param.is::<T>().then_some(()))
            .is_some()
    }

    pub fn get<T: Parameter>(&self, name: &str) -> Option<&T> {
        self.params.get(name).and_then(|param| param.as_ref())
    }

    pub fn get_mut<T: Parameter>(&mut self, name: &str) -> Option<&mut T> {
        self.params.get_mut(name).and_then(|param| param.as_mut())
    }

    pub fn extract<T: Parameter>(&mut self, name: &str) -> Option<T> {
        self.params
            .remove(name)
            .and_then(|param| param.into_inner())
    }

    pub fn try_extract<T: Parameter>(&mut self, name: &str) -> ExecResult<T> {
        self.extract(name).wrap_err(format!(
            "parameter {}:{} not found",
            name,
            std::any::type_name::<T>()
        ))
    }

    pub fn flatten(&mut self) -> bool {
        let keys: Vec<String> = self.params.keys().cloned().collect();
        let mut modified = false;
        for key in &keys {
            if self.params[key].is::<Self>() {
                modified = true;
                let mut inner = self
                    .params
                    .remove(key)
                    .unwrap()
                    .into_inner::<Self>()
                    .unwrap();
                inner.flatten();
                for (inner_key, inner_param) in inner.params {
                    let flat_key = format!("{key}.{inner_key}");
                    assert!(
                        !self.params.contains_key(&flat_key),
                        "flat key is already present"
                    );
                    self.params.insert(flat_key, inner_param);
                }
            }
        }
        modified
    }

    pub fn as_flat(&self) -> Self {
        let mut params = self.clone();
        params.flatten();
        params
    }

    pub fn nest(&mut self) -> bool {
        let keys = self
            .params
            .keys()
            .filter_map(|key| {
                let (outer, inner) = key.split_once('.')?;
                Some((outer.to_string(), (key.to_string(), inner.to_string())))
            })
            .into_group_map();

        let modified = !keys.is_empty();

        for (outer, group) in keys {
            let params: HashMap<_, _> = group
                .into_iter()
                .map(|(key, inner)| {
                    let value = self.params.remove(&key).unwrap();
                    (inner, value)
                })
                .collect();

            self.insert(outer, Self::from(params));
        }

        modified
    }

    pub fn as_nest(&self) -> Self {
        let mut params = self.clone();
        params.nest();
        params
    }
}

impl Debug for Params {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.params.fmt(f)
    }
}

impl From<HashMap<String, Param>> for Params {
    fn from(value: HashMap<String, Param>) -> Self {
        Self { params: value }
    }
}

pub trait TryFromParams: TryFrom<Params> {
    fn try_from_params(params: Params) -> ExecResult<Self>;
}

impl<T> TryFromParams for T
where
    T: TryFrom<Params>,
    eyre::Report: From<<T as TryFrom<Params>>::Error>,
{
    fn try_from_params(params: Params) -> ExecResult<Self> {
        Ok(Self::try_from(params)?)
    }
}

pub trait TryComponentFromParams<P: Problem> {
    fn try_from_params(params: Params) -> ExecResult<Box<dyn Component<P>>>
    where
        Self: Sized;
}

impl<P, T> TryComponentFromParams<P> for T
where
    P: Problem,
    T: TryFromParams + Component<P> + 'static,
{
    fn try_from_params(params: Params) -> ExecResult<Box<dyn Component<P>>>
    where
        Self: Sized,
    {
        Self::try_from_params(params).map(|t| Box::new(t) as Box<dyn Component<P>>)
    }
}

pub trait Parametrized {
    fn param_names(&self) -> HashSet<Cow<str>>;

    fn get_params(&self) -> Params;

    fn set_params(&mut self, params: Params);
}
