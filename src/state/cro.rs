use crate::{
    framework::{components::*, Individual},
    problems::{Problem, SingleObjectiveProblem},
    state::{CustomState, State},
};
use better_any::Tid;
use rand::Rng;

#[derive(Clone)]
pub struct Molecule<P: Problem> {
    pub kinetic_energy: f64,
    pub num_hit: u32,
    pub min_hit: u32,
    pub best: Individual<P>,
}

impl<P: Problem> Molecule<P> {
    pub fn new(initial_kinetic_energy: f64, individual: &Individual<P>) -> Self {
        Self {
            kinetic_energy: initial_kinetic_energy,
            num_hit: 0,
            min_hit: 0,
            best: individual.clone(),
        }
    }
}

impl<P: SingleObjectiveProblem> Molecule<P> {
    pub fn update_best(&mut self, individual: &Individual<P>) -> bool {
        if individual.objective().value() < self.best.objective().value() {
            self.best = individual.clone();
            self.min_hit = self.num_hit;
            true
        } else {
            false
        }
    }
}

/// State required for CRO.
///
/// For preserving energy buffer level and molecule data.
#[derive(Tid)]
pub struct CroState<P: Problem + 'static> {
    pub buffer: f64,
    pub molecules: Vec<Molecule<P>>,
}
impl<P: Problem> CustomState<'_> for CroState<P> {}

#[derive(Debug, serde::Serialize, Clone)]
pub struct CroStateInitialization {
    initial_kinetic_energy: f64,
    buffer: f64,
}

impl<P> Component<P> for CroStateInitialization
where
    P: SingleObjectiveProblem,
{
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        // Initialize with empty state to satisfy `state.require()` statements
        state.insert(CroState::<P> {
            buffer: 0.,
            molecules: Vec::new(),
        })
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) {
        let population = state.populations().current();
        let molecules = population
            .iter()
            .map(|i| Molecule::new(self.initial_kinetic_energy, i))
            .collect();

        state.insert(CroState {
            buffer: self.buffer,
            molecules,
        });
    }
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct OnWallIneffectiveCollisionUpdate {
    pub kinetic_energy_loss_rate: f64,
}

impl<P> Component<P> for OnWallIneffectiveCollisionUpdate
where
    P: SingleObjectiveProblem,
{
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.require::<Self, CroState<P>>();
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) {
        let mut mut_state = state.get_states_mut();
        let populations = mut_state.populations_mut();
        let mut cro_state = mut_state.get_mut::<CroState<P>>();
        let rng = mut_state.random_mut();

        // Get reaction reactant and product
        let product = populations.pop().into_iter().next().unwrap();
        let reactant = populations.pop().into_iter().next().unwrap();

        let reactant_index = populations
            .current()
            .iter()
            .position(|i| i == &reactant)
            .unwrap();

        cro_state.molecules[reactant_index].num_hit += 1;

        let total_reactant_energy =
            reactant.objective().value() + cro_state.molecules[reactant_index].kinetic_energy;
        let product_energy = product.objective().value();

        // Reaction is possible
        if total_reactant_energy >= product_energy {
            let alpha = rng.gen_range(self.kinetic_energy_loss_rate..1.0);

            // Update buffer
            cro_state.buffer += (total_reactant_energy - product_energy) * (1. - alpha);

            // Update best with new
            cro_state.molecules[reactant_index].update_best(&product);

            // Replace individual
            cro_state.molecules[reactant_index].kinetic_energy =
                (total_reactant_energy - product_energy) * alpha;
            populations.current_mut()[reactant_index] = product;
        }
    }
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct DecompositionUpdate;

impl<P> Component<P> for DecompositionUpdate
where
    P: SingleObjectiveProblem,
{
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.require::<Self, CroState<P>>();
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) {
        let mut mut_state = state.get_states_mut();
        let populations = mut_state.populations_mut();
        let mut cro_state = mut_state.get_mut::<CroState<P>>();
        let rng = mut_state.random_mut();

        // Get reaction reactant and products p1, p2
        let [p1, p2] = TryInto::<[_; 2]>::try_into(populations.pop()).ok().unwrap();
        let reactant = populations.pop().into_iter().next().unwrap();

        let reactant_index = populations
            .current()
            .iter()
            .position(|i| i == &reactant)
            .unwrap();

        let total_reactant_energy =
            reactant.objective().value() + cro_state.molecules[reactant_index].kinetic_energy;
        let products_energy = p1.objective().value() + p2.objective().value();

        let decomposition_energy = if total_reactant_energy >= products_energy {
            total_reactant_energy - products_energy
        } else {
            let deltas: f64 = rng
                .sample_iter(rand::distributions::Uniform::new(0., 1.))
                .take(2)
                .product();
            let decomposition_energy =
                total_reactant_energy + deltas * cro_state.buffer - products_energy;

            // Not enough energy for reaction even with buffer, so abort
            if decomposition_energy < 0. {
                cro_state.molecules[reactant_index].num_hit += 1;
                return;
            }

            cro_state.buffer *= 1. - deltas;
            decomposition_energy
        };

        // Initialize molecule data and insert new individuals
        let d3 = rng.gen_range(0.0..=1.0);

        cro_state.molecules[reactant_index] = Molecule::new(decomposition_energy * d3, &p1);
        cro_state
            .molecules
            .push(Molecule::new(decomposition_energy * (1. - d3), &p2));

        let population = populations.current_mut();
        population[reactant_index] = p1;
        population.push(p2);
    }
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct IntermolecularIneffectiveCollisionUpdate;

