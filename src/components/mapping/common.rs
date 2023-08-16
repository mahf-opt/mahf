//! Common state mappings.

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
    lens::{BaseLens, Lens, LensAssign},
    state::random::Random,
    Problem, State,
};

/// Linear maps the input between `start` and `end` using `f(x) = (end - start) * x + start`.
///
/// Note that `x`, i.e. `I::Target` is assumed to be within `[0, 1]` to respect the bounds.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Linear<I: BaseLens, O: BaseLens> {
    pub start: f64,
    pub end: f64,
    pub input_lens: I,
    pub output_lens: O,
}

impl<I: BaseLens, O: BaseLens> Linear<I, O> {
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

/// Maps the input between `start` and `end` using the polynomial function `f(x) = (end - start) * x^n + start`.
///
/// Note that `x`, i.e. `I::Target` is assumed to be within `[0, 1]` to respect the bounds.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct Polynomial<I: BaseLens, O: BaseLens> {
    pub start: f64,
    pub end: f64,
    pub n: f64,
    pub input_lens: I,
    pub output_lens: O,
}

impl<I: BaseLens, O: BaseLens> Polynomial<I, O> {
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

/// Overwrites the output using a value uniformly distributed in the `range`.
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct RandomRange<R: AnySampleRange, O: BaseLens> {
    pub range: R,
    pub output_lens: O,
}

impl<R: AnySampleRange, O: BaseLens> RandomRange<R, O> {
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
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let rand = state.random_mut().gen_range(self.range.clone());
        self.output_lens.assign(rand, problem, state)?;
        Ok(())
    }
}
