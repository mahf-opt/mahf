use crate::{
    functions::Problem,
    modular::{Individual, Solution, State},
};

pub trait Initialization {
    fn initialize(&mut self, problem: &Problem, population: &mut Vec<Solution>);
}

pub trait Selection {
    fn select<'p>(
        &mut self,
        state: &mut State,
        population: &'p [Individual],
        selection: &mut Vec<&'p Solution>,
    );
}

pub trait Generation {
    fn generate(
        &mut self,
        state: &mut State,
        problem: &Problem,
        parents: &mut Vec<&Solution>,
        offspring: &mut Vec<Solution>,
    );
}

pub trait Replacement {
    fn replace(
        &mut self,
        state: &mut State,
        population: &mut Vec<Individual>,
        offspring: &mut Vec<Individual>,
    );
}

pub trait Termination {
    fn terminate(&mut self, state: &mut State) -> bool;
}
