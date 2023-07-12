//! Common metaheuristic algorithm conditions, e.g. used as termination criteria.

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
    lens::{AnyLens, Lens, LensRef, ValueOf},
    problems::KnownOptimumProblem,
    state::common::{Evaluations, Iterations, Progress},
    CustomState, Problem, State,
};

/// Evaluates to `true` with a probability of `p`.
///
/// # Examples
///
/// Executing two branches with equal probability:
///
/// ```
/// use mahf::{conditions::RandomChance, Configuration};
/// # use mahf::Problem;
///
/// # fn example<P: Problem>() -> Configuration<P> {
/// Configuration::builder()
///     .if_else_(
///         RandomChance::new(0.5),
///         |builder| {
///             /* if branch */
/// #        builder
///         },
///         |builder| {
///             /* else branch */
/// #        builder
///         },
///     )
///     .build()
/// # }
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct RandomChance {
    // Probability of the condition evaluating to `true`.
    pub p: f64,
}

impl RandomChance {
    /// Constructs a new `RandomChance` with probability `p`.
    pub fn from_params(p: f64) -> Self {
        Self { p }
    }

    /// Constructs a new `RandomChance` with probability `p`.
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
    /// Helper trait to represent serializable float-like numbers (e.g. `f64` or `u32`).
    pub trait AnyFloatLike = Copy + Serialize + PartialOrd + Into<f64> + Send + Sync + 'static;
}

/// Evaluates to `true` if `lens` evaluates to a value less than `n`.
///
/// # Common lenses
///
/// Common lenses used with this condition are [`ValueOf<Iterations>`] and [`ValueOf<Evaluations>`],
/// for which the [`LessThanN::iterations`] and [`LessThanN::evaluations`] methods are provided.
///
/// [`ValueOf<Iterations>`]: ValueOf
/// [`ValueOf<Evaluations>`]: ValueOf
/// [`LessThanN::iterations`]: LessThanN<ValueOf<Iterations>>::iterations
/// [`LessThanN::evaluations`]: LessThanN<ValueOf<Evaluations>>::evaluations
///
/// # Examples
///
/// Looping for 1000 iterations:
///
/// ```
/// use mahf::{conditions::LessThanN, Configuration};
/// # use mahf::Problem;
///
/// # fn example<P: Problem>() -> Configuration<P> {
/// Configuration::builder()
///     .while_(LessThanN::iterations(1_000), |builder| {
///         /* main loop */
///         # builder
///     })
///     .build()
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct LessThanN<L>
where
    L: AnyLens,
    L::Target: AnyFloatLike,
{
    /// The value of N.
    pub n: L::Target,
    /// The lens to the value to compare with `n`
    pub lens: L,
}

impl<L> LessThanN<L>
where
    L: AnyLens,
    L::Target: AnyFloatLike,
{
    /// Constructs a new `LessThanN` with the given `n` and `lens`.
    pub fn from_params(n: L::Target, lens: L) -> Self {
        Self { n, lens }
    }

    /// Constructs a new `LessThanN` with the given `n` and `lens`.
    pub fn new<P>(n: L::Target, lens: L) -> Box<dyn Condition<P>>
    where
        P: Problem,
        L: Lens<P>,
    {
        Box::new(Self::from_params(n, lens))
    }
}

impl LessThanN<ValueOf<Iterations>> {
    /// Creates a new `LessThanN` that evaluates to `true` if the number of [`Iterations`] is less than `n`.
    pub fn iterations<P>(n: u32) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self::from_params(n, ValueOf::<Iterations>::new()))
    }
}

impl LessThanN<ValueOf<Evaluations>> {
    /// Creates a new `LessThanN` that evaluates to `true` if the number of [`Evaluations`] is less than `n`.
    pub fn evaluations<P>(n: u32) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self::from_params(n, ValueOf::<Evaluations>::new()))
    }
}

