use rand::distributions::uniform::SampleRange;
use rand::Rng;
use serde::Serialize;
use std::marker::PhantomData;
use trait_set::trait_set;

use crate::{
    components::Component, framework::AnyComponent, problems::Problem, state::State, CustomState,
};

pub trait Mapping<P: Problem> {
    type Input;
    type Output;
    #[allow(unused_variables)]
    fn initialize(&self, problem: &P, state: &mut State<P>) {}
    fn map(&self, value: Self::Input, problem: &P, state: &mut State<P>) -> Self::Output;
}

#[derive(Serialize, derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Mapper<T, X, S>(pub T, PhantomData<fn() -> (X, S)>)
where
    T: Clone;

impl<T, X, S> Mapper<T, X, S>
where
    T: Clone,
{
    pub fn new(schedule: T) -> Self {
        Self(schedule, PhantomData)
    }
}

impl<T, X, S, P> Component<P> for Mapper<T, X, S>
where
    P: Problem,
    T: AnyComponent + Mapping<P> + Serialize + Clone,
    <T as Mapping<P>>::Input: Copy,
    <T as Mapping<P>>::Output: Copy,
    X: for<'a> CustomState<'a> + std::ops::Deref<Target = <T as Mapping<P>>::Input>,
    S: for<'a> CustomState<'a> + std::ops::DerefMut<Target = <T as Mapping<P>>::Output>,
{
    fn initialize(&self, problem: &P, state: &mut State<P>) {
        self.0.initialize(problem, state);
    }

    fn require(&self, _problem: &P, state: &State<P>) {
        state.require::<Self, S>();
    }

    fn execute(&self, problem: &P, state: &mut State<P>) {
        let value = state.get_value::<X>();
        let result = self.0.map(value, problem, state);
        state.set_value::<S>(result);
    }
}

#[derive(Clone, Serialize)]
pub struct Linear {
    pub start: f64,
    pub end: f64,
}

impl Linear {
    pub fn new<P, X, S>(start: f64, end: f64) -> Box<dyn Component<P>>
    where
        P: Problem,
        X: for<'a> CustomState<'a> + std::ops::Deref<Target = <Self as Mapping<P>>::Input>,
        S: for<'a> CustomState<'a> + std::ops::DerefMut<Target = <Self as Mapping<P>>::Output>,
    {
        Box::new(Mapper::<_, X, S>::new(Self { start, end }))
    }
}

impl<P: Problem> Mapping<P> for Linear {
    type Input = f64;
    type Output = f64;

    #[contracts::requires((0.0..=1.0).contains(&value))]
    fn map(&self, value: f64, _problem: &P, _state: &mut State<P>) -> f64 {
        (self.end - self.start) * value + self.start
    }
}

trait_set! {
    pub trait RandomRange = SampleRange<f64> + Clone + Serialize + Send + Sync + 'static;
}

#[derive(Clone, Serialize)]
pub struct Random<R: RandomRange>(pub R);

impl<R> Random<R>
where
    R: RandomRange,
{
    pub fn new<P, X, S>(range: R) -> Box<dyn Component<P>>
    where
        P: Problem,
        X: for<'a> CustomState<'a> + std::ops::Deref<Target = <Self as Mapping<P>>::Input>,
        S: for<'a> CustomState<'a> + std::ops::DerefMut<Target = <Self as Mapping<P>>::Output>,
    {
        Box::new(Mapper::<_, X, S>::new(Self(range)))
    }
}

impl<P, R> Mapping<P> for Random<R>
where
    R: RandomRange,
    P: Problem,
{
    type Input = f64;
    type Output = f64;

    fn map(&self, _value: f64, _problem: &P, state: &mut State<P>) -> f64 {
        state.random_mut().gen_range(self.0.clone())
    }
}

#[derive(Clone, Serialize)]
pub struct GeometricCooling {
    pub alpha: f64,
}

impl GeometricCooling {
    #[contracts::requires(((0.0..=1.0).contains(&alpha)))]
    pub fn new<P, S>(alpha: f64) -> Box<dyn Component<P>>
    where
        P: Problem,
        S: for<'a> CustomState<'a> + std::ops::DerefMut<Target = f64>,
    {
        Box::new(Mapper::<_, S, S>::new(Self { alpha }))
    }
}

impl<P: Problem> Mapping<P> for GeometricCooling {
    type Input = f64;
    type Output = f64;

    fn map(&self, value: Self::Input, _problem: &P, _state: &mut State<P>) -> Self::Output {
        value * self.alpha
    }
}
