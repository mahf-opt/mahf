use crate::{
    heuristic::{Individual, State},
    problem::Problem,
};

pub trait Initialization<P: Problem> {
    fn initialize(&mut self, problem: &P, population: &mut Vec<P::Encoding>);
}

pub trait Selection {
    fn select<'p>(
        &mut self,
        state: &mut State,
        population: &'p [Individual],
        selection: &mut Vec<&'p Individual>,
    );
}

pub trait Generation<P: Problem> {
    fn generate(
        &mut self,
        state: &mut State,
        problem: &P,
        parents: &mut Vec<&P::Encoding>,
        offspring: &mut Vec<P::Encoding>,
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
