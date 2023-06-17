use std::ops::Sub;

use better_any::{Tid, TidAble};
use derivative::Derivative;
use derive_more::{Deref, DerefMut};
use dyn_clone::DynClone;
use erased_serde::Serialize as DynSerialize;
use eyre::ensure;
use rand::Rng;
use serde::{Deserialize, Serialize};
use trait_set::trait_set;

use crate::{
    component::ExecResult,
    conditions::Condition,
    problems::KnownOptimumProblem,
    state::{
        common::Progress,
        lens::{AnyLens, Lens, LensRef},
    },
    CustomState, Problem, State,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct RandomChance {
    // Probability of the condition evaluating to `true`.
    p: f64,
}

impl RandomChance {
    pub fn from_params(p: f64) -> Self {
        Self { p }
    }

    pub fn new<P>(p: f64) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self::from_params(p))
    }
}

impl<P> Condition<P> for RandomChance
where
    P: Problem,
{
    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        Ok(state.random_mut().gen_bool(self.p))
    }
}

trait_set! {
    pub trait AnyFloatLike =  Copy + Serialize + PartialOrd + Into<f64> + Send + Sync + 'static;
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct LessThan<L>
where
    L: AnyLens,
    L::Target: AnyFloatLike,
{
    pub n: L::Target,
    pub lens: L,
}

impl<L> LessThan<L>
where
    L: AnyLens,
    L::Target: AnyFloatLike,
{
    pub fn from_params(n: L::Target, lens: L) -> Self {
        Self { n, lens }
    }

    pub fn new<P>(n: L::Target, lens: L) -> Box<dyn Condition<P>>
    where
        P: Problem,
        L: Lens<P>,
    {
        Box::new(Self::from_params(n, lens))
    }
}

impl<P, L> Condition<P> for LessThan<L>
where
    P: Problem,
    L: Lens<P>,
    L::Target: AnyFloatLike,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(Progress::<L>::default());
        Ok(())
    }

    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let value = self.lens.get(state)?;
        state.set_value::<Progress<L>>(value.into() / self.n.into());

        Ok(value < self.n)
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct EveryN<L: AnyLens> {
    pub n: u32,
    lens: L,
}

impl<L: AnyLens> EveryN<L> {
    pub fn from_params(n: u32, lens: L) -> Self
where {
        Self { n, lens }
    }

    pub fn new<P>(n: u32, lens: L) -> Box<dyn Condition<P>>
    where
        P: Problem,
        L: Lens<P, Target = u32>,
    {
        Box::new(Self::from_params(n, lens))
    }
}

impl<P, L> Condition<P> for EveryN<L>
where
    P: Problem,
    L: Lens<P, Target = u32>,
{
    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let value = self.lens.get(state)?;
        Ok(value % self.n == 0)
    }
}

#[derive(Deref, DerefMut, Tid)]
struct Previous<T: 'static>(Option<T>);

impl<T> Default for Previous<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Send> CustomState<'_> for Previous<T> {}

pub trait EqualityChecker<T>: DynClone + DynSerialize + Send + Sync {
    fn eq(&self, a: &T, b: &T) -> bool;
}

dyn_clone::clone_trait_object!(<T> EqualityChecker<T>);
erased_serde::serialize_trait_object!(<T> EqualityChecker<T>);

#[derive(Default, Clone, Serialize)]
pub struct PartialEqChecker;

impl PartialEqChecker {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<T: PartialEq>() -> Box<dyn EqualityChecker<T>> {
        Box::new(Self::from_params())
    }
}

impl<T: PartialEq> EqualityChecker<T> for PartialEqChecker {
    fn eq(&self, a: &T, b: &T) -> bool {
        a.eq(b)
    }
}

#[derive(Clone, Serialize)]
pub struct DeltaEqChecker<T: Clone + Serialize + Send + Sync> {
    threshold: T,
}

impl<T: Clone + Serialize + Sub<Output = T> + Ord + Send + Sync + 'static> DeltaEqChecker<T> {
    pub fn from_params(threshold: T) -> Self {
        Self { threshold }
    }

    pub fn new(threshold: T) -> Box<dyn EqualityChecker<T>> {
        Box::new(Self::from_params(threshold))
    }
}

impl<T> EqualityChecker<T> for DeltaEqChecker<T>
where
    T: Clone + Serialize + Sub<Output = T> + Ord + Send + Sync,
{
    fn eq(&self, a: &T, b: &T) -> bool {
        let diff = match (a.clone(), b.clone()) {
            (a, b) if a < b => b - a,
            (a, b) => a - b,
        };

        diff >= self.threshold
    }
}

#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct ChangeOf<L>
where
    L: AnyLens,
{
    checker: Box<dyn EqualityChecker<L::Target>>,
    lens: L,
}

impl<L> ChangeOf<L>
where
    L: AnyLens,
    L::Target: Clone + Send,
{
    pub fn from_params(checker: Box<dyn EqualityChecker<L::Target>>, lens: L) -> Self
where {
        Self { checker, lens }
    }

    pub fn new<P>(checker: Box<dyn EqualityChecker<L::Target>>, lens: L) -> Box<dyn Condition<P>>
    where
        P: Problem,
        L: LensRef<P>,
        L::Target: Clone + Send,
    {
        Box::new(Self::from_params(checker, lens))
    }
}

impl<P, L> Condition<P> for ChangeOf<L>
where
    P: Problem,
    L: LensRef<P>,
    L::Target: Clone + Send,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(Previous::<L::Target>::default());
        Ok(())
    }

    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let current = self.lens.get_ref(state)?;
        let mut previous = state.try_borrow_value_mut::<Previous<L::Target>>()?;

        let changed = if let Some(previous) = &*previous {
            self.checker.eq(&*current, previous)
        } else {
            true
        };

        if changed {
            *previous = Some(current.clone());
        }

        Ok(changed)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OptimumReached {
    delta: f64,
}

impl OptimumReached {
    pub fn from_params(delta: f64) -> Self {
        Self { delta }
    }

    pub fn new<P>(delta: f64) -> Box<dyn Condition<P>>
    where
        P: KnownOptimumProblem,
    {
        Box::new(Self::from_params(delta))
    }
}

impl<P> Condition<P> for OptimumReached
where
    P: KnownOptimumProblem,
{
    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let value = if let Some(objective) = state.best_objective_value() {
            let provided = objective.value();
            let known = problem.known_optimum().value();
            debug_assert!(
                provided >= known,
                "the provided objective value is smaller than the known optimum: {} vs. {}",
                provided,
                known
            );
            (provided - known).abs() <= self.delta
        } else {
            false
        };
        Ok(value)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DistanceToOptimumGreaterThan {
    /// Distance to known optimum.
    pub distance: f64,
}

impl DistanceToOptimumGreaterThan {
    pub fn from_params(distance: f64) -> ExecResult<Self> {
        ensure!(distance >= 0., "distance must be greater than 0");
        Ok(Self { distance })
    }

    pub fn new<P>(distance: f64) -> ExecResult<Box<dyn Condition<P>>>
    where
        P: KnownOptimumProblem,
    {
        Ok(Box::new(Self::from_params(distance)?))
    }
}

impl<P> Condition<P> for DistanceToOptimumGreaterThan
where
    P: KnownOptimumProblem,
{
    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let value = if let Some(objective) = state.best_objective_value() {
            objective.value() >= problem.known_optimum().value() + self.distance
        } else {
            false
        };
        Ok(value)
    }
}
