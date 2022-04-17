#![doc = embed_doc_image::embed_image!("module_system", "docs/MAHF-module-system.svg")]
#![doc = include_str!("../../docs/heuristic.md")]

pub mod components;
pub mod legacy;

mod fitness;
pub use fitness::{Fitness, IllegalFitness};

mod state;
pub use state::common as common_state;
pub use state::{CustomState, CustomStateMap};

mod individual;
pub use individual::Individual;

use crate::tracking::Log;
use crate::{
    framework::{components::Configuration, state::StateTree},
    problems::Problem,
    random::Random,
};

pub fn run<P: Problem>(
    problem: &P,
    config: &Configuration<P>,
    log: Option<Log>,
    rng: Option<Random>,
) -> StateTree {
    let mut state = StateTree::new_root();

    state.insert(common_state::Rng(rng.unwrap_or_default()));

    if let Some(log) = log {
        state.insert(log);
    }

    config.initialize(problem, &mut state);
    config.execute(problem, &mut state);

    state
}
