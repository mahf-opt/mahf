use std::marker::PhantomData;

use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use serde::{Serialize, Serializer};

use crate::{
    problems::{MultiObjectiveProblem, SingleObjectiveProblem},
    CustomState, Individual, Problem,
};

/// The number of evaluations.
#[derive(Clone, Default, Deref, DerefMut, Serialize, Tid)]
pub struct Evaluations(pub u32);

impl CustomState<'_> for Evaluations {}

/// The number of iterations.
#[derive(Clone, Default, Deref, DerefMut, Serialize, Tid)]
pub struct Iterations(pub u32);

impl CustomState<'_> for Iterations {}

/// The progress of some process.
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
        // Deriving `Serialize` has the disadvantage that it is treated as multiple values even when skipping the PhantomData,
        // resulting in a serialized list with a single value instead of just a single value.
        serializer.serialize_f64(self.0)
    }
}

impl<T> CustomState<'_> for Progress<T> {}

/// The best individual yet found.
#[derive(Deref, DerefMut, Tid)]
pub struct BestIndividual<P: SingleObjectiveProblem + 'static>(Option<Individual<P>>);

impl<P: SingleObjectiveProblem> BestIndividual<P> {
    pub fn new() -> Self {
        Self(None::<Individual<P>>)
    }

    /// Update the best individual yet found with the `candidate`.
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
#[derive(Deref, DerefMut, Tid)]
pub struct ParetoFront<P: MultiObjectiveProblem + 'static>(Vec<Individual<P>>);

impl<P: MultiObjectiveProblem> ParetoFront<P> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    // Update the Pareto front with the new `individual`.
    pub fn update(&mut self, _individual: &Individual<P>) -> bool {
        todo!()
    }

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

/// A stack of populations of [`Individual`]s.
#[derive(Tid)]
pub struct Populations<P: Problem + 'static> {
    stack: Vec<Vec<Individual<P>>>,
}

impl<P: Problem> Populations<P> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn current(&self) -> &[Individual<P>] {
        self.stack.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut Vec<Individual<P>> {
        self.stack.last_mut().unwrap()
    }

    pub fn push(&mut self, population: Vec<Individual<P>>) {
        self.stack.push(population);
    }

    pub fn pop(&mut self) -> Vec<Individual<P>> {
        self.stack.pop().unwrap()
    }

    pub fn try_pop(&mut self) -> Option<Vec<Individual<P>>> {
        self.stack.pop()
    }

    pub fn peek(&self, index: usize) -> &[Individual<P>] {
        let n = self.stack.len();
        &self.stack[n - 1 - index]
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

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
