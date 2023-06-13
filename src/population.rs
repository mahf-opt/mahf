use std::ops::{Deref, DerefMut};

use crate::{problems::SingleObjectiveProblem, Individual, Problem};

pub trait AsSolutions<'a, P: Problem> {
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

pub trait AsSolutionsMut<'a, P: Problem> {
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

pub trait IntoSolutions<P: Problem> {
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

pub trait IntoIndividuals<P: Problem> {
    fn into_individuals(self) -> Vec<Individual<P>>
    where
        Self: Sized;
}

impl<P, T> IntoIndividuals<P> for T
where
    P: Problem,
    T: IntoIterator<Item = P::Encoding>,
{
    fn into_individuals(self) -> Vec<Individual<P>>
    where
        Self: Sized,
    {
        self.into_iter().map(Individual::new_unevaluated).collect()
    }
}

pub trait IntoSingle<P: Problem> {
    type Item;

    fn into_single(self) -> Option<Self::Item>;
}

impl<P> IntoSingle<P> for Vec<Individual<P>>
where
    P: Problem,
{
    type Item = Individual<P>;

    fn into_single(self) -> Option<Self::Item> {
        self.into_iter().next()
    }
}

impl<'a, P> IntoSingle<P> for &'a [Individual<P>]
where
    P: Problem,
{
    type Item = &'a Individual<P>;

    fn into_single(self) -> Option<Self::Item> {
        self.iter().next()
    }
}

pub trait BestIndividual<'a, P: SingleObjectiveProblem> {
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
