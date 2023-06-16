//! Common custom state.
//!
//! This module contains custom state used by almost all metaheuristics and components, for example:
//! - the number of [`Iterations`] and objective function [`Evaluations`] performed,
//! - the [`BestIndividual`] yet found,
//! - the current approximation of the [`ParetoFront`], or
//! - storing [`Populations`] of [`Individual`]s.

use std::{marker::PhantomData, ops::Deref};

use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use serde::{Serialize, Serializer};

use crate::{
    problems::{MultiObjectiveProblem, SingleObjectiveProblem},
    CustomState, Individual, Problem,
};

/// The number of evaluations of the objective function.
///
/// # Usages
///
/// This state is automatically managed by the [`PopulationEvaluator`] component.
///
/// [`Evaluator`]: crate::components::evaluation::PopulationEvaluator
///
/// # Examples
///
/// Using the [`evaluations`] method on [`State`] to retrieve the value:
///
/// [`evaluations`]: crate::State::evaluations
/// [`State`]: crate::State
///
/// ```
/// # use mahf::Problem;
/// use mahf::{State, state::common::Evaluations};
///
/// # pub fn example<P: Problem>() {
/// let mut state: State<P> = State::new();
/// state.insert(Evaluations(0)); // Automatically done by `PopulationEvaluator`.
///
/// let evaluations: u32 = state.evaluations();
/// assert_eq!(evaluations, 0);
/// # }
/// ```
#[derive(Clone, Default, Deref, DerefMut, Serialize, Tid)]
pub struct Evaluations(pub u32);

impl CustomState<'_> for Evaluations {}

/// The number of iterations performed by a loop.
///
/// # Usages
///
/// This state is automatically managed by the [`Loop`] component.
///
/// [`Loop`]: crate::components::Loop
///
/// # Examples
///
/// Using the [`iterations`] method on [`State`] to retrieve the value:
///
/// [`iterations`]: crate::State::iterations
/// [`State`]: crate::State
///
/// ```
/// # use mahf::Problem;
/// use mahf::{State, state::common::Iterations};
///
/// # pub fn example<P: Problem>() {
/// let mut state: State<P> = State::new();
/// state.insert(Iterations(0)); // Automatically done by `Loop`.
///
/// let iterations: u32 = state.iterations();
/// assert_eq!(iterations, 0);
/// # }
/// ```
#[derive(Clone, Default, Deref, DerefMut, Serialize, Tid)]
pub struct Iterations(pub u32);

impl CustomState<'_> for Iterations {}

/// The progress of some process from 0 to 1.
///
/// # Usages
///
/// The [`LessThan<T>`] condition automatically inserts and updates `Progress<T>`.
///
/// [`LessThan<T>`]: crate::conditions::LessThan
#[derive(Clone, Deref, DerefMut, Tid)]
pub struct Progress<T: 'static>(
    #[deref]
    #[deref_mut]
    pub f64,
    PhantomData<fn() -> T>,
);

impl<T> Default for Progress<T> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

impl<T> Serialize for Progress<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Deriving `Serialize` has the disadvantage that it is treated as multiple values even
        // when skipping the PhantomData, resulting in a serialized list with a single value
        // instead of just a single value.
        serializer.serialize_f64(self.0)
    }
}

impl<T> CustomState<'_> for Progress<T> {}

/// The best individual yet found.
///
/// Note that this state is only possible for [`SingleObjectiveProblem`]s.
///
/// # Usages
///
/// Call [`ConfigurationBuilder::update_best_individual`] or insert the [`BestIndividualUpdate`]
/// component manually to insert and update this state.
///
/// [`BestIndividualUpdate`]: crate::components::evaluation::BestIndividualUpdate
///
/// # Examples
///
/// Using the [`best_individual`] method on [`State`] to retrieve the best individual:
///
/// [`best_individual`]: crate::State::best_individual
/// [`State`]: crate::State
///
/// ```
/// # use std::cell::Ref;
/// # use std::fmt::Debug;
/// # use std::ops::Deref;
/// # use mahf::{Individual, SingleObjectiveProblem};
/// use mahf::{State, state::common::BestIndividual};
///
/// # pub fn example<P: SingleObjectiveProblem>() where P::Encoding: Debug {
/// // Requires `P: SingleObjectiveProblem`.
/// let mut state: State<P> = State::new();
/// state.insert(BestIndividual::<P>::new()); // None by default
///
/// let best: Option<Ref<Individual<P>>> = state.best_individual();
/// # }
/// ```
#[derive(Deref, DerefMut, Tid)]
pub struct BestIndividual<P: SingleObjectiveProblem + 'static>(Option<Individual<P>>);

