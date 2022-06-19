//! Tracking and logging.

pub mod trigger;

mod logger;
pub use logger::Logger;

pub mod log;
pub use log::Log;

pub mod logfn;
pub use logfn::LoggerFunction;