impl<P, L> Condition<P> for LessThanN<L>
where
    P: Problem,
    L: Lens<P>,
    L::Target: AnyFloatLike,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(Progress::<L>::default());
        Ok(())
    }

    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let value = self.lens.get(problem, state)?;
        state.set_value::<Progress<L>>(value.into() / self.n.into());

        Ok(value < self.n)
    }
}

/// Evaluates to `true` if `lens` evaluates to a value `v` such that `v % n == 0`.
///
/// The condition is most commonly used as a trigger for logging.
///
/// # Common lenses
///
/// The most common lens used with this condition is [`ValueOf<Iterations>`],
/// for which the [`LessThanN::iterations`] method is provided.
///
/// [`ValueOf<Iterations>`]: ValueOf
/// [`LessThanN::iterations`]: LessThanN<ValueOf<Iterations>>::iterations
///
/// # Examples
///
/// Logging the best objective value every 10 iterations:
///
/// ```
/// # use mahf::{ExecResult, SingleObjectiveProblem, State};
/// use mahf::{conditions::EveryN, lens::common::BestObjectiveValueLens};
///
/// # fn example<P: SingleObjectiveProblem>(state: &mut State<P>) -> ExecResult<()> {
/// state.configure_log(|config| {
///     config.with(EveryN::iterations(10), BestObjectiveValueLens::entry());
///     Ok(())
/// })?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct EveryN<L: AnyLens> {
    /// The value of N.
    pub n: u32,
    /// The lens to the value to compare with `n`
    pub lens: L,
}

impl<L: AnyLens> EveryN<L> {
    /// Constructs a new `LessThanN` with the given `n` and `lens`.
    pub fn from_params(n: u32, lens: L) -> Self {
        Self { n, lens }
    }

    /// Constructs a new `LessThanN` with the given `n` and `lens`.
    pub fn new<P>(n: u32, lens: L) -> Box<dyn Condition<P>>
    where
        P: Problem,
        L: Lens<P, Target = u32>,
    {
        Box::new(Self::from_params(n, lens))
    }
}

impl EveryN<ValueOf<Iterations>> {
    /// Creates a new `LessThanN` that evaluates to `true` every `n` [`Iterations`].
    pub fn iterations<P>(n: u32) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self::from_params(n, ValueOf::<Iterations>::new()))
    }
}

impl<P, L> Condition<P> for EveryN<L>
where
    P: Problem,
    L: Lens<P, Target = u32>,
{
    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let value = self.lens.get(problem, state)?;
        Ok(value % self.n == 0)
    }
}

/// Holds the previous value for comparison.
#[derive(Deref, DerefMut, Tid)]
struct Previous<T: 'static>(Option<T>);

impl<T> Default for Previous<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Send> CustomState<'_> for Previous<T> {}

/// Checks if two values of type `&T` are equal using some measure.
///
/// Note that the implementation of [`PartialEq`] can be used using the [`PartialEqChecker`].
pub trait EqualityChecker<T>: DynClone + DynSerialize + Send + Sync {
    fn eq(&self, a: &T, b: &T) -> bool;
}

dyn_clone::clone_trait_object!(<T> EqualityChecker<T>);
erased_serde::serialize_trait_object!(<T> EqualityChecker<T>);

/// Checks equality of two values using [`PartialEq`].
#[derive(Default, Clone, Serialize)]
pub struct PartialEqChecker;

impl PartialEqChecker {
    /// Creates a new `PartialEqChecker`.
    pub fn from_params() -> Self {
        Self
    }

    /// Creates a new `PartialEqChecker`.
    pub fn new<T: PartialEq>() -> Box<dyn EqualityChecker<T>> {
        Box::new(Self::from_params())
    }
}

impl<T: PartialEq> EqualityChecker<T> for PartialEqChecker {
    fn eq(&self, a: &T, b: &T) -> bool {
        a.eq(b)
    }
}

/// Checks equality of two values by comparing if their difference is less than some `threshold`.
#[derive(Clone, Serialize)]
pub struct DeltaEqChecker<T: Clone + Serialize + Send + Sync> {
    threshold: T,
}

