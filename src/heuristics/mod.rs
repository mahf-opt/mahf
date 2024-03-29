//! (Meta)Heuristic templates.
//!
//! The templates in this module allow customization of certain operators
//! similar to other frameworks (e.g. a custom mutation operator) and are meant to
//! provide a starting point for more specialized configurations.
//!
//! A common workflow is to copy the definition of the heuristic closest to the desired
//! structure and adapt it to one's needs.

pub mod aco;
pub mod bh;
pub mod cro;
pub mod de;
pub mod es;
pub mod fa;
pub mod ga;
pub mod ils;
pub mod iwo;
pub mod ls;
pub mod pso;
pub mod rs;
pub mod rw;
pub mod sa;
