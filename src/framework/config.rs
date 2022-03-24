use crate::{framework::components::*, operators::*, problems::Problem};
use serde::Serialize;

/// A set of components, representing a heuristic.
///
/// While `generation_scheduler`, `archiving` and `post_replacement` can
/// often be ommitet, the other components should always be specified.
///
/// A simple GA could look like this:
/// ```
///# use mahf::operators::*;
///# use mahf::framework::Configuration;
///# use mahf::problems::bmf::BenchmarkFunction;
///# let config: Configuration<BenchmarkFunction> =
/// Configuration {
///     initialization: initialization::RandomSpread::new(25),
///     selection: selection::RouletteWheel::new(25),
///     generation: vec![
///         generation::UniformCrossover::new(0.8),
///         generation::FixedDeviationDelta::new(0.2),
///     ],
///     replacement: replacement::Generational::new(25),
///     termination: termination::FixedIterations::new(500),
///     ..Default::default()
/// }
///# ;
/// ```
///
/// See [framework](crate::framework) documentation.
#[derive(Serialize)]
pub struct Configuration<P: Problem + 'static> {
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
    pub termination: Box<dyn Termination<P>>,
}

impl<P: Problem> Default for Configuration<P> {
    fn default() -> Self {
        Self {
            initialization: initialization::Noop::new(),
            selection: selection::None::new(),
            generation: vec![generation::Noop::new()],
            generation_scheduler: schedulers::AllInOrder::new(),
            replacement: replacement::Noop::new(),
            archiving: archive::None::new(),
            post_replacement: postprocess::None::new(),
            termination: termination::Undefined::new(),
        }
    }
}