impl<P: SingleObjectiveProblem> BestIndividual<P> {
    /// Construct the state with no individual.
    pub fn new() -> Self {
        Self(None::<Individual<P>>)
    }

    /// Update the best individual yet found with the `candidate`.
    ///
    /// Returns if the best individual was updated.
    pub fn update(&mut self, candidate: &Individual<P>) -> bool {
        if let Some(individual) = &mut self.0 {
            if candidate.objective() < individual.objective() {
                *individual = candidate.clone();
                true
            } else {
                false
            }
        } else {
            self.0 = Some(candidate.clone());
            true
        }
    }
}

impl<P: SingleObjectiveProblem> Default for BestIndividual<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: SingleObjectiveProblem> CustomState<'_> for BestIndividual<P> {}

/// The current approximation of the Pareto front.
///
/// Note that this state is only possible for [`MultiObjectiveProblem`]s.
///
/// # Usages
///
/// Call [`ConfigurationBuilder::update_pareto_front`] or insert the [`ParetoFrontUpdate`]
/// component manually to insert and update this state.
///
/// [`BestIndividualUpdate`]: crate::components::evaluation::ParetoFrontUpdate
///
/// # Examples
///
/// Using the [`pareto_front`] method on [`State`] to retrieve the front:
///
/// [`pareto_front`]: crate::State::pareto_front
/// [`State`]: crate::State
///
/// ```
/// # use std::cell::Ref;
/// # use std::fmt::Debug;
/// # use mahf::{Individual, MultiObjectiveProblem};
/// use mahf::{State, state::common::ParetoFront};
///
/// // Requires `P: MultiObjectiveProblem`.
/// # pub fn example<P: MultiObjectiveProblem>() where P::Encoding: Debug {
/// let mut state: State<P> = State::new();
/// state.insert(ParetoFront::<P>::new()); // Empty by default
///
/// let pareto_front: Ref<ParetoFront<P>> = state.pareto_front();
/// # }
/// ```
#[derive(Deref, DerefMut, Tid)]
pub struct ParetoFront<P: MultiObjectiveProblem + 'static>(Vec<Individual<P>>);

impl<P: MultiObjectiveProblem> ParetoFront<P> {
    /// Constructs the state without any individuals.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    // Update the Pareto front with the new `individual`, returning whether the front was updated.
    pub fn update(&mut self, _individual: &Individual<P>) -> bool {
        todo!("implement non-dominated sorting")
    }

    /// Returns the current approximation of the Pareto front.
    pub fn front(&self) -> &[Individual<P>] {
        &self.0
    }
}

impl<P: MultiObjectiveProblem> CustomState<'_> for ParetoFront<P> {}

impl<P: MultiObjectiveProblem> Default for ParetoFront<P> {
    fn default() -> Self {
        Self::new()
    }
}

