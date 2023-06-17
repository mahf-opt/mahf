use std::cell::{Ref, RefMut};

use serde::Serialize;
use trait_set::trait_set;

use crate::{component::ExecResult, CustomState, Problem, State};

pub mod common;

pub use common::{IdLens, ValueOf};

pub trait AnyLens: Clone + Serialize + Send + Sync + 'static {
    type Target;
}

pub trait Lens<P: Problem>: AnyLens {
    fn get(&self, state: &State<P>) -> ExecResult<Self::Target>;
}

pub trait LensMap: AnyLens {
    type Source;

    fn map(&self, source: &Self::Source) -> Self::Target;
}

impl<P, E> Lens<P> for E
where
    P: Problem,
    E: LensMap,
    E::Source: for<'a> CustomState<'a>,
{
    fn get(&self, state: &State<P>) -> ExecResult<Self::Target> {
        Ok(self.map(&*state.try_borrow::<E::Source>()?))
    }
}

pub trait LensRef<P: Problem>: AnyLens {
    fn get_ref<'a>(&self, state: &'a State<P>) -> ExecResult<Ref<'a, Self::Target>>;
}

pub trait LensMapRef: AnyLens {
    type Source;

    fn map<'a>(&self, source: &'a Self::Source) -> &'a Self::Target;
}

impl<P, E> LensRef<P> for E
where
    P: Problem,
    E: LensMapRef,
    E::Source: for<'a> CustomState<'a>,
{
    fn get_ref<'a>(&self, state: &'a State<P>) -> ExecResult<Ref<'a, Self::Target>> {
        Ok(Ref::map(state.try_borrow::<E::Source>()?, |source| {
            self.map(source)
        }))
    }
}

pub trait LensMut<P: Problem>: LensRef<P> {
    fn get_mut<'a>(&self, state: &'a State<P>) -> ExecResult<RefMut<'a, Self::Target>>;
}

pub trait LensAssign<P: Problem>: LensMut<P> {
    fn assign(&self, value: Self::Target, state: &State<P>) -> ExecResult<()>;
}

impl<P, E> LensAssign<P> for E
where
    P: Problem,
    E: LensMut<P>,
{
    fn assign(&self, value: Self::Target, state: &State<P>) -> ExecResult<()> {
        let mut t = self.get_mut(state)?;
        *t = value;
        Ok(())
    }
}

trait_set! {
    pub trait ValueLens<P: Problem, T> = Lens<P, Target=T> + LensAssign<P, Target=T>;
}
