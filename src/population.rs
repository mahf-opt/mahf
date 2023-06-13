use std::ops::{Deref, DerefMut};

use crate::{problems::SingleObjectiveProblem, Individual, Problem};

/// Trait for obtaining solution references from a collection of [`Individual`]s.
///
/// Internally, [`Individual::solution`] is called on every individual.
///
/// The trait is automatically implemented for types which implement [`IntoIterator<Item=&Individual`].
///
/// # Examples
///
/// ```
/// use mahf::{Individual, Problem};
/// use mahf::population::AsSolutions;
///
/// pub fn example<P: Problem>(population: &[Individual<P>]) {
///     for solution in population.as_solutions() {
///         // Do something with each solution (`&P::Encoding`).
///     }
/// }
/// ```
pub trait AsSolutions<'a, P: Problem> {
    /// Obtains solution references from a collection of [`Individual`]s.
    fn as_solutions(&'a self) -> Vec<&'a P::Encoding>;
}

impl<'a, P, T> AsSolutions<'a, P> for T
where
    P: Problem,
    T: Deref + ?Sized + 'a,
    &'a T::Target: IntoIterator<Item = &'a Individual<P>>,
{
    fn as_solutions(&'a self) -> Vec<&'a P::Encoding> {
        self.deref().into_iter().map(Individual::solution).collect()
    }
}

/// Trait for obtaining mutable solution references from a collection of [`Individual`]s.
///
/// Internally, [`Individual::solution_mut`] is called on every individual, which means that
/// all objective values are reset.
///
/// The trait is automatically implemented for types which implement [`IntoIterator<Item=&mut Individual`].
///
/// # Examples
///
/// ```
/// use mahf::{Individual, Problem};
/// use mahf::population::AsSolutionsMut;
///
/// pub fn example<P: Problem>(mut population: &mut [Individual<P>]) {
///     for solution in population.as_solutions_mut() {
///         // Do something with each mutable solution (`&mut P::Encoding`).
///     }
/// }
/// ```
pub trait AsSolutionsMut<'a, P: Problem> {
    /// Obtains mutable solution references from a collection of [`Individual`]s.
    fn as_solutions_mut(&'a mut self) -> Vec<&'a mut P::Encoding>;
}

impl<'a, P, T> AsSolutionsMut<'a, P> for T
where
    P: Problem,
    T: DerefMut + ?Sized + 'a,
    &'a mut T::Target: IntoIterator<Item = &'a mut Individual<P>>,
{
    fn as_solutions_mut(&'a mut self) -> Vec<&'a mut P::Encoding> {
        self.deref_mut()
            .into_iter()
            .map(Individual::solution_mut)
            .collect()
    }
}

/// Trait for converting a collection of [`Individual`]s into their solutions.
///
/// Internally, [`Individual::into_solution`] is called on every individual.
///
/// The trait is automatically implemented for types which implement [`IntoIterator<Item=Individual`].
///
/// # Reverse conversion
///
/// For converting from solutions into [`Individual`]s, see [`IntoIndividuals`].
///
/// # Examples
///
/// ```
/// use mahf::{Individual, Problem};
/// use mahf::population::IntoSolutions;
///
/// pub fn example<P: Problem>(population: Vec<Individual<P>>) {
///     for solution in population.into_solutions() {
///         // Do something with each owned solution (`P::Encoding`).
///     }
/// }
/// ```
pub trait IntoSolutions<P: Problem> {
    /// Converts a collection of [`Individual`]s into their solutions.
    fn into_solutions(self) -> Vec<P::Encoding>
    where
        Self: Sized;
}

impl<P, T> IntoSolutions<P> for T
where
    P: Problem,
    T: IntoIterator<Item = Individual<P>>,
{
    fn into_solutions(self) -> Vec<P::Encoding>
    where
        Self: Sized,
    {
        self.into_iter().map(Individual::into_solution).collect()
    }
}

/// Trait for converting solutions into a collection of [`Individual`]s .
///
/// Internally, [`Individual::new_unevaluated`] is called on every solution.
///
/// The trait is automatically implemented for types which implement [`IntoIterator<Item=Problem::Encoding`].
///
/// # Reverse conversion
///
/// For converting from [`Individual`]s into their solutions, see [`IntoSolutions`].
///
/// # Examples
///
/// ```
/// use mahf::{Individual, Problem};
/// use mahf::population::{IntoIndividuals};
///
/// pub fn example<P: Problem>(solutions: Vec<P::Encoding>) {
///     for individual in solutions.into_individuals::<P>() {
///         // Do something with each individual.
///     }
/// }
/// ```
pub trait IntoIndividuals<T> {
    /// Converts solutions into a collection of [`Individual`]s.
    fn into_individuals<P: Problem<Encoding = T>>(self) -> Vec<Individual<P>>
    where
        Self: Sized;
}

impl<T> IntoIndividuals<T::Item> for T
where
    T: IntoIterator,
{
    fn into_individuals<P: Problem<Encoding = T::Item>>(self) -> Vec<Individual<P>>
    where
        Self: Sized,
    {
        self.into_iter().map(Individual::new_unevaluated).collect()
    }
}

/// Trait for converting a collection of [`Individual`]s into its single [`Individual`].
///
/// For unwrapping a collection of `&`[`Individual`]s into its single `&`[`Individual`], see [`IntoSingleRef`].
///
/// # Examples
///
/// ```
/// use mahf::{Individual, Problem};
/// use mahf::population::IntoSingle;
///
/// pub fn example<P: Problem>(population: Vec<Individual<P>>) {
///     let single: Option<Individual<P>> = population.into_single();
/// }
/// ```
pub trait IntoSingle<P: Problem> {
    /// Converts a collection of [`Individual`]s into its single [`Individual`].
    fn into_single(self) -> Option<Individual<P>>;
}

impl<P, T> IntoSingle<P> for T
where
    P: Problem,
    T: IntoIterator<Item = Individual<P>>,
{
    fn into_single(self) -> Option<Individual<P>> {
        self.into_iter().next()
    }
}

/// Trait for converting a collection of `&`[`Individual`]s into its single `&`[`Individual`].
///
/// For unwrapping a collection of [`Individual`]s into its single [`Individual`], see [`IntoSingle`].
///
/// # Examples
///
/// ```
/// use mahf::{Individual, Problem};
/// use mahf::population::IntoSingleRef;
///
/// pub fn example<P: Problem>(population: &[Individual<P>]) {
///     let single: Option<&Individual<P>> = population.into_single_ref();
/// }
/// ```
pub trait IntoSingleRef<'a, P: Problem> {
    /// Converts a collection of `&`[`Individual`]s into its single `&`[`Individual`].
    fn into_single_ref(self) -> Option<&'a Individual<P>>;
}

impl<'a, P, T> IntoSingleRef<'a, P> for T
where
    P: Problem,
    T: IntoIterator<Item = &'a Individual<P>> + 'a,
{
    fn into_single_ref(self) -> Option<&'a Individual<P>> {
        self.into_iter().next()
    }
}

/// Trait for obtaining a reference to the best [`Individual`] from a collection of [`Individual`]s,
/// i.e. the individual with lowest objective value.
///
/// # Examples
///
/// ```
/// use mahf::{Individual, SingleObjectiveProblem};
/// use mahf::population::BestIndividual;
///
/// pub fn example<P: SingleObjectiveProblem>(population: &[Individual<P>]) {
///     let best: Option<&Individual<P>> = population.best_individual();
///
///     for individual in population {
///         // `best` has the lowest objective value.
///         assert!(best.unwrap().objective() <= individual.objective());
///     }
/// }
/// ```
pub trait BestIndividual<'a, P: SingleObjectiveProblem> {
    /// Obtains a reference to the best [`Individual`] from a collection of [`Individual`]s.
    fn best_individual(&'a self) -> Option<&'a Individual<P>>;
}

impl<'a, P, T> BestIndividual<'a, P> for T
where
    P: SingleObjectiveProblem,
    T: Deref + ?Sized + 'a,
    &'a T::Target: IntoIterator<Item = &'a Individual<P>>,
{
    fn best_individual(&'a self) -> Option<&'a Individual<P>> {
        self.deref().into_iter().min_by_key(|i| i.objective())
    }
}