impl<P> Component<P> for IntermolecularIneffectiveCollisionUpdate
where
    P: SingleObjectiveProblem,
{
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.require::<Self, CroState<P>>();
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) {
        let mut mut_state = state.get_states_mut();
        let populations = mut_state.populations_mut();
        let cro_state = mut_state.get_mut::<CroState<P>>();
        let rng = mut_state.random_mut();

        // Get reaction reactants r1, r2 and products p1, p2
        let [p1, p2] = TryInto::<[_; 2]>::try_into(populations.pop()).ok().unwrap();
        let [r1, r2] = TryInto::<[_; 2]>::try_into(populations.pop()).ok().unwrap();

        let r1_index = populations.current().iter().position(|i| i == &r1).unwrap();
        let r2_index = populations.current().iter().position(|i| i == &r2).unwrap();

        if r1_index == r2_index {
            panic!("Molecule can't collide with itself.");
        }

        cro_state.molecules[r1_index].num_hit += 1;
        cro_state.molecules[r2_index].num_hit += 1;

        let total_r1_energy = r1.objective().value() + cro_state.molecules[r1_index].kinetic_energy;
        let total_r2_energy = r2.objective().value() + cro_state.molecules[r2_index].kinetic_energy;
        let total_reactants_energy = total_r1_energy + total_r2_energy;

        let p1_energy = p1.objective().value();
        let p2_energy = p2.objective().value();
        let products_energy = p1_energy + p2_energy;

        let collision_energy = total_reactants_energy - products_energy;

        // Reaction is possible
        if collision_energy >= 0. {
            let d4 = rng.gen_range(0.0..=1.0);

            // Update kinetic energies
            cro_state.molecules[r1_index].kinetic_energy = collision_energy * d4;
            cro_state.molecules[r2_index].kinetic_energy = collision_energy * (1. - d4);

            // Update best with new
            cro_state.molecules[r1_index].update_best(&p1);
            cro_state.molecules[r2_index].update_best(&p2);

            // Replace individual
            let population = populations.current_mut();
            population[r1_index] = p1;
            population[r2_index] = p2;
        }
    }
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct SynthesisUpdate;

impl<P> Component<P> for SynthesisUpdate
where
    P: SingleObjectiveProblem,
{
    fn initialize(&self, _problem: &P, state: &mut State<P>) {
        state.require::<Self, CroState<P>>();
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) {
        let mut mut_state = state.get_states_mut();
        let populations = mut_state.populations_mut();
        let cro_state = mut_state.get_mut::<CroState<P>>();

        // Get reaction reactants r1, r2 and product
        let product = populations.pop().into_iter().next().unwrap();
        let [r1, r2] = TryInto::<[_; 2]>::try_into(populations.pop()).ok().unwrap();

        let r1_index = populations.current().iter().position(|i| i == &r1).unwrap();
        let r2_index = populations.current().iter().position(|i| i == &r2).unwrap();

        if r1_index == r2_index {
            panic!("Molecule can't collide with itself.");
        }

        let total_r1_energy = r1.objective().value() + cro_state.molecules[r1_index].kinetic_energy;
        let total_r2_energy = r2.objective().value() + cro_state.molecules[r2_index].kinetic_energy;
        let total_reactants_energy = total_r1_energy + total_r2_energy;

        let product_energy = product.objective().value();

        // Reaction is possible
        if total_reactants_energy >= product_energy {
            // Replace one molecule and remove other
            cro_state.molecules[r1_index] =
                Molecule::new(total_reactants_energy - product_energy, &product);
            cro_state.molecules.remove(r2_index);

            // Replace one individual and remove other
            let population = populations.current_mut();
            population[r1_index] = product;
            population.remove(r2_index);
        }
    }
}

impl<P: Problem> CroState<P>
where
    P: SingleObjectiveProblem,
{
    /// State initialization for CRO.
    ///
    /// Initializes the energy buffer and molecule data in [CroState].
    pub fn initializer(initial_kinetic_energy: f64, buffer: f64) -> Box<dyn Component<P>> {
        Box::new(CroStateInitialization {
            initial_kinetic_energy,
            buffer,
        })
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
    pub fn on_wall_ineffective_collision_update(
        kinetic_energy_loss_rate: f64,
    ) -> Box<dyn Component<P>> {
        Box::new(OnWallIneffectiveCollisionUpdate {
            kinetic_energy_loss_rate,
        })
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
    pub fn decomposition_update() -> Box<dyn Component<P>> {
        Box::new(DecompositionUpdate)
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
    pub fn intermolecular_ineffective_collision_update() -> Box<dyn Component<P>> {
        Box::new(IntermolecularIneffectiveCollisionUpdate)
    }

    /// Updates state after a Synthesis.
    ///
    /// Updates the energy buffer and molecule data in [CroState].
    ///
    /// It assumes the following [Population][crate::state::common::Population] structure:
    /// - One combined individual k
    /// - Two selected individuals i and j
    /// - Population
    ///
    /// Note that this component does **NOT** perform the operation, but only updates state afterwards.
    pub fn synthesis_update() -> Box<dyn Component<P>> {
        Box::new(SynthesisUpdate)
    }
}
