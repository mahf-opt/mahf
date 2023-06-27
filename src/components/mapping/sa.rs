//! State mappings for Simulated Annealing (SA).

use derivative::Derivative;
use eyre::ensure;
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::{
        mapping::{mapping, Mapping},
        Component,
    },
    lens::{AnyLens, ValueLens},
    state::random::Random,
    Problem, State,
};

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct GeometricCooling<L: AnyLens> {
    pub alpha: f64,
    pub lens: L,
}

impl<L: AnyLens> GeometricCooling<L> {
    pub fn from_params(alpha: f64, lens: L) -> ExecResult<Self> {
        ensure!((0.0..1.0).contains(&alpha), "`alpha` must be in [0, 1)");
        Ok(Self { alpha, lens })
    }

    pub fn new<P>(alpha: f64, lens: L) -> ExecResult<Box<dyn Component<P>>>
    where
        P: Problem,
        L: ValueLens<P, f64>,
    {
        Ok(Box::new(Self::from_params(alpha, lens)?))
    }
}

impl<P, L> Mapping<P> for GeometricCooling<L>
where
    P: Problem,
    L: ValueLens<P, f64>,
{
    type Input = f64;
    type Output = f64;

    fn map(&self, value: Self::Input, _rng: &mut Random) -> ExecResult<Self::Output> {
        Ok(value * self.alpha)
    }
}

impl<P, L> Component<P> for GeometricCooling<L>
where
    P: Problem,
    L: ValueLens<P, f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        mapping(self, &self.lens, &self.lens, problem, state)
    }
}
