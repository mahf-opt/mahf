use dyn_clone::DynClone;
use serde::{Serialize, Serializer};

use crate::{
    framework::components::Component,
    problems::{Problem, SingleObjectiveProblem},
    state::State,
};

/// Doesn't do anything.
///
/// Note that this component is different from [initialization::Empty] as it doesn't modify
/// the state at all, while [Empty][initialization::Empty] pushes an empty population on the stack.
#[derive(Serialize, Clone)]
pub struct Noop;
impl Noop {
    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: Problem,
    {
        Box::new(Self)
    }
}
impl<P: Problem> Component<P> for Noop {
    fn execute(&self, _problem: &P, _state: &mut State) {
        // Noop
    }
}

/// Clears the current population, deleting all individuals.
#[derive(Serialize, Clone)]
pub struct ClearPopulation;
impl ClearPopulation {
    pub fn new<P: Problem>() -> Box<dyn Component<P>> {
        Box::new(Self)
    }
}
impl<P: Problem> Component<P> for ClearPopulation {
    fn execute(&self, _problem: &P, state: &mut State) {
        state.population_stack_mut::<P>().current_mut().clear();
    }
}

/// Helper trait to allow cloning of debug functions.
pub trait DynCustomFunc<P: Problem>: Fn(&P, &mut State) + Send + Sync + DynClone + 'static {}
dyn_clone::clone_trait_object!(<P: Problem> DynCustomFunc<P>);

impl<P: Problem, F> DynCustomFunc<P> for F where
    F: Fn(&P, &mut State) + Send + Sync + Clone + 'static
{
}

/// Allows for minor custom behaviour for debug purposes, e.g., asserts.
///
/// The contents of the function passed to this component are **NOT** serialized.
///
/// Note that this is for debug **ONLY**.
/// The recommended way of implementing larger custom functionality is to implement
/// [Component] for your struct.
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Debug<P: Problem>(Box<dyn DynCustomFunc<P>>);
impl<P: Problem> Debug<P> {
    pub fn new(
        custom: impl Fn(&P, &mut State) + Send + Sync + Clone + 'static,
    ) -> Box<dyn Component<P>> {
        Box::new(Self(Box::new(custom)))
    }
}
impl<P: Problem> Component<P> for Debug<P> {
    fn execute(&self, problem: &P, state: &mut State) {
        self.0(problem, state);
    }
}

impl<P: Problem> Serialize for Debug<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_struct("Debug")
    }
}

/// Prints a summary of the current [State] for single-objective problems.
/// The summary includes statistics like number of iterations, evaluations and best solution found yet.
#[derive(Serialize, Clone)]
pub struct PrintSingleObjectiveSummary;
impl PrintSingleObjectiveSummary {
    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>>
    where
        P::Encoding: std::fmt::Debug,
    {
        Box::new(Self)
    }
}
impl<P: SingleObjectiveProblem> Component<P> for PrintSingleObjectiveSummary
where
    P::Encoding: std::fmt::Debug,
{
    fn execute(&self, _problem: &P, state: &mut State) {
        let heading = "--- SUMMARY ---";
        println!("{}", heading);
        println!("Iterations: {}", state.iterations());
        println!("Evaluations: {}", state.evaluations());

        if let Some(individual) = state.best_individual::<P>() {
            println!("Optimum found: {:?}", individual.solution());
            println!("Best objective value: {:?}", individual.objective());
        } else {
            println!("No solution found.")
        }
        println!("{}", "-".repeat(heading.len()));
    }
}
