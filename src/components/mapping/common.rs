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
        lens::{AnyLens, Lens, LensAssign},
        random::Random,
    },
    Problem, State,
};

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Linear<I: AnyLens, O: AnyLens> {
    pub start: f64,
    pub end: f64,
    pub input_lens: I,
    pub output_lens: O,
}

impl<I: AnyLens, O: AnyLens> Linear<I, O> {
    pub fn from_params(start: f64, end: f64, input_lens: I, output_lens: O) -> Self {
        Self {
            start,
            end,
            input_lens,
            output_lens,
        }
    }

    pub fn new<P>(start: f64, end: f64, input_lens: I, output_lens: O) -> Box<dyn Component<P>>
    where
        P: Problem,
        I: Lens<P, Target = f64>,
        O: LensAssign<P, Target = f64>,
    {
        Box::new(Self::from_params(start, end, input_lens, output_lens))
    }
}

impl<P, I, O> Mapping<P> for Linear<I, O>
where
    P: Problem,
    I: Lens<P, Target = f64>,
    O: LensAssign<P, Target = f64>,
{
    type Input = f64;
    type Output = f64;

    fn map(&self, value: Self::Input, _rng: &mut Random) -> ExecResult<Self::Output> {
        Ok((self.end - self.start) * value + self.start)
    }
}

impl<P, I, O> Component<P> for Linear<I, O>
where
    P: Problem,
    I: Lens<P, Target = f64>,
    O: LensAssign<P, Target = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        mapping(self, &self.input_lens, &self.output_lens, problem, state)
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Polynomial<I: AnyLens, O: AnyLens> {
    pub start: f64,
    pub end: f64,
    pub n: f64,
    pub input_lens: I,
    pub output_lens: O,
}

impl<I: AnyLens, O: AnyLens> Polynomial<I, O> {
    pub fn from_params(start: f64, end: f64, n: f64, input_lens: I, output_lens: O) -> Self {
        Self {
            start,
            end,
            n,
            input_lens,
            output_lens,
        }
    }

    pub fn new<P>(
        start: f64,
        end: f64,
        n: f64,
        input_lens: I,
        output_lens: O,
    ) -> Box<dyn Component<P>>
    where
        P: Problem,
        I: Lens<P, Target = f64>,
        O: LensAssign<P, Target = f64>,
    {
        Box::new(Self::from_params(start, end, n, input_lens, output_lens))
    }
}

impl<P, I, O> Mapping<P> for Polynomial<I, O>
where
    P: Problem,
    I: Lens<P, Target = f64>,
    O: LensAssign<P, Target = f64>,
{
    type Input = f64;
    type Output = f64;

    fn map(&self, value: Self::Input, _rng: &mut Random) -> ExecResult<Self::Output> {
        Ok((self.end - self.start) * value.powf(self.n) + self.start)
    }
}

impl<P, I, O> Component<P> for Polynomial<I, O>
where
    P: Problem,
    I: Lens<P, Target = f64>,
    O: LensAssign<P, Target = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        mapping(self, &self.input_lens, &self.output_lens, problem, state)
    }
}

trait_set! {
    pub trait AnySampleRange = SampleRange<f64> + Clone + Serialize + Send + Sync + 'static;
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct RandomRange<R: AnySampleRange, O: AnyLens> {
    pub range: R,
    pub output_lens: O,
}

impl<R: AnySampleRange, O: AnyLens> RandomRange<R, O> {
    pub fn from_params(range: R, output_lens: O) -> Self {
        Self { range, output_lens }
    }

    pub fn new<P>(range: R, output_lens: O) -> Box<dyn Component<P>>
    where
        P: Problem,
        R: AnySampleRange,
        O: LensAssign<P, Target = f64>,
    {
        Box::new(Self::from_params(range, output_lens))
    }
}

impl<P, R, O> Component<P> for RandomRange<R, O>
where
    P: Problem,
    R: AnySampleRange,
    O: LensAssign<P, Target = f64>,
{
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let rand = state.random_mut().gen_range(self.range.clone());
        self.output_lens.assign(rand, state)?;
        Ok(())
    }
}
