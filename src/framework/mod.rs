#![doc = embed_doc_image::embed_image!("module_system", "docs/MAHF-module-system.svg")]
#![doc = include_str!("../../docs/heuristic.md")]

pub mod components;
pub mod state;

mod configuration;
pub use configuration::{Configuration, ConfigurationBuilder};

mod fitness;
pub use fitness::{Fitness, IllegalFitness};

mod individual;
pub use individual::Individual;

mod random;
pub use random::{Random, RandomConfig};

use crate::problems::Problem;
use crate::tracking::Log;

pub fn run<P: Problem>(
    problem: &P,
    config: &Configuration<P>,
    rng: Option<Random>,
) -> state::State {
    let mut state = state::State::new_root();

    state.insert(Log::new());
    state.insert(rng.unwrap_or_default());
    state.insert(state::common::Population::new());

    config.initialize(problem, &mut state);
    config.execute(problem, &mut state);

    state
}
