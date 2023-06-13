use eyre::{eyre, ContextCompat};
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult, components::bound::cro::ChemicalReaction, conditions::Condition,
    population::IntoSingleRef, Problem, State,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct DecompositionCriterion {
    // Balances between diversification (Decomposition) and intensification (OnWallIneffectiveCollision).
    // A lower value makes Decomposition more likely.
    alpha: u32,
}

impl DecompositionCriterion {
    pub fn from_params(alpha: u32) -> Self {
        Self { alpha }
    }

    pub fn new<P>(alpha: u32) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self::from_params(alpha))
    }
}

impl<P> Condition<P> for DecompositionCriterion
where
    P: Problem,
{
    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let populations = state.populations();
        let reaction = state.borrow::<ChemicalReaction<P>>();

        let selected = populations
            .peek(0)
            .into_single_ref()
            .wrap_err("expected a single individual")?;
        let population = populations.peek(1);

        let selected_idx = population.iter().position(|i| i == selected).unwrap();
        let molecule = &reaction[selected_idx];

        Ok(molecule.num_hit - molecule.min_hit > self.alpha)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SynthesisCriterion {
    // Balances between diversification (Synthesis) and intensification (IntermolecularIneffectiveCollision).
    // A higher value makes Synthesis more likely.
    beta: f64,
}

impl SynthesisCriterion {
    pub fn from_params(beta: f64) -> Self {
        Self { beta }
    }

    pub fn new<P>(beta: f64) -> Box<dyn Condition<P>>
    where
        P: Problem,
    {
        Box::new(Self::from_params(beta))
    }
}

impl<P> Condition<P> for SynthesisCriterion
where
    P: Problem,
{
    fn evaluate(&self, _problem: &P, state: &mut State<P>) -> ExecResult<bool> {
        let populations = state.populations();
        let reaction = state.borrow::<ChemicalReaction<P>>();

        let [s1, s2]: &[_; 2] = populations
            .peek(0)
            .try_into()
            .map_err(|_| eyre!("expected two individuals"))?;
        let population = populations.peek(1);

        let s1_idx = population.iter().position(|i| i == s1).unwrap();
        let s1_molecule = &reaction[s1_idx];
        let s2_idx = population.iter().position(|i| i == s2).unwrap();
        let s2_molecule = &reaction[s2_idx];

        Ok(s1_molecule.kinetic_energy <= self.beta && s2_molecule.kinetic_energy <= self.beta)
    }
}
