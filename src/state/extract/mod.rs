use std::cell::{Ref, RefMut};

use trait_set::trait_set;

use crate::{component::ExecResult, state::StateRegistry, CustomState};

pub mod common;

pub use common::{IdFn, ValueOf};

pub trait Extract: 'static {
    type Target;

    fn extract(state: &StateRegistry) -> ExecResult<Self::Target>;
}

pub trait ExtractMap {
    type Source;
    type Target;

    fn map(source: &Self::Source) -> Self::Target;
}

impl<E, Source, T> Extract for E
where
    E: ExtractMap<Source = Source, Target = T> + 'static,
    Source: for<'a> CustomState<'a>,
{
    type Target = T;

    fn extract(state: &StateRegistry) -> ExecResult<Self::Target> {
        Ok(E::map(&*state.try_borrow::<Source>()?))
    }
}

pub trait ExtractRef: 'static {
    type Target;

    fn extract_ref<'a>(state: &'a StateRegistry) -> ExecResult<Ref<'a, Self::Target>>;
}

pub trait ExtractMapRef {
    type Source;
    type Target;

    fn map(source: &Self::Source) -> &Self::Target;
}

impl<E, Source, T> ExtractRef for E
where
    E: ExtractMapRef<Source = Source, Target = T> + 'static,
    Source: for<'a> CustomState<'a>,
{
    type Target = T;

    fn extract_ref<'a>(state: &'a StateRegistry) -> ExecResult<Ref<'a, Self::Target>> {
        Ok(Ref::map(state.try_borrow::<Source>()?, E::map))
    }
}

pub trait ExtractMutRef: ExtractRef {
    fn extract_mut<'a>(state: &'a StateRegistry) -> ExecResult<RefMut<'a, Self::Target>>;
}

pub trait Assign: ExtractMutRef {
    fn assign(value: Self::Target, state: &StateRegistry) -> ExecResult<()>;
}

impl<E, T> Assign for E
where
    E: ExtractMutRef<Target = T> + 'static,
{
    fn assign(value: Self::Target, state: &StateRegistry) -> ExecResult<()> {
        let mut t = Self::extract_mut(state)?;
        *t = value;
        Ok(())
    }
}

trait_set! {
    pub trait ExtractAssign<T> = Extract<Target=T> + Assign<Target=T>;
}
