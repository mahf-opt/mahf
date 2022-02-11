use crate::{framework::components::*, problems::Problem};
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
    pub archiving: Option<Box<dyn Archiving<P>>>,

    /// Updates (custom) state after an iteration.
    #[serde(with = "erased_serde")]
    pub post_replacement: Option<Box<dyn Postprocess<P>>>,

    /// Decides when to terminate the process.
    #[serde(with = "erased_serde")]
    pub termination: Box<dyn Termination>,
}

impl<P: Problem> Configuration<P> {
    pub fn new(
        initialization: impl Initialization<P> + 'static,
        selection: impl Selection + 'static,
        generation: impl Generation<P> + 'static,
        replacement: impl Replacement + 'static,
        termination: impl Termination + 'static,
    ) -> Self {
        Configuration {
            initialization: Box::new(initialization),
            selection: Box::new(selection),
            generation: vec![Box::new(generation)],
            generation_scheduler: Box::new(crate::operators::schedulers::AllInOrder),
            replacement: Box::new(replacement),
            archiving: None,
            post_replacement: None,
            termination: Box::new(termination),
        }
    }

    pub fn new_extended(
        initialization: impl Initialization<P> + 'static,
        selection: impl Selection + 'static,
        generation: impl Generation<P> + 'static,
        replacement: impl Replacement + 'static,
        archiving: Option<impl Archiving<P> + 'static>,
        post_replacement: Option<impl Postprocess<P> + 'static>,
        termination: impl Termination + 'static,
    ) -> Self {
        let post_replacement = post_replacement.map::<Box<dyn Postprocess<P>>, _>(|c| Box::new(c));
        let archiving = archiving.map::<Box<dyn Archiving<P>>, _>(|c| Box::new(c));
        Configuration {
            initialization: Box::new(initialization),
            selection: Box::new(selection),
            generation: vec![Box::new(generation)],
            generation_scheduler: Box::new(crate::operators::schedulers::AllInOrder),
            replacement: Box::new(replacement),
            archiving,
            post_replacement,
            termination: Box::new(termination),
        }
    }

    pub fn with_initialization(mut self, initialization: impl Initialization<P> + 'static) -> Self {
        self.initialization = Box::new(initialization);
        self
    }

    pub fn with_selection(mut self, selection: impl Selection + 'static) -> Self {
        self.selection = Box::new(selection);
        self
    }

    pub fn with_generation(mut self, generation: impl Generation<P> + 'static) -> Self {
        self.generation = vec![Box::new(generation)];
        self
    }

    pub fn with_replacement(mut self, replacement: impl Replacement + 'static) -> Self {
        self.replacement = Box::new(replacement);
        self
    }

    pub fn with_termination(mut self, termination: impl Termination + 'static) -> Self {
        self.termination = Box::new(termination);
        self
    }

    pub fn add_generator(mut self, generation: impl Generation<P> + 'static) -> Self {
        self.generation.push(Box::new(generation));
        self
    }
}
