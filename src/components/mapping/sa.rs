use std::marker::PhantomData;

use derivative::Derivative;
use eyre::ensure;
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::{
        mapping::{mapping, Mapping},
        Component,
    },
    state::{extract::ExtractAssign, random::Random},
    Problem, State,
};

#[derive(Serialize, Derivative)]
#[derivative(Clone(bound = ""))]
pub struct GeometricCooling<T> {
    pub alpha: f64,
    marker: PhantomData<fn() -> T>,
}

impl<T> GeometricCooling<T>
where
    T: ExtractAssign<f64>,
{
    pub fn from_params(alpha: f64) -> ExecResult<Self> {
        ensure!((0.0..1.0).contains(&alpha), "`alpha` must be in [0, 1)");
        Ok(Self {
            alpha,
            marker: PhantomData,
        })
    }

    pub fn new<P>(alpha: f64) -> ExecResult<Box<dyn Component<P>>>
    where
        P: Problem,
    {
        Ok(Box::new(Self::from_params(alpha)?))
    }
}

impl<P, T> Mapping<P> for GeometricCooling<T>
where
    P: Problem,
    T: ExtractAssign<f64>,
{
    type Input = f64;
    type Output = f64;

    fn map(&self, value: &Self::Input, _rng: &mut Random) -> ExecResult<Self::Output> {
        Ok(value * self.alpha)
    }
}

impl<P, T> Component<P> for GeometricCooling<T>
where
    P: Problem,
    T: ExtractAssign<f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        mapping::<_, _, T, T>(self, problem, state)
    }
}
