use crate::{heuristic::components::*, problem::Problem};
use serde::Serialize;

/// A full set of components, effectively representing a heuristic.
#[derive(Serialize)]
pub struct Configuration<P: 'static> {
    #[serde(with = "erased_serde")]
    pub initialization: Box<dyn Initialization<P>>,

    #[serde(with = "erased_serde")]
    pub post_initialization: Option<Box<dyn Postprocess<P>>>,

    #[serde(with = "erased_serde")]
    pub selection: Box<dyn Selection>,

    #[serde(with = "erased_serde")]
    pub generation: Box<dyn Generation<P>>,

    #[serde(with = "erased_serde")]
    pub replacement: Box<dyn Replacement>,

    #[serde(with = "erased_serde")]
    pub post_replacement: Option<Box<dyn Postprocess<P>>>,

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
            post_initialization: None,
            selection: Box::new(selection),
            generation: Box::new(generation),
            replacement: Box::new(replacement),
            post_replacement: None,
            termination: Box::new(termination),
        }
    }

    pub fn new_extended(
        initialization: impl Initialization<P> + 'static,
        post_initialization: Option<impl Postprocess<P> + 'static>,
        selection: impl Selection + 'static,
        generation: impl Generation<P> + 'static,
        replacement: impl Replacement + 'static,
        post_replacement: Option<impl Postprocess<P> + 'static>,
        termination: impl Termination + 'static,
    ) -> Self {
        let post_initialization =
            post_initialization.map::<Box<dyn Postprocess<P>>, _>(|c| Box::new(c));
        let post_replacement = post_replacement.map::<Box<dyn Postprocess<P>>, _>(|c| Box::new(c));
        Configuration {
            initialization: Box::new(initialization),
            post_initialization,
            selection: Box::new(selection),
            generation: Box::new(generation),
            replacement: Box::new(replacement),
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
