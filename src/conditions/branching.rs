//! Branching conditions

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{framework::conditions::Condition, problems::Problem, state, state::State};

#[derive(Serialize, Deserialize, Clone)]
pub struct RandomChance {
    // Probability of the condition evaluating to `true`.
    p: f64,
}
impl RandomChance {
    pub fn new<P>(p: f64) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self { p })
    }
}
impl<P> Condition<P> for RandomChance
where
    P: Problem,
{
    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        state.random_mut().gen_bool(self.p)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompositionCriterion {
    // Balances between diversification (Decomposition) and intensification (OnWallIneffectiveCollision).
    // A lower value makes Decomposition more likely.
    alpha: u32,
}
impl DecompositionCriterion {
    pub fn new<P>(alpha: u32) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self { alpha })
    }
}
impl<P> Condition<P> for DecompositionCriterion
where
    P: Problem,
{
    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        let mut mut_state = state.get_states_mut();
        let cro_state = mut_state.get::<state::CroState<P>>();
        let stack = mut_state.population_stack::<P>();

        let selected = stack.peek(0).first().unwrap();
        let population = stack.peek(1);

        let selected_index = cro_state.molecule_index(population, selected);
        let molecule = cro_state.molecule(selected_index);

        molecule.num_hit - molecule.min_hit > self.alpha
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SynthesisCriterion {
    // Balances between diversification (Synthesis) and intensification (IntermolecularIneffectiveCollision).
    // A higher value makes Synthesis more likely.
    beta: f64,
}
impl SynthesisCriterion {
    pub fn new<P>(beta: f64) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self { beta })
    }
}
impl<P> Condition<P> for SynthesisCriterion
where
    P: Problem,
{
    fn evaluate(&self, _problem: &P, state: &mut State) -> bool {
        let mut mut_state = state.get_states_mut();
        let cro_state = mut_state.get::<state::CroState<P>>();
        let stack = mut_state.population_stack::<P>();

        let [s1, s2] = TryInto::<&[_; 2]>::try_into(stack.peek(0)).unwrap();
        let population = stack.peek(1);

        let s1_index = cro_state.molecule_index(population, s1);
        let s1_molecule = cro_state.molecule(s1_index);
        let s2_index = cro_state.molecule_index(population, s2);
        let s2_molecule = cro_state.molecule(s2_index);

        s1_molecule.kinetic_energy <= self.beta && s2_molecule.kinetic_energy <= self.beta
    }
}
