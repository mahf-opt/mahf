//! Generation methods

use crate::{
    framework::{
        components::{AnyComponent, Component},
        Individual,
    },
    problems::{LimitedVectorProblem, Problem, VectorProblem},
    state::State,
};
use itertools::Itertools;
use rand::distributions::uniform::SampleUniform;
use serde::{Deserialize, Serialize};

pub mod mutation;
pub mod recombination;
pub mod swarm;

/// Specialized component trait to generate a new population from the current one.
///
/// This trait is especially useful for components that modify solutions independently.
/// For combining multiple solutions, see [Recombination].
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Generator].
pub trait Generation<P: Problem> {
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        problem: &P,
        state: &mut State<P>,
    );
}

#[derive(serde::Serialize, Clone)]
pub struct Generator<T: Clone>(pub T);

impl<T, P> Component<P> for Generator<T>
where
    P: Problem,
    T: AnyComponent + Generation<P> + Serialize + Clone,
{
    fn execute(&self, problem: &P, state: &mut State<P>) {
        let population = state.populations_mut().pop();
        let mut population = population
            .into_iter()
            .map(Individual::into_solution)
            .collect();
        self.0.generate_population(&mut population, problem, state);
        let population = population
            .into_iter()
            .map(Individual::<P>::new_unevaluated)
            .collect();
        state.populations_mut().push(population);
    }
}

/// Specialized component trait to generate a new population from the current one.
///
/// This trait is especially useful for components that combine multiple solutions.
/// For modifying solutions independently, see [Generation].
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Recombinator].
pub trait Recombination<P: Problem> {
    fn recombine_solutions(
        &self,
        parents: Vec<P::Encoding>,
        offspring: &mut Vec<P::Encoding>,
        problem: &P,
        state: &mut State<P>,
    );
}

#[derive(serde::Serialize, Clone)]
pub struct Recombinator<T: Clone>(pub T);

impl<T, P, D> Component<P> for Recombinator<T>
where
    P: Problem<Encoding = Vec<D>>,
    T: AnyComponent + Recombination<P> + Serialize + Clone,
    D: Clone + PartialEq + 'static,
{
    fn execute(&self, problem: &P, state: &mut State<P>) {
        let population = state.populations_mut().pop();
        let population = population
            .into_iter()
            .map(Individual::into_solution)
            .collect();
        let mut offspring = Vec::new();
        self.0
            .recombine_solutions(population, &mut offspring, problem, state);
        let offspring = offspring
            .into_iter()
            .map(Individual::<P>::new_unevaluated)
            .collect();
        state.populations_mut().push(offspring);
    }
}

// Random Operators without state

pub use crate::components::initialization::RandomPermutation;
impl RandomPermutation {
    /// Creates this component as an generator, modifying the current population.
    pub fn new_gen<P>() -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
    {
        Box::new(Generator(Self {
            initial_population_size: None,
        }))
    }
}
impl<P> Generation<P> for RandomPermutation
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        problem: &P,
        state: &mut State<P>,
    ) {
        let population_size = population.len() as u32;
        *population = self.random_permutation(problem, state.random_mut(), population_size);
    }
}

pub use crate::components::initialization::RandomSpread;
impl RandomSpread {
    /// Creates this component as an generator, modifying the current population.
    pub fn new_gen<P, D>() -> Box<dyn Component<P>>
    where
        D: SampleUniform + Clone + PartialOrd + 'static,
        P: Problem<Encoding = Vec<D>> + LimitedVectorProblem<T = D>,
    {
        Box::new(Generator(Self {
            initial_population_size: None,
        }))
    }
}
impl<P, D> Generation<P> for RandomSpread
where
    D: SampleUniform + Clone + PartialOrd + 'static,
    P: Problem<Encoding = Vec<D>> + LimitedVectorProblem<T = D>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        problem: &P,
        state: &mut State<P>,
    ) {
        let population_size = population.len() as u32;
        *population = self.random_spread(problem, state.random_mut(), population_size);
    }
}

pub use crate::components::initialization::RandomBitstring;
impl RandomBitstring {
    /// Initializes the component with p being the probability for a 1.
    ///
    /// Creates this component as an generator, modifying the current population.
    pub fn new_gen<P>(p: f64) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<bool>> + VectorProblem<T = bool>,
    {
        Box::new(Generator(Self {
            initial_population_size: None,
            p,
        }))
    }

    /// Initializes the component with uniform probability for 0 and 1.
    ///
    /// Creates this component as an generator, modifying the current population.
    pub fn new_uniform_gen<P>() -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<bool>> + VectorProblem<T = bool>,
    {
        Box::new(Generator(Self {
            initial_population_size: None,
            p: 0.5,
        }))
    }
}
impl<P> Generation<P> for RandomBitstring
where
    P: Problem<Encoding = Vec<bool>> + VectorProblem<T = bool>,
{
    fn generate_population(
        &self,
        population: &mut Vec<P::Encoding>,
        problem: &P,
        state: &mut State<P>,
    ) {
        let population_size = population.len() as u32;
        *population = self.random_bitstring(problem, state.random_mut(), population_size);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DuplicatePopulation;
impl DuplicatePopulation {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}
impl<P: Problem> Component<P> for DuplicatePopulation {
    fn execute(&self, _problem: &P, state: &mut State<P>) {
        let population = state.populations_mut().pop();
        let duplicates = population.clone();

        let population = population.into_iter().interleave(duplicates).collect();
        state.populations_mut().push(population);
    }
}
