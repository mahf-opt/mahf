use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use eyre::{ensure, eyre, ContextCompat, WrapErr};
use rand::{distributions::Uniform, Rng};
use rand_distr::Distribution;
use serde::Serialize;

use crate::{
    component::ExecResult, components::Component, population::IntoSingle,
    problems::SingleObjectiveProblem, state::StateReq, CustomState, Individual, Problem, State,
};

#[derive(Clone)]
pub struct Molecule<P: Problem> {
    pub kinetic_energy: f64,
    pub num_hit: u32,
    pub min_hit: u32,
    pub best: Individual<P>,
}

impl<P: Problem> Molecule<P> {
    pub fn new(initial_kinetic_energy: f64, individual: Individual<P>) -> Self {
        Self {
            kinetic_energy: initial_kinetic_energy,
            num_hit: 0,
            min_hit: 0,
            best: individual,
        }
    }
}

impl<P: SingleObjectiveProblem> Molecule<P> {
    pub fn update_best(&mut self, individual: &Individual<P>) -> bool {
        if individual.objective() < self.best.objective() {
            self.best = individual.clone();
            self.min_hit = self.num_hit;
            true
        } else {
            false
        }
    }
}

#[derive(Deref, DerefMut, Tid)]
pub struct ChemicalReaction<P: Problem + 'static>(pub Vec<Molecule<P>>);

impl<P: Problem> Default for ChemicalReaction<P> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<P: Problem> CustomState<'_> for ChemicalReaction<P> {}

#[derive(Deref, DerefMut, Tid)]
pub struct EnergyBuffer(pub f64);

impl CustomState<'_> for EnergyBuffer {}

#[derive(Clone, Serialize)]
pub struct ChemicalReactionInit {
    kinetic_energy: f64,
    buffer: f64,
}

impl ChemicalReactionInit {
    pub fn from_params(kinetic_energy: f64, buffer: f64) -> Self {
        Self {
            kinetic_energy,
            buffer,
        }
    }

    pub fn new<P: SingleObjectiveProblem>(
        kinetic_energy: f64,
        buffer: f64,
    ) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(kinetic_energy, buffer))
    }
}

impl<P> Component<P> for ChemicalReactionInit
where
    P: SingleObjectiveProblem,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(ChemicalReaction::<P>::default());
        state.insert(EnergyBuffer(self.buffer));
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        *state.borrow_value_mut::<ChemicalReaction<P>>() = state
            .populations()
            .current()
            .iter()
            .map(|i| Molecule::new(self.kinetic_energy, i.clone()))
            .collect();
        Ok(())
    }
}

/// Updates state after an OnWallIneffectiveCollision.
///
/// Updates the energy buffer and molecule data in [CroState].
///
/// It assumes the following [Population][crate::state::common::Population] structure:
/// - One mutated individual i'
/// - One selected individual i
/// - Population
///
/// Note that this component does **NOT** perform the operation, but only updates state afterwards.
#[derive(Clone, Serialize)]
pub struct OnWallIneffectiveCollisionUpdate {
    /// The kinetic energy loss rate.
    pub kinetic_energy_lr: f64,
}

impl OnWallIneffectiveCollisionUpdate {
    pub fn from_params(kinetic_energy_lr: f64) -> Self {
        Self { kinetic_energy_lr }
    }

    pub fn new<P: SingleObjectiveProblem>(kinetic_energy_lr: f64) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(kinetic_energy_lr))
    }
}

