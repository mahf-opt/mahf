use crate::{
    framework::{components::*, Individual},
    problems::{LimitedVectorProblem, Problem, SingleObjectiveProblem},
    state::{CustomState, State},
};
use rand::Rng;

/// State required for CRO.
///
/// For preserving energy buffer level and molecule data.
pub struct CroState<P: Problem> {
    pub energy_buffer: f64,
    pub kinetic_energies: Vec<f64>,
    pub num_hits: Vec<usize>,
    pub bests: Vec<Individual<P>>,
    pub min_hits: Vec<usize>,
}
impl<P: Problem> CustomState for CroState<P> {}

impl<P: Problem> CroState<P>
where
    P: SingleObjectiveProblem,
{
    /// State initialization for CRO.
    ///
    /// Initializes the energy buffer and molecule data in [CroState].
    pub fn initializer(initial_kinetic_energy: f64, beta: f64, buffer: f64) -> Box<dyn Component<P>> {
        #[derive(Debug, serde::Serialize, Clone)]
        pub struct CroStateInitialization {
            initial_kinetic_energy: f64,
            beta: f64,
            buffer: f64,
        }

        impl<P> Component<P> for CroStateInitialization
        where
            P: SingleObjectiveProblem,
        {
            fn initialize(&self, _problem: &P, state: &mut State) {
                // Initialize with empty state to satisfy `state.require()` statements
                state.insert(CroState {
                    energy_buffer: 0.,
                    kinetic_energies: vec![],
                    num_hits: vec![],
                    bests: vec![],
                    min_hits: vec![]
                })
            }

            fn execute(&self, _problem: &P, state: &mut State) {
                let population_size = state.population_stack::<P>().current().len();

                state.insert(CroState {
                    energy_buffer: self.buffer,
                    kinetic_energies: vec![self.initial_kinetic_energy; population_size],
                    num_hits: vec![0; population_size],
                    bests: state.population_stack::<P>().current().to_vec(),
                    min_hits: vec![0; population_size],
                });

            }
        }

        Box::new(CroStateInitialization { initial_kinetic_energy, beta, buffer })
    }
}
