use crate::{
    framework::{
        common_state,
        components::{self, Block, Component, Condition, Logger, Loop, Scope},
    },
    operators::*,
    problems::Problem,
};
use serde::Serialize;

/// A set of components, representing a heuristic.
///
/// While `generation_scheduler`, `archiving` and `post_replacement` can
/// often be ommitet, the other components should always be specified.
///
/// A simple GA could look like this:
/// ```
///# use mahf::operators::*;
///# use mahf::framework::legacy::Configuration;
///# use mahf::problems::bmf::BenchmarkFunction;
///# let config: Configuration<BenchmarkFunction> =
/// Configuration {
///     initialization: initialization::RandomSpread::new_init(25),
///     selection: selection::RouletteWheel::new(25),
///     generation: vec![
///         generation::recombination::UniformCrossover::new(0.8),
///         generation::mutation::FixedDeviationDelta::new(0.2),
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
    pub initialization: Box<dyn Component<P>>,

    /// Selects individuals from the population.
    #[serde(with = "erased_serde")]
    pub selection: Box<dyn Component<P>>,

    /// Generates new solutions based on selection.
    #[serde(with = "erased_serde")]
    pub generation: Vec<Box<dyn Component<P>>>,

    /// Decides which generations should be called.
    #[deprecated(note = "Will allways use AllInOrder. The new framework should be used instead.")]
    #[serde(with = "erased_serde")]
    pub generation_scheduler: Box<dyn Component<P>>,

    /// Replaces old solutions with newly generated.
    #[serde(with = "erased_serde")]
    pub replacement: Box<dyn Component<P>>,

    /// Exchanges solutions with population after replacement.
    #[serde(with = "erased_serde")]
    pub archiving: Box<dyn Component<P>>,

    /// Updates (custom) state after an iteration.
    #[serde(with = "erased_serde")]
    pub post_replacement: Box<dyn Component<P>>,

    /// Decides when to terminate the process.
    #[serde(with = "erased_serde")]
    pub termination: Box<dyn Condition<P>>,

    /// Logs the heuristic state.
    #[serde(with = "erased_serde")]
    pub logger: Box<dyn Component<P>>,
}

impl<P: Problem> Default for Configuration<P> {
    fn default() -> Self {
        #[allow(deprecated)]
        Self {
            initialization: initialization::Empty::new(),
            selection: selection::None::new(),
            generation: vec![Noop::new()],
            generation_scheduler: schedulers::AllInOrder::new(),
            replacement: replacement::Noop::new(),
            archiving: archive::None::new(),
            post_replacement: postprocess::None::new(),
            termination: termination::Undefined::new(),
            logger: Logger::builder().build(),
        }
    }
}

impl<P: Problem> From<Configuration<P>> for components::Configuration<P> {
    fn from(cfg: Configuration<P>) -> Self {
        Scope::new_with(
            common_state::default,
            vec![
                cfg.initialization,
                components::SimpleEvaluator::new(),
                Loop::new(
                    cfg.termination,
                    vec![
                        cfg.selection,
                        Block::new(cfg.generation),
                        components::SimpleEvaluator::new(),
                        cfg.replacement,
                        cfg.archiving,
                        cfg.post_replacement,
                        cfg.logger,
                    ],
                ),
            ],
        )
    }
}
