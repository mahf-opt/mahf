//! Generation methods

use crate::{
    framework::{
        components::{AnyComponent, Component},
        Individual, State,
    },
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};
use rand::distributions::uniform::SampleUniform;
use serde::Serialize;

pub mod mutation;
pub mod recombination;

/// Specialized component trait to generate a new population from the current one.
///
/// This trait is especially useful for components which modify solutions independently.
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
        state: &mut State,
    );
}

#[derive(serde::Serialize)]
pub struct Generator<T>(pub T);

impl<T, P> Component<P> for Generator<T>
where
    P: Problem,
    T: AnyComponent + Generation<P> + Serialize,
{
    fn execute(&self, problem: &P, state: &mut State) {
        let population = state.population_stack_mut().pop();
        let mut population = population
            .into_iter()
            .map(Individual::into_solution)
            .collect();
        self.0.generate_population(&mut population, problem, state);
        let population = population
            .into_iter()
            .map(Individual::new_unevaluated::<P::Encoding, P::Objective>)
            .collect();
        state.population_stack_mut().push(population);
    }
}

/// Specialized component trait to generate a new population from the current one.
///
/// This trait is especially useful for components which combine multiple solutions.
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
        state: &mut State,
    );
}

#[derive(serde::Serialize)]
pub struct Recombinator<T>(pub T);

impl<T, P, D> Component<P> for Recombinator<T>
where
    P: Problem<Encoding = Vec<D>>,
    T: AnyComponent + Recombination<P> + Serialize,
    D: Clone + PartialEq + 'static,
{
    fn execute(&self, problem: &P, state: &mut State) {
        let population = state.population_stack_mut().pop();
        let population = population
            .into_iter()
            .map(Individual::into_solution)
            .collect();
        let mut offspring = Vec::new();
        self.0
            .recombine_solutions(population, &mut offspring, problem, state);
        let offspring = offspring
            .into_iter()
            .map(Individual::new_unevaluated::<P::Encoding, P::Objective>)
            .collect();
        state.population_stack_mut().push(offspring);
    }
}

// Random Operators without state

pub use crate::operators::initialization::RandomPermutation;
impl RandomPermutation {
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
        state: &mut State,
    ) {
        let population_size = population.len() as u32;
        *population = self.random_permutation(problem, state.random_mut(), population_size);
    }
}

pub use crate::operators::initialization::RandomSpread;
impl RandomSpread {
    /// Create this component as an generator, modifying the current population.
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
        state: &mut State,
    ) {
        let population_size = population.len() as u32;
        *population = self.random_spread(problem, state.random_mut(), population_size);
    }
}

pub mod swarm {
    use rand::distributions::Uniform;
    use rand::Rng;

    use crate::random::Random;
    use crate::{
        framework::{components::*, Individual, State},
        operators::custom_state::PsoState,
        problems::SingleObjectiveProblem,
    };

    /// Applies the PSO specific generation operator.
    ///
    /// Requires [PsoStateUpdate][crate::heuristics::pso::pso_ops::PsoStateUpdate].
    #[derive(serde::Serialize)]
    pub struct PsoGeneration {
        pub a: f64,
        pub b: f64,
        pub c: f64,
        pub v_max: f64,
    }
    impl PsoGeneration {
        pub fn new<P>(a: f64, b: f64, c: f64, v_max: f64) -> Box<dyn Component<P>>
        where
            P: SingleObjectiveProblem<Encoding = Vec<f64>>,
        {
            Box::new(Self { a, b, c, v_max })
        }
    }
    impl<P> Component<P> for PsoGeneration
    where
        P: SingleObjectiveProblem<Encoding = Vec<f64>>,
    {
        fn initialize(&self, _problem: &P, state: &mut State) {
            state.require::<PsoState>();
        }

        fn execute(&self, _problem: &P, state: &mut State) {
            let &Self { a, b, c, v_max } = self;

            let mut offspring = Vec::new();
            let mut parents = state.population_stack_mut().pop();

            let rng = state.random_mut();
            let rng_iter = |rng: &mut Random| {
                rng.sample_iter(Uniform::new(0., 1.))
                    .take(parents.len())
                    .collect::<Vec<_>>()
            };

            let rs = rng_iter(rng);
            let rt = rng_iter(rng);

            let pso_state = state.get_mut::<PsoState>();

            for (i, x) in parents.drain(..).enumerate() {
                let mut x = x.into_solution::<Vec<f64>>();
                let v = &mut pso_state.velocities[i];
                let xl = pso_state.bests[i].solution::<Vec<f64>>();
                let xg = pso_state.global_best.solution::<Vec<f64>>();

                for i in 0..v.len() {
                    v[i] = a * v[i] + b * rs[i] * (xg[i] - x[i]) + c * rt[i] * (xl[i] - x[i]);
                    v[i] = v[i].clamp(-v_max, v_max);
                }

                for i in 0..x.len() {
                    x[i] = (x[i] + v[i]).clamp(-1.0, 1.0);
                }

                offspring.push(Individual::new_unevaluated::<P::Encoding, P::Objective>(x));
            }

            state.population_stack_mut().push(offspring);
        }
    }
}
