use crate::{heuristic::components::*, problem::Problem};

/// A full set of components, effectively representing a heuristic.
pub struct Configuration<P> {
    pub initialization: Box<dyn Initialization<P>>,
    pub selection: Box<dyn Selection>,
    pub generation: Box<dyn Generation<P>>,
    pub replacement: Box<dyn Replacement>,
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
            generation: Box::new(generation),
            replacement: Box::new(replacement),
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
        self.generation = Box::new(generation);
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
}