impl<T: Clone + Serialize + Sub<Output = T> + Ord + Send + Sync + 'static> DeltaEqChecker<T> {
    /// Creates a new `DeltaEqChecker` with some `threshold`.
    pub fn from_params(threshold: T) -> Self {
        Self { threshold }
    }

    /// Creates a new `DeltaEqChecker` with some `threshold`.
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

/// Evaluates to `true` if there is a change in the value `lens` evaluates to.
///
/// The current and previous values of `lens` are compared using an [`EqualityChecker`].
///
/// The condition is most commonly used as a trigger for logging.
///
/// # Checking equality
///
/// The [`PartialEq`] implementation can be used using the [`PartialEqChecker`].
///
/// Threshold-based change detection is possible using the [`DeltaEqChecker`].
///
/// # Examples
///
/// Logging the best objective value if it changes at least by `0.1`:
///
/// ```
/// # use mahf::{ExecResult, SingleObjectiveProblem, State};
/// use mahf::{
///     conditions::{common::DeltaEqChecker, ChangeOf},
///     lens::common::BestObjectiveValueLens,
/// };
///
/// # fn example<P: SingleObjectiveProblem>(state: &mut State<P>) -> ExecResult<()> {
/// state.configure_log(|config| {
///     config.with(
///         ChangeOf::new(
///             DeltaEqChecker::new(0.1.try_into().unwrap()),
///             BestObjectiveValueLens::new(),
///         ),
///         BestObjectiveValueLens::entry(),
///     );
///     Ok(())
/// })?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = ""))]
pub struct ChangeOf<L>
where
    L: AnyLens,
{
    /// The equality checker.
    pub checker: Box<dyn EqualityChecker<L::Target>>,
    /// The lens to the value.
    pub lens: L,
}

impl<L> ChangeOf<L>
where
    L: AnyLens,
    L::Target: Clone + Send,
{
    /// Creates a new `ChangeOf` with the provided equality `checker` and `lens`.
    pub fn from_params(checker: Box<dyn EqualityChecker<L::Target>>, lens: L) -> Self {
        Self { checker, lens }
    }

    /// Creates a new `ChangeOf` with the provided equality `checker` and `lens`.
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

    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let current = self.lens.get_ref(problem, state)?;
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

/// Evaluates to `true` if the objective value of the [`BestIndividual`] is within `delta` of the
/// known optimal objective value.
///
/// The condition therefore requires a valid [`BestIndividual`].
///
/// [`BestIndividual`]: crate::state::common::BestIndividual
///
/// # Examples
///
/// Terminating if the optimum was found requires inverting the condition with `!`:
///
/// ```
/// use mahf::{conditions::OptimumReached, Configuration};
/// # use mahf::{problems::KnownOptimumProblem, ExecResult};
///
/// # fn example<P: KnownOptimumProblem>() -> ExecResult<Configuration<P>> {
/// # Ok(
/// Configuration::builder()
///     .while_(!OptimumReached::new(1e-6)?, |builder| {
///         /* main loop */
///         # builder
///     })
///     .build()
/// # )
/// # }
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct OptimumReached {
    /// The maximal difference between best objective value and optimum.
    pub delta: f64,
}

impl OptimumReached {
    /// Creates a new `OptimumReached` with the given `delta`.
    pub fn from_params(delta: f64) -> ExecResult<Self> {
        ensure!(delta >= 0., "distance must be greater than 0");
        Ok(Self { delta })
    }

    /// Creates a new `OptimumReached` with the given `delta`.
    pub fn new<P>(delta: f64) -> ExecResult<Box<dyn Condition<P>>>
    where
        P: KnownOptimumProblem,
    {
        Ok(Box::new(Self::from_params(delta)?))
    }
}

impl<P> Condition<P> for OptimumReached
where
    P: KnownOptimumProblem,
{
    fn evaluate(&self, problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let value = if let Some(objective) = state.best_objective_value() {
            objective.value() <= problem.known_optimum().value() + self.delta
        } else {
            false
        };
        Ok(value)
    }
}
