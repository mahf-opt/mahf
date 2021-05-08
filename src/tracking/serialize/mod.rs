#![allow(clippy::new_without_default)]

use crate::heuristic::Configuration;
use std::collections::HashMap;

pub mod error;

mod component;
mod config;
mod noop;
mod value;

//pub use names::collect_names;
//pub use values::collect_values;
pub use config::serialize_config;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SerializedComponent {
    pub name: &'static str,
    pub fields: HashMap<&'static str, String>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SerializedConfiguration {
    pub initialization: SerializedComponent,
    pub selection: SerializedComponent,
    pub generation: SerializedComponent,
    pub replacement: SerializedComponent,
    pub termination: SerializedComponent,
}

pub fn validate_serializability<P>(config: &Configuration<P>) -> bool {
    serialize_config(config).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{serialize_config, validate_serializability};
    use crate::heuristics::es;
    use crate::problems::functions::BenchmarkFunction;
    use serde::Serialize;

    #[test]
    pub fn serializing_es_config() {
        let instance = es::mu_plus_lambda::<BenchmarkFunction>(0, 0, 0.0, 42);

        let config = serialize_config(&instance);
        assert!(config.is_ok());

        let config = config.unwrap();
        let term = config.termination;
        assert_eq!(term.name, "FixedIterations");
        assert_eq!(term.fields.get("max_iterations"), Some(&String::from("42")));
    }

    #[test]
    pub fn validate_es_config() {
        let instance = es::mu_plus_lambda::<BenchmarkFunction>(0, 0, 0.0, 42);
        assert!(validate_serializability(&instance));
    }
}