impl<P> Component<P> for OnWallIneffectiveCollisionUpdate
where
    P: SingleObjectiveProblem,
{
    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, ChemicalReaction<P>>()?;
        state_req.require::<Self, EnergyBuffer>()?;
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let mut reaction = state.borrow_mut::<ChemicalReaction<P>>();
        let mut buffer = state.borrow_value_mut::<EnergyBuffer>();

        // Get reaction reactant `r` and product `p`
        ensure!(
            populations.len() >= 3,
            "invalid population stack layout for this component"
        );
        let p = populations
            .pop()
            .into_single()
            .wrap_err("expected a single individual as product")?;
        let r = populations
            .pop()
            .into_single()
            .wrap_err("expected a single individual as reactant")?;

        let r_idx = populations
            .current()
            .iter()
            .position(|i| i == &r)
            .wrap_err("couldn't find reactant in population")?;
        reaction[r_idx].num_hit += 1;

        let total_reactant_energy = r.objective().value() + reaction[r_idx].kinetic_energy;
        let product_energy = p.objective().value();

        // Reaction is possible
        if total_reactant_energy >= product_energy {
            let alpha = rng.gen_range(self.kinetic_energy_lr..1.0);

            // Update buffer
            *buffer += (total_reactant_energy - product_energy) * (1. - alpha);

            // Update best with new
            reaction[r_idx].update_best(&p);

            // Replace individual
            reaction[r_idx].kinetic_energy = (total_reactant_energy - product_energy) * alpha;
            populations.current_mut()[r_idx] = p;
        }
        Ok(())
    }
}

/// Updates state after a Decomposition.
///
/// Updates the energy buffer and molecule data in [CroState].
///
/// It assumes the following [Population][crate::state::common::Population] structure:
/// - Two mutated individuals i' and i''
/// - One selected individual i
/// - Population
///
/// Note that this component does **NOT** perform the operation, but only updates state afterwards.
#[derive(Clone, Serialize)]
pub struct DecompositionUpdate;

impl DecompositionUpdate {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P> Component<P> for DecompositionUpdate
where
    P: SingleObjectiveProblem,
{
    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, ChemicalReaction<P>>()?;
        state_req.require::<Self, EnergyBuffer>()?;
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let mut reaction = state.borrow_mut::<ChemicalReaction<P>>();
        let mut buffer = state.borrow_value_mut::<EnergyBuffer>();

        // Get reaction reactant `r` and products `p1`, `p2`
        ensure!(
            populations.len() >= 3,
            "invalid population stack layout for this component"
        );
        let [p1, p2]: [_; 2] = populations
            .pop()
            .try_into()
            .map_err(|_| eyre!("expected two individuals as products"))?;
        let r = populations
            .pop()
            .into_single()
            .wrap_err("expected a single individual as reactant")?;

        let r_idx = populations
            .current()
            .iter()
            .position(|i| i == &r)
            .wrap_err("couldn't find reactant in population")?;

        let total_reactant_energy = r.objective().value() + reaction[r_idx].kinetic_energy;
        let products_energy = p1.objective().value() + p2.objective().value();

        let decomposition_energy = if total_reactant_energy >= products_energy {
            total_reactant_energy - products_energy
        } else {
            let deltas: f64 = Uniform::new(0., 1.)
                .sample_iter(&mut *rng)
                .take(2)
                .product();
            let decomposition_energy = total_reactant_energy + deltas * *buffer - products_energy;

            // Not enough energy for reaction even with buffer, so abort
            if decomposition_energy < 0. {
                reaction[r_idx].num_hit += 1;
                return Ok(());
            }

            *buffer *= 1. - deltas;
            decomposition_energy
        };

        // Initialize molecule data and insert new individuals
        let d3 = rng.gen_range(0.0..=1.0);

        reaction[r_idx] = Molecule::new(decomposition_energy * d3, p1.clone());
        reaction.push(Molecule::new(decomposition_energy * (1. - d3), p2.clone()));

        let population = populations.current_mut();
        population[r_idx] = p1;
        population.push(p2);
        Ok(())
    }
}

/// Updates state after an IntermolecularIneffectiveCollision.
///
/// Updates the energy buffer and molecule data in [CroState].
///
/// It assumes the following [Population][crate::state::common::Population] structure:
/// - Two mutated individuals i' and j'
/// - Two selected individuals i and j
/// - Population
///
/// Note that this component does **NOT** perform the operation, but only updates state afterwards.
#[derive(Debug, serde::Serialize, Clone)]
pub struct IntermolecularIneffectiveCollisionUpdate;

