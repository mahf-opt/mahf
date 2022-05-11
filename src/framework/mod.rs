#![doc = embed_doc_image::embed_image!("module_system", "docs/MAHF-module-system.svg")]
#![doc = include_str!("../../docs/heuristic.md")]

pub mod legacy;

pub mod components;
pub use components::Configuration;

pub mod specializations;

mod builder;
pub use builder::ConfigurationBuilder;

mod fitness;
pub use fitness::{Fitness, IllegalFitness};

mod state;
pub use state::common as common_state;
pub use state::{CustomState, State};

mod individual;
pub use individual::Individual;

use crate::{problems::Problem, random::Random, tracking::Log};

pub fn run<P: Problem>(
    problem: &P,
    config: &Configuration<P>,
    log: Option<Log>,
    rng: Option<Random>,
) -> State {
    let mut state = State::new_root();

    state.insert(rng.unwrap_or_default());

    if let Some(log) = log {
        state.insert(log);
    }

    config.initialize(problem, &mut state);
    config.execute(problem, &mut state);

    state
}
