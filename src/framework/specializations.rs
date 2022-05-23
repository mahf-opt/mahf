use serde::Serialize;

use crate::framework::common_state::Population;
use crate::framework::components::{AnyComponent, Component};
use crate::framework::{Individual, State};
use crate::problems::Problem;

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

/// Specialized component trait to select a subset of the current population and push it on the stack.
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Selector].
pub trait Selection {
    fn select_offspring<'p>(
        &self,
        population: &'p [Individual],
        state: &mut State,
    ) -> Vec<&'p Individual>;
}

#[derive(serde::Serialize)]
pub struct Selector<T>(pub T);

impl<T, P> Component<P> for Selector<T>
where
    P: Problem,
    T: AnyComponent + Selection + Serialize,
{
    fn execute(&self, _problem: &P, state: &mut State) {
        let population = state.population_stack_mut().pop();
        let selection: Vec<_> = self
            .0
            .select_offspring(&population, state)
            .into_iter()
            .cloned()
            .collect();
        state.population_stack_mut().push(population);
        state.population_stack_mut().push(selection);
    }
}

/// Specialized component trait to generate a new population from the current one.
///
/// This trait is especially useful for components which modify solutions independently.
/// For mixing multiple solutions, see [Recombination].
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Generator].
pub trait Generation<P> {
    fn generate_population(&self, population: &mut Vec<Individual>, problem: &P, state: &mut State);
}

#[derive(serde::Serialize)]
pub struct Generator<T>(pub T);

impl<T, P> Component<P> for Generator<T>
where
    P: Problem,
    T: AnyComponent + Generation<P> + Serialize,
{
    fn execute(&self, problem: &P, state: &mut State) {
        let mut population = state.population_stack_mut().pop();
        self.0.generate_population(&mut population, problem, state);
        state.population_stack_mut().push(population);
    }
}

/// Specialized component trait to mutate individual positions or solutions as a whole.
///
/// This is a sub-trait to [Generation].
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type either
/// in a [PositionMutator] or [SolutionMutator], depending on the use case.
/// The mutator must then be wrapped in a [Generator] to implement [Component].
pub trait Mutation<P, F> {
    fn mutation_func<'p>(
        &self,
        problem: &'p P,
        state: &'p mut State,
    ) -> Box<dyn FnMut(&mut F) + 'p>;
}

/// Implements [Generation] by calling [Mutation::mutation_func] on individual solution positions.
#[derive(serde::Serialize)]
pub struct PositionMutator<T>(pub T);

impl<T, P, F> Generation<P> for PositionMutator<T>
where
    P: Problem,
    for<'a> &'a mut <P as Problem>::Encoding: IntoIterator<Item = &'a mut F>,
    T: AnyComponent + Mutation<P, F> + Serialize,
{
    fn generate_population(
        &self,
        population: &mut Vec<Individual>,
        problem: &P,
        state: &mut State,
    ) {
        let mut mutation = self.0.mutation_func(problem, state);
        for solution in population.iter_mut() {
            let solution = solution.solution_mut::<P::Encoding>();
            for x in solution.into_iter() {
                mutation(x);
            }
        }
    }
}

/// Implements [Generation] by calling [Mutation::mutation_func] on individual solutions.
#[derive(serde::Serialize)]
pub struct SolutionMutator<T>(pub T);

impl<T, P, D> Generation<P> for SolutionMutator<T>
where
    D: Clone + 'static,
    P: Problem<Encoding = Vec<D>>,
    T: AnyComponent + Mutation<P, Vec<D>> + Serialize,
{
    fn generate_population(
        &self,
        population: &mut Vec<Individual>,
        problem: &P,
        state: &mut State,
    ) {
        let mut mutation = self.0.mutation_func(problem, state);
        for solution in population.iter_mut() {
            let solution = solution.solution_mut::<P::Encoding>();
            mutation(solution);
        }
    }
}

/// Specialized component trait to generate a new population from the current one.
///
/// This trait is especially useful for components which mix multiple solutions.
/// For modifying solutions independently, see [Generation].
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Recombinator].
pub trait Recombination<P, F> {
    fn recombine_solutions(&self, parents: Vec<F>, offspring: &mut Vec<F>, state: &mut State);
}

#[derive(serde::Serialize)]
pub struct Recombinator<T>(pub T);

impl<T, P, D> Component<P> for Recombinator<T>
where
    P: Problem<Encoding = Vec<D>>,
    T: AnyComponent + Recombination<P, Vec<D>> + Serialize,
    D: Clone + PartialEq + 'static,
{
    fn execute(&self, _problem: &P, state: &mut State) {
        let population = state.population_stack_mut().pop();
        let population = population
            .into_iter()
            .map(|i| i.into_solution::<P::Encoding>())
            .collect();
        let mut offspring = Vec::new();
        self.0
            .recombine_solutions(population, &mut offspring, state);
        let offspring = offspring
            .into_iter()
            .map(Individual::new_unevaluated)
            .collect();
        state.population_stack_mut().push(offspring);
    }
}

/// Specialized component trait to replace the population with the child population,
/// typically generated by a [Selection] component.
/// Merges the two topmost populations on the stack.
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [Replacer].
pub trait Replacement {
    fn replace_population(
        &self,
        parents: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
        state: &mut State,
    );
}

#[derive(serde::Serialize)]
pub struct Replacer<T>(pub T);

impl<T, P> Component<P> for Replacer<T>
where
    P: Problem,
    T: AnyComponent + Replacement + Serialize,
{
    fn execute(&self, _problem: &P, state: &mut State) {
        let mut offspring = state.get_mut::<Population>().pop();
        let mut parents = state.get_mut::<Population>().pop();
        self.0
            .replace_population(&mut parents, &mut offspring, state);
        state.population_stack_mut().push(parents);
    }
}
