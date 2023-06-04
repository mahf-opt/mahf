use std::marker::PhantomData;

use derivative::Derivative;
use rand::{distributions::uniform::SampleRange, Rng};
use serde::Serialize;
use trait_set::trait_set;

use crate::{
    component::ExecResult,
    components::{
        mapping::{mapping, Mapping},
        Component,
    },
    state::{
        extract::{Assign, Extract},
        random::Random,
    },
    Problem, State,
};

#[derive(Serialize, Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Linear<I, O> {
    pub start: f64,
    pub end: f64,
    marker: PhantomData<fn() -> (I, O)>,
}

impl<I, O> Linear<I, O>
where
    I: Extract<Target = f64>,
    O: Assign<Target = f64>,
{
    pub fn from_params(start: f64, end: f64) -> Self {
        Self {
            start,
            end,
            marker: PhantomData,
        }
    }

    pub fn new<P>(start: f64, end: f64) -> Box<dyn Component<P>>
    where
        P: Problem,
    {
        Box::new(Self::from_params(start, end))
    }
}

impl<P, I, O> Mapping<P> for Linear<I, O>
where
    P: Problem,
{
    type Input = f64;
    type Output = f64;

    fn map(&self, value: &Self::Input, _rng: &mut Random) -> ExecResult<Self::Output> {
        Ok((self.end - self.start) * value + self.start)
    }
}

impl<P, I, O> Component<P> for Linear<I, O>
where
    P: Problem,
    I: Extract<Target = f64>,
    O: Assign<Target = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        mapping::<P, Self, I, O>(self, problem, state)
    }
}

#[derive(Serialize, Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Polynomial<I, O> {
    pub start: f64,
    pub end: f64,
    pub n: f64,
    marker: PhantomData<fn() -> (I, O)>,
}

impl<I, O> Polynomial<I, O>
where
    I: Extract<Target = f64>,
    O: Assign<Target = f64>,
{
    pub fn from_params(start: f64, end: f64, n: f64) -> Self {
        Self {
            start,
            end,
            n,
            marker: PhantomData,
        }
    }

    pub fn new<P>(start: f64, end: f64, n: f64) -> Box<dyn Component<P>>
    where
        P: Problem,
    {
        Box::new(Self::from_params(start, end, n))
    }
}

impl<P, I, O> Mapping<P> for Polynomial<I, O>
where
    P: Problem,
{
    type Input = f64;
    type Output = f64;

    fn map(&self, value: &Self::Input, _rng: &mut Random) -> ExecResult<Self::Output> {
        Ok((self.end - self.start) * value.powf(self.n) + self.start)
    }
}

impl<P, I, O> Component<P> for Polynomial<I, O>
where
    P: Problem,
    I: Extract<Target = f64>,
    O: Assign<Target = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        mapping::<P, Self, I, O>(self, problem, state)
    }
}

trait_set! {
    pub trait AnySampleRange = SampleRange<f64> + Clone + Serialize + Send + Sync + 'static;
}

#[derive(Serialize, Derivative)]
#[derivative(Clone(bound = ""))]
pub struct RandomRange<R: AnySampleRange, I, O> {
    pub range: R,
    marker: PhantomData<fn() -> (I, O)>,
}

impl<R, I, O> RandomRange<R, I, O>
where
    R: AnySampleRange,
    I: Extract<Target = f64>,
    O: Assign<Target = f64>,
{
    pub fn from_params(range: R) -> Self {
        Self {
            range,
            marker: PhantomData,
        }
    }

    pub fn new<P>(range: R) -> Box<dyn Component<P>>
    where
        P: Problem,
    {
        Box::new(Self::from_params(range))
    }
}

impl<P, R, I, O> Mapping<P> for RandomRange<R, I, O>
where
    P: Problem,
    R: AnySampleRange,
    I: Extract<Target = f64>,
    O: Assign<Target = f64>,
{
    type Input = f64;
    type Output = f64;

    fn map(&self, _value: &Self::Input, rng: &mut Random) -> ExecResult<Self::Output> {
        Ok(rng.gen_range(self.range.clone()))
    }
}

impl<P, R, I, O> Component<P> for RandomRange<R, I, O>
where
    P: Problem,
    R: AnySampleRange,
    I: Extract<Target = f64>,
    O: Assign<Target = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        mapping::<_, _, I, O>(self, problem, state)
    }
}