/// A stack of populations consisting of one or multiple [`Individual`]s.
///
/// # Usages
///
/// This state is automatically inserted into the [`State`] by the [`Configuration`].
///
/// [`Configuration`]: crate::Configuration
///
/// # Motivation
///
/// [`Component`]s are allowed to freely push to and pop from the stack, which serves
/// to represent a wide range of operations common to metaheuristics.
///
/// For example, the classical evolutionary operators are represented as follows:
/// - Initialization: Push a newly generated population.
/// - Selection: Select a subset from the population on top of the stack and push it as a new population.
/// - Generation: Modify only the top population.
/// - Replacement: Merge the two top-most populations.
///
/// Additionally, applying different components to different parts of a population is possible
/// through splitting the single population into multiple on the stack, and rotating through them.
///
/// # Nesting heuristics
///
/// Note that is **not only** a call stack for nested heuristics, but a generalization of the containers
/// needed for an evolutionary process.
/// Multiple populations may be part of one evolutionary process, and multiple processes may be
/// part of the whole stack.
///
/// For nesting heuristics, additionally see the [`Scope`] component, which defines
/// a proper hierarchy of [`State`]s for other arbitrary custom state.
///
/// [`Scope`]: crate::components::Scope
/// [`State`]: crate::State
///
/// # Examples
///
/// Using the [`populations`] method on [`State`] to retrieve the stack:
///
/// [`populations`]: crate::State::populations
/// [`State`]: crate::State
///
/// ```
/// # use std::cell::Ref;
/// # use mahf::{Individual, Problem, State};
/// use mahf::state::common::Populations;
///
/// // `state: State` is assumed to contain `Populations`.
/// # pub fn example<P: Problem>(state: &mut State<P>) {
/// let populations: Ref<Populations<P>> = state.populations();
/// let top_most_population: &[Individual<P>] = populations.current();
/// // Do something with the population.
/// # }
/// ```
///
/// Use the [`populations_mut`] method on [`State`] to retrieve the stack mutably:
///
/// [`populations_mut`]: crate::State::populations_mut
/// [`State`]: crate::State
///
/// ```
/// # use std::cell::RefMut;
/// # use mahf::{Individual, Problem, State};
/// use mahf::state::common::Populations;
///
/// // `state: State` is assumed to contain `Populations`.
/// # pub fn example<P: Problem>(state: &mut State<P>) {
/// let mut populations: RefMut<Populations<P>> = state.populations_mut();
/// let top_most_population: &mut Vec<Individual<P>> = populations.current_mut();
/// // Do something with the mutable population.
/// # }
/// ```
///
/// Note that usually a separate binding is necessary to allow (mutable) references to
/// populations to exist:
///
/// ```no_run
/// # use std::cell::Ref;
/// # use mahf::{Individual, Problem, State};
/// use mahf::state::common::Populations;
///
/// // `state: State` is assumed to contain `Populations`.
/// # pub fn example<P: Problem>(state: &mut State<P>) {
/// // The following fails to compile because `Ref<Populations>` is immediately dropped.
/// let top_most_population: &[Individual<P>] = state.populations().current();
/// # }
/// ```
#[derive(Tid)]
pub struct Populations<P: Problem + 'static> {
    stack: Vec<Vec<Individual<P>>>,
}

impl<P: Problem> Populations<P> {
    /// Constructs an empty stack of populations.
    ///
    /// It is usually not necessary to call this method manually, as the stack is automatically
    /// constructed by [`Configuration`].
    ///
    /// [`Configuration`]: crate::Configuration
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Returns a reference to the top-most population on the stack.
    ///
    /// # Panics
    ///
    /// Panics when no populations are on the stack.
    /// For a non-failing version, see [`get_current`].
    ///
    /// [`get_current`]: Populations::get_current
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::Ref;
    /// # use mahf::{Individual, Problem, State};
    /// use mahf::state::common::Populations;
    ///
    /// // `state: State` is assumed to contain `Populations`.
    /// # pub fn example<P: Problem>(state: &mut State<P>) {
    /// let populations: Ref<Populations<P>> = state.populations();
    /// let top_most_population: &[Individual<P>] = populations.current();
    /// // Do something with the population.
    /// # }
    /// ```
    pub fn current(&self) -> &[Individual<P>] {
        self.get_current().expect("no population on the stack")
    }

    /// Returns a reference to the top-most population on the stack, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::Ref;
    /// # use mahf::{Individual, Problem, State};
    /// use mahf::state::common::Populations;
    ///
    /// // `state: State` is assumed to contain `Populations`.
    /// # pub fn example<P: Problem>(state: &mut State<P>) {
    /// let populations: Ref<Populations<P>> = state.populations();
    /// if let Some(top_most_population) = populations.get_current() {
    ///    // Do something with the population.
    /// }
    /// # }
    /// ```
    pub fn get_current(&self) -> Option<&[Individual<P>]> {
        self.stack.last().map(|p| p.deref())
    }

    /// Returns a mutable reference to the top-most population on the stack.
    ///
    /// # Panics
    ///
    /// Panics when no populations are on the stack.
    /// For a non-failing version, see [`get_current_mut`].
    ///
    /// [`get_current_mut`]: Populations::get_current_mut
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::RefMut;
    /// # use mahf::{Individual, Problem, State};
    /// use mahf::state::common::Populations;
    ///
    /// // `state: State` is assumed to contain `Populations`.
    /// # pub fn example<P: Problem>(state: &mut State<P>) {
    /// let mut populations: RefMut<Populations<P>> = state.populations_mut();
    /// let top_most_population: &mut Vec<Individual<P>> = populations.current_mut();
    /// // Do something with the population mutably.
    /// # }
    /// ```
    pub fn current_mut(&mut self) -> &mut Vec<Individual<P>> {
        self.stack.last_mut().expect("no population on the stack")
    }

