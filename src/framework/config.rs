use crate::{framework::components::*, operators::*, problems::Problem};
use serde::Serialize;

/// A set of components, representing a heuristic.
///
/// See [framework](crate::framework) documentation.
#[derive(Serialize)]
pub struct Configuration<P: 'static> {
    /// Initializes the population.
    #[serde(with = "erased_serde")]
    pub initialization: Box<dyn Initialization<P>>,

    /// Selects individuals from the population.
    #[serde(with = "erased_serde")]
    pub selection: Box<dyn Selection>,

    /// Generates new solutions based on selection.
    #[serde(with = "erased_serde")]
    pub generation: Vec<Box<dyn Generation<P>>>,

    /// Decides which generations should be called.
    #[serde(with = "erased_serde")]
    pub generation_scheduler: Box<dyn Scheduler>,

    /// Replaces old solutions with newly generated.
    #[serde(with = "erased_serde")]
    pub replacement: Box<dyn Replacement>,

    /// Exchanges solutions with population after replacement.
    #[serde(with = "erased_serde")]
    pub archiving: Box<dyn Archiving<P>>,

    /// Updates (custom) state after an iteration.
    #[serde(with = "erased_serde")]
    pub post_replacement: Box<dyn Postprocess<P>>,

    /// Decides when to terminate the process.
    #[serde(with = "erased_serde")]
    pub termination: Box<dyn Termination>,
}

impl<P: Problem> Default for Configuration<P> {
    fn default() -> Self {
        Self {
            initialization: Box::new(initialization::Noop),
            selection: Box::new(selection::None),
            generation: vec![Box::new(generation::Noop)],
            generation_scheduler: Box::new(schedulers::AllInOrder),
            replacement: Box::new(replacement::Noop),
            archiving: Box::new(archive::None),
            post_replacement: Box::new(postprocess::None),
            termination: Box::new(termination::FixedIterations { max_iterations: 0 }),
        }
    }
}
