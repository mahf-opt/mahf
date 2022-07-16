//! Tracking and logging.
//!
//! # Concept
//! Logging in MAHF operates in a modular fashion,
//! similar to the rest of the framework.
//! The heart of MAHFs logging is the [Logger] component which can be
//! added to a heuristics main loop to track its state during
//! the optimization process.
//!
//! A Logger consists of [LogSet]s which in turn consist of
//! [Trigger](trigger::Trigger) and [LogFn](functions::LogFn).
//! See their respective documentation pages for more information
//! on how to use them.
//!
//! When defining a heuristic template like [crate::heuristics::iwo],
//! the [Logger] should be a [Component](crate::framework::components::Component) parameter.
//!
//! When calling [run](crate::framework::run), a [Log] will be
//! added to the state, which the [Logger] will use to store the entries.
//! The [Log] can be retrieved afterwards using [State::log](crate::framework::State::log).

/// Utils to write [Log]s to the disc.
pub mod files;

/// Utils to create log functions.
pub mod functions;

/// Collection of log triggers.
pub mod trigger;

/// Components of a [Log].
pub mod log;
pub use log::Log;

mod logger;
pub use logger::Logger;

mod set;
pub use set::LogSet;
