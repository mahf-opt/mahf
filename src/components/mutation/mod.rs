use crate::{
    component::{AnyComponent, ExecResult},
    population::AsSolutionsMut,
    Problem, State,
};

pub mod common;
pub mod de;
pub mod functional;

pub use common::{
    BitFlipMutation, InversionMutation, MutationRate, MutationStrength, NormalMutation,
    PartialRandomBitstring, PartialRandomSpread, ScrambleMutation, SwapMutation,
    TranslocationMutation, UniformMutation,
};

pub trait Mutation<P: Problem>: AnyComponent {
    fn mutate(
        &self,
        solution: &mut P::Encoding,
        problem: &P,
        state: &mut State<P>,
    ) -> ExecResult<()>;
}

pub fn mutation<T, P>(component: &T, problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: Mutation<P>,
{
    let mut population = state.populations_mut().pop();
    for solution in population.as_solutions_mut() {
        component.mutate(solution, problem, state)?;
    }
    state.populations_mut().push(population);

    Ok(())
}
