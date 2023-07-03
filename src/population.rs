//! Helper traits for dealing with collections of individuals, i.e. populations, and obtaining
//! `&`, `&mut` or owned solutions from them.
//!
//! Most traits have blanket implementations for types that implement `IntoIterator<Item=`{`&`, `&mut`, ` `}`Individual>`,
//! which means that manual implementation is not necessary.

use std::ops::{Deref, DerefMut};

use thiserror::Error;

use crate::{problems::SingleObjectiveProblem, Individual, Problem};

/// Trait for obtaining solution references from a collection of [`Individual`]s.
///
/// Internally, [`Individual::solution`] is called on every individual.
///
/// The trait is automatically implemented for types which implement [`IntoIterator<Item=&Individual>`].
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
/// The trait is automatically implemented for types which implement [`IntoIterator<Item=&mut Individual>`].
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
/// The trait is automatically implemented for types which implement [`IntoIterator<Item=Individual>`].
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
/// The trait is automatically implemented for types which implement [`IntoIterator<Item=Problem::Encoding>`].
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

/// An error returned by [`IntoSingle::into_single`] and [`IntoSingleRef::into_single_ref`].
#[derive(Debug, PartialEq, Error)]
pub enum SingleIndividualError {
    /// Population is empty.
    #[error("expected a single individual, but found none")]
    EmptyPopulation,
    /// Population contains too many individuals.
    #[error("`expected a single individual, but found {0}")]
    TooManyIndividuals(usize),
}

/// Trait for converting a collection of [`Individual`]s into its single [`Individual`].
///
/// For converting a collection of `&`[`Individual`]s into its single `&`[`Individual`], see [`IntoSingleRef`].
///
/// # Examples
///
/// `Ok` is only returned for collections with exactly **one** individual:
///
/// ```
/// # use mahf::problems::VectorProblem;
/// use mahf::Individual;
/// use mahf::population::{IntoSingle, SingleIndividualError};
///
/// # pub fn make_individual<P: VectorProblem<Element = usize>>() -> Individual<P> {
/// #     Individual::new_unevaluated(vec![1, 2, 3])
/// # }
/// # pub fn example<P: VectorProblem<Element = usize>>() {
/// let population: Vec<Individual<P>> = vec![];
/// // `into_single` returns `Err` for empty populations.
/// assert_eq!(population.into_single(), Err(SingleIndividualError::EmptyPopulation));
///
/// let population: Vec<Individual<P>> = vec![make_individual()];
/// // `into_single` returns `Ok` for populations with exactly a single individual.
/// assert_eq!(population.into_single(), Ok(make_individual()));
///
/// let population: Vec<Individual<P>> = vec![make_individual(), make_individual()];
/// // `into_single` returns `Err` for populations with more than one individual.
/// assert_eq!(population.into_single(), Err(SingleIndividualError::TooManyIndividuals(2)));
/// # }
/// ```
///
/// # Difference to [`IntoSingleRef`]
///
/// Note that this functionality cannot be merged with [`IntoSingleRef`], as the trait bounds necessary
/// for two blanket implementation for `Individual` and `&Individual` are not expressible yet.
pub trait IntoSingle<P: Problem> {
    /// Tries to convert a collection of [`Individual`]s into its single [`Individual`].
    fn into_single(self) -> Result<Individual<P>, SingleIndividualError>;
}

impl<P, T> IntoSingle<P> for T
where
    P: Problem,
    T: IntoIterator<Item = Individual<P>>,
    T::IntoIter: ExactSizeIterator,
{
    fn into_single(self) -> Result<Individual<P>, SingleIndividualError> {
        let mut iter = self.into_iter();
        let n = iter.len();
        match n {
            0 => Err(SingleIndividualError::EmptyPopulation),
            1 => iter.next().ok_or_else(|| unreachable!()),
            _ => Err(SingleIndividualError::TooManyIndividuals(n)),
        }
    }
}

/// Trait for converting a collection of `&`[`Individual`]s into its single `&`[`Individual`].
///
/// For converting a collection of [`Individual`]s into its single [`Individual`], see [`IntoSingle`].
///
/// # Examples
///
/// `Ok` is only returned for collections with exactly **one** individual:
///
/// ```
/// # use mahf::problems::VectorProblem;
/// use mahf::Individual;
/// use mahf::population::{IntoSingleRef, SingleIndividualError};
///
/// # pub fn make_individual<P: VectorProblem<Element = usize>>() -> Individual<P> {
/// #     Individual::new_unevaluated(vec![1, 2, 3])
/// # }
/// # pub fn example<P: VectorProblem<Element = usize>>() {
/// let population: Vec<Individual<P>> = vec![];
/// // `into_single_ref` returns `Err` for empty populations.
/// assert_eq!(population.into_single_ref(), Err(SingleIndividualError::EmptyPopulation));
///
/// let population: Vec<Individual<P>> = vec![make_individual()];
/// // `into_single_ref` returns `Ok` for populations with exactly a single individual.
/// assert_eq!(population.into_single_ref(), Ok(&make_individual()));
///
/// let population: Vec<Individual<P>> = vec![make_individual(), make_individual()];
/// // `into_single_ref` returns `Err` for populations with more than one individual.
/// assert_eq!(population.into_single_ref(), Err(SingleIndividualError::TooManyIndividuals(2)));
/// # }
/// ```
///
/// # Difference to [`IntoSingle`]
///
/// Note that this functionality cannot be merged with [`IntoSingle`], as the trait bounds necessary
/// for two blanket implementation for `Individual` and `&Individual` are not expressible yet.
pub trait IntoSingleRef<'a, P: Problem> {
    /// Tries to convert a collection of `&`[`Individual`]s into its single `&`[`Individual`].
    fn into_single_ref(self) -> Result<&'a Individual<P>, SingleIndividualError>;
}

impl<'a, P, T> IntoSingleRef<'a, P> for T
where
    P: Problem,
    T: IntoIterator<Item = &'a Individual<P>> + 'a,
    T::IntoIter: ExactSizeIterator,
{
    fn into_single_ref(self) -> Result<&'a Individual<P>, SingleIndividualError> {
        let mut iter = self.into_iter();
        let n = iter.len();
        match n {
            0 => Err(SingleIndividualError::EmptyPopulation),
            1 => iter.next().ok_or_else(|| unreachable!()),
            _ => Err(SingleIndividualError::TooManyIndividuals(n)),
        }
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