    /// Returns a mutable reference to the top-most population on the stack, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::RefMut;
    /// # use mahf::{Individual, Problem, State};
    /// use mahf::state::common::Populations;
    ///
    /// // `state: State` is assumed to contain `Populations`.
    /// # pub fn example<P: Problem>(state: &mut State<P>) {
    /// let mut populations: RefMut<Populations<P>> = state.populations_mut();
    /// if let Some(top_most_population) = populations.get_current_mut() {
    ///     // Do something with the population mutably.
    /// }
    /// # }
    /// ```
    pub fn get_current_mut(&mut self) -> Option<&mut Vec<Individual<P>>> {
        self.stack.last_mut()
    }

    /// Pushes a population on top of the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::RefMut;
    /// # use mahf::{Individual, Problem, State};
    /// use mahf::state::common::Populations;
    ///
    /// // `state: State` is assumed to contain `Populations`.
    /// # pub fn example<P: Problem>(population: Vec<Individual<P>>, state: &mut State<P>) {
    /// let mut populations: RefMut<Populations<P>> = state.populations_mut();
    /// let height = populations.len();
    /// populations.push(population);
    /// assert_eq!(populations.len(), height + 1);
    /// # }
    /// ```
    pub fn push(&mut self, population: Vec<Individual<P>>) {
        self.stack.push(population);
    }

    /// Pops the top-most population from the stack.
    ///
    /// # Panics
    ///
    /// Panics when no populations are on the stack.
    /// For a non-failing version, see [`try_pop`].
    ///
    /// [`try_pop`]: Populations::try_pop
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::RefMut;
    /// # use mahf::{Individual, Problem, State};
    /// use mahf::state::common::Populations;
    ///
    /// // `state: State` is assumed to contain `Populations`.
    /// # pub fn example<P: Problem>(state: &mut State<P>) {
    /// let mut populations: RefMut<Populations<P>> = state.populations_mut();
    /// let height = populations.len();
    /// assert!(!populations.is_empty());
    /// let top_most_population = populations.pop();
    /// assert_eq!(populations.len(), height - 1);
    /// // Do something with the owned population.
    /// # }
    /// ```
    pub fn pop(&mut self) -> Vec<Individual<P>> {
        self.stack.pop().expect("no population on the stack to pop")
    }

    /// Pops the top-most population from the stack, or returns `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::cell::RefMut;
    /// # use mahf::{Individual, Problem, State};
    /// use mahf::state::common::Populations;
    ///
    /// // `state: State` is assumed to contain `Populations`.
    /// # pub fn example<P: Problem>(state: &mut State<P>) {
    /// let mut populations: RefMut<Populations<P>> = state.populations_mut();
    /// let height = populations.len();
    /// if let Some(population) = populations.try_pop() {
    ///     assert_eq!(populations.len(), height - 1);
    ///     // Do something with the owned population.
    /// }
    /// # }
    /// ```
    pub fn try_pop(&mut self) -> Option<Vec<Individual<P>>> {
        self.stack.pop()
    }

    /// Peeks the population at `depth` from the top of the stack.
    ///
    ///
    /// # Panics
    ///
    /// Panics when there are less than `depth + 1` populations on the stack.
    ///
    /// For a non-failing version, see [`try_peek`].
    ///
    /// [`try_peek`]: Populations::try_peek
    ///
    /// # Examples
    ///
    /// Peeking at `depth` 0 is equivalent to [`current`].
    ///
    /// [`current`]: Populations::current
    ///
    /// ```
    /// # use std::cell::Ref;
    /// # use std::fmt::Debug;
    /// # use mahf::{Individual, Problem, State};
    /// use mahf::state::common::Populations;
    ///
    /// // `state: State` is assumed to contain `Populations`.
    /// # pub fn example<P: Problem>(state: &mut State<P>) where P::Encoding: Debug {
    /// let mut populations: Ref<Populations<P>> = state.populations();
    /// assert!(!populations.is_empty());
    /// assert_eq!(populations.current(), populations.peek(0));
    /// # }
    /// ```
    pub fn peek(&self, depth: usize) -> &[Individual<P>] {
        self.try_peek(depth)
            .expect("not enough populations on the stack")
    }

