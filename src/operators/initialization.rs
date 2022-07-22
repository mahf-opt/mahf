//! Initialization methods

use rand::seq::SliceRandom;
use rand::{distributions::uniform::SampleUniform, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    framework::{components::*, state::State, Individual, Random},
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

/// Specialized component trait to initialize a new population on the stack.
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Initializer].
pub trait Initialization<P: Problem>: AnyComponent {
    fn initialize_population(&self, problem: &P, state: &mut State) -> Vec<Individual>;
}

#[derive(Serialize)]
pub struct Initializer<T>(pub T);

impl<T, P> Component<P> for Initializer<T>
where
    P: Problem,
    T: AnyComponent + Initialization<P> + Serialize,
{
    fn execute(&self, problem: &P, state: &mut State) {
        let population = self.0.initialize_population(problem, state);
        state.population_stack_mut().push(population);
    }
}

/// Initializes an empty population.
#[derive(Serialize)]
pub struct Empty;
impl Empty {
    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: Problem,
    {
        Box::new(Initializer(Self))
    }
}
impl<P: Problem> Initialization<P> for Empty {
    fn initialize_population(&self, _problem: &P, _state: &mut State) -> Vec<Individual> {
        Vec::new()
    }
}

/// Generates new random solutions in the search space.
#[derive(Serialize, Deserialize)]
pub struct RandomSpread {
    /// Size of the initial population.
    pub initial_population_size: Option<u32>,
}
impl RandomSpread {
    /// Create this component as an initializer, pushing a new population on the stack.
    pub fn new_init<P, D>(initial_population_size: u32) -> Box<dyn Component<P>>
    where
        D: SampleUniform + Clone + PartialOrd + 'static,
        P: Problem<Encoding = Vec<D>> + LimitedVectorProblem<T = D>,
    {
        Box::new(Initializer(Self {
            initial_population_size: Some(initial_population_size),
        }))
    }

    pub(crate) fn random_spread<P, D>(
        &self,
        problem: &P,
        rng: &mut Random,
        population_size: u32,
    ) -> Vec<P::Encoding>
    where
        D: SampleUniform + Clone + PartialOrd + 'static,
        P: Problem<Encoding = Vec<D>> + LimitedVectorProblem<T = D>,
    {
        let mut population = Vec::new();

        for _ in 0..population_size {
            let solution = (0..problem.dimension())
                .map(|d| rng.gen_range(problem.range(d)))
                .collect::<Vec<D>>();

            population.push(solution);
        }
        population
    }
}
impl<P, D> Initialization<P> for RandomSpread
where
    D: SampleUniform + Clone + PartialOrd + 'static,
    P: Problem<Encoding = Vec<D>> + LimitedVectorProblem<T = D>,
{
    fn initialize_population(&self, problem: &P, state: &mut State) -> Vec<Individual> {
        let population_size = self.initial_population_size.unwrap();
        self.random_spread(problem, state.random_mut(), population_size)
            .into_iter()
            .map(Individual::new_unevaluated)
            .collect()
    }
}

/// Generates new random permutations for combinatorial problems.
#[derive(Serialize, Deserialize)]
pub struct RandomPermutation {
    /// Size of the initial population.
    pub initial_population_size: Option<u32>,
}
impl RandomPermutation {
    pub fn new_init<P>(initial_population_size: u32) -> Box<dyn Component<P>>
    where
        P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
    {
        Box::new(Initializer(Self {
            initial_population_size: Some(initial_population_size),
        }))
    }

    pub(crate) fn random_permutation<P>(
        &self,
        problem: &P,
        rng: &mut Random,
        population_size: u32,
    ) -> Vec<P::Encoding>
    where
        P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
    {
        let mut population = Vec::new();
        for _ in 0..population_size {
            let mut solution = (0..problem.dimension()).collect::<Vec<usize>>();
            solution.shuffle(rng);
            population.push(solution);
        }
        population
    }
}
impl<P> Initialization<P> for RandomPermutation
where
    P: Problem<Encoding = Vec<usize>> + VectorProblem<T = usize>,
{
    fn initialize_population(&self, problem: &P, state: &mut State) -> Vec<Individual> {
        let population_size = self.initial_population_size.unwrap();
        self.random_permutation(problem, state.random_mut(), population_size)
            .into_iter()
            .map(Individual::new_unevaluated)
            .collect()
    }
}