impl IntermolecularIneffectiveCollisionUpdate {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P> Component<P> for IntermolecularIneffectiveCollisionUpdate
where
    P: SingleObjectiveProblem,
{
    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, ChemicalReaction<P>>()?;
        state_req.require::<Self, EnergyBuffer>()?;
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let mut reaction = state.borrow_mut::<ChemicalReaction<P>>();

        // Get reaction reactants `r1`, `r2` and products `p1`, `p2`
        ensure!(
            populations.len() >= 3,
            "invalid population stack layout for this component"
        );

        let [p1, p2]: [_; 2] = populations
            .pop()
            .try_into()
            .map_err(|_| eyre!("expected two individuals as products"))?;
        let [r1, r2]: [_; 2] = populations
            .pop()
            .try_into()
            .map_err(|_| eyre!("expected two individuals as reactants"))?;

        let r1_idx = populations
            .current()
            .iter()
            .position(|i| i == &r1)
            .wrap_err("couldn't find reactant in population")?;
        let r2_idx = populations
            .current()
            .iter()
            .position(|i| i == &r2)
            .wrap_err("couldn't find reactant in population")?;
        ensure!(
            r1_idx != r2_idx,
            "the same molecule can't be used as two reactants"
        );

        reaction[r1_idx].num_hit += 1;
        reaction[r2_idx].num_hit += 1;

        let total_r1_energy = r1.objective().value() + reaction[r1_idx].kinetic_energy;
        let total_r2_energy = r2.objective().value() + reaction[r2_idx].kinetic_energy;
        let total_reactants_energy = total_r1_energy + total_r2_energy;

        let p1_energy = p1.objective().value();
        let p2_energy = p2.objective().value();
        let products_energy = p1_energy + p2_energy;

        let collision_energy = total_reactants_energy - products_energy;

        // Reaction is possible
        if collision_energy >= 0. {
            let d4 = rng.gen_range(0.0..=1.0);

            // Update kinetic energies
            reaction[r1_idx].kinetic_energy = collision_energy * d4;
            reaction[r2_idx].kinetic_energy = collision_energy * (1. - d4);

            // Update best with new
            reaction[r1_idx].update_best(&p1);
            reaction[r2_idx].update_best(&p2);

            // Replace individual
            let population = populations.current_mut();
            population[r1_idx] = p1;
            population[r2_idx] = p2;
        }

        Ok(())
    }
}

#[derive(Clone, Serialize)]
pub struct SynthesisUpdate;

impl SynthesisUpdate {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P: SingleObjectiveProblem>() -> Box<dyn Component<P>> {
        Box::new(Self::from_params())
    }
}

impl<P> Component<P> for SynthesisUpdate
where
    P: SingleObjectiveProblem,
{
    fn require(&self, _problem: &P, state_req: &StateReq<P>) -> ExecResult<()> {
        state_req.require::<Self, ChemicalReaction<P>>()?;
        state_req.require::<Self, EnergyBuffer>()?;
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();

        let mut reaction = state.borrow_mut::<ChemicalReaction<P>>();

        // Get reaction reactants r1, r2 and product
        ensure!(
            populations.len() >= 3,
            "invalid population stack layout for this component"
        );

        let p = populations
            .pop()
            .into_single()
            .wrap_err("expected a single individual as product")?;
        let [r1, r2]: [_; 2] = populations
            .pop()
            .try_into()
            .map_err(|_| eyre!("expected two individuals as products"))?;

        let r1_idx = populations.current().iter().position(|i| i == &r1).unwrap();
        let r2_idx = populations.current().iter().position(|i| i == &r2).unwrap();
        ensure!(
            r1_idx != r2_idx,
            "the same molecule can't be used as two reactants"
        );

        let total_r1_energy = r1.objective().value() + reaction[r1_idx].kinetic_energy;
        let total_r2_energy = r2.objective().value() + reaction[r2_idx].kinetic_energy;
        let total_reactants_energy = total_r1_energy + total_r2_energy;

        let product_energy = p.objective().value();

        // Reaction is possible
        if total_reactants_energy >= product_energy {
            // Replace one molecule and remove other
            reaction[r1_idx] = Molecule::new(total_reactants_energy - product_energy, p.clone());
            reaction.remove(r2_idx);

            // Replace one individual and remove other
            let population = populations.current_mut();
            population[r1_idx] = p;
            population.remove(r2_idx);
        }

        Ok(())
    }
}