    /// Peeks the population at `depth` from the top of the stack, or returns `None` if there
    /// are less than `depth + 1` populations on the stack.
    ///
    /// # Examples
    ///
    /// Peeking at `depth` 0 is equivalent to [`get_current`].
    ///
    /// [`get_current`]: Populations::get_current
    ///
    /// ```
    /// # use std::cell::Ref;
    /// # use std::fmt::Debug;
    /// # use mahf::{Individual, Problem, State};
    /// use mahf::state::common::Populations;
    ///
    /// // `state: State` is assumed to contain `Populations`.
    /// # pub fn example<P: Problem>(state: &mut State<P>) where P::Encoding: Debug {
    /// let mut populations: Ref<Populations<P>> = state.populations();
    /// assert!(!populations.is_empty());
    /// assert_eq!(populations.get_current(), populations.try_peek(0));
    /// # }
    /// ```
    pub fn try_peek(&self, depth: usize) -> Option<&[Individual<P>]> {
        let n = self.stack.len();
        // i = `n` - 1 - `index`
        let i = n.checked_sub(1).and_then(|i| i.checked_sub(depth));
        i.and_then(|i| self.stack.get(i)).map(|p| p.deref())
    }

    /// Shifts the first `n` populations circularly.
    ///
    /// This is useful for applying different operations to different populations on the stack
    /// without explicitly popping and pushing them.
    ///
    /// # Panics
    ///
    /// Panics if `n` is greater than `len() - 1`.
    ///
    /// # Examples
    ///
    /// Applying three different operations to the three populations.
    ///
    /// ```
    /// # use std::cell::RefMut;
    /// # use mahf::{Individual, Problem, State};
    /// use mahf::state::common::Populations;
    ///
    /// pub fn op1<P: Problem>(pop: &[Individual<P>]) { /* ... */ }
    /// pub fn op2<P: Problem>(pop: &[Individual<P>]) { /* ... */ }
    /// pub fn op3<P: Problem>(pop: &[Individual<P>]) { /* ... */ }
    ///
    /// // `state: State` is assumed to contain `Populations`.
    /// # pub fn example<P: Problem>(state: &mut State<P>) {
    /// let mut populations: RefMut<Populations<P>> = state.populations_mut();
    /// // `populations` is assumed to contain [..., `pop3`, `pop2`, `pop1`] <-- top.
    /// assert!(populations.len() >= 3);
    ///
    /// // Applying op1 to pop1.
    /// op1(populations.current());
    /// populations.shift(3);
    /// // [..., `pop1`, `pop3`, `pop2`]
    ///
    /// // Applying op2 to pop2.
    /// op2(populations.current());
    /// populations.shift(3);
    /// // [..., `pop2`, `pop1`, `pop3`]
    ///
    /// // Applying op3 to pop3.
    /// op3(populations.current());
    /// populations.shift(3);
    /// // [..., `pop3`, `pop2`, `pop1`]
    ///
    /// // Populations are ordered like before.
    /// # }
    /// ```
    pub fn shift(&mut self, n: usize) {
        let len = self.stack.len();
        self.stack[len - 1 - n..len].rotate_right(1);
    }

    /// Returns `true` if the stack contains no populations.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mahf::{Problem};
    /// use mahf::state::common::Populations;
    ///
    /// # pub fn example<P: Problem>() {
    /// let mut populations: Populations<P> = Populations::new();
    /// assert!(populations.is_empty());
    ///
    /// populations.push(Vec::new());
    /// assert!(!populations.is_empty());
    /// # }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Returns the number of populations on the stack, also referred to as its 'height' or `depth`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mahf::{Problem};
    /// use mahf::state::common::Populations;
    ///
    /// # pub fn example<P: Problem>() {
    /// let mut populations: Populations<P> = Populations::new();
    ///
    /// populations.push(Vec::new());
    /// populations.push(Vec::new());
    /// populations.push(Vec::new());
    ///
    /// assert_eq!(populations.len(), 3);
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl<P: Problem> CustomState<'_> for Populations<P> {}

impl<P: Problem> Default for Populations<P> {
    fn default() -> Self {
        Self::new()
    }
}
