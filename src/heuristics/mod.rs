//! Collection of common heuristics

pub mod aco;
pub use aco::aco;

pub mod es;
pub use es::mu_plus_lambda as mu_plus_lambda_es;

pub mod iwo;
pub use iwo::iwo;
