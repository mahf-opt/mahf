//! Utilities for logging.
//!
//! Logging in MAHF operates in a modular fashion, similar to the rest of the framework.
//! The heart of MAHFs logging is the [`Logger`] component, which can be added to a (meta)heuristic
//! main loop to track its state during the optimization process.
//!
//! A [`Logger`] is configured by the [`LogConfig`], which in turn consist of [`Condition`]s
//! and [`EntryExtractor`]s.
//! See their documentation for more information on how to use them.
//!
//! When calling [`Configuration::optimize`] or [`Configuration::optimize_with`], a [`Log`] will be
//! added to the state, which the [`Logger`] will use to store the entries.
//! The [`Log`] can be retrieved afterwards using e.g. the [`State::log`] method.
//!
//! [`Condition`]: crate::Condition
//! [`EntryExtractor`]: extractor::EntryExtractor
//! [`Configuration::optimize`]: crate::Configuration::optimize
//! [`Configuration::optimize_with`]: crate::Configuration::optimize_with
//! [`State::log`]: crate::State::log

pub mod config;
pub mod extractor;
pub mod log;
pub mod logger;

#[doc(inline)]
pub use config::LogConfig;
#[doc(inline)]
pub use log::Log;
#[doc(inline)]
pub use logger::Logger;
