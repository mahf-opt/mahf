#![allow(clippy::new_without_default)]

pub mod error;

mod names;
mod noop;
mod value;
mod values;

pub use names::collect_names;
pub use values::collect_values;

pub fn validate_serializability<T>(structure: &T) -> bool
where
    T: serde::Serialize,
{
    collect_values(structure).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{collect_names, collect_values, validate_serializability};
    use serde::Serialize;

    #[derive(Default, Serialize)]
    struct Simple {
        a: usize,
        b: &'static str,
        c: f64,
    }

    #[derive(Default, Serialize)]
    struct TooComplex {
        tuple: (i32, i32),
    }

    #[test]
    pub fn simple_names_work() {
        let instance = Simple::default();

        let names = collect_names(&instance);
        assert!(names.is_ok());

        let names = names.unwrap();
        assert_eq!(names.name, "Simple");
        assert_eq!(names.fields, &["a", "b", "c"]);
    }

    #[test]
    pub fn simple_values_work() {
        let instance = Simple {
            a: 42,
            b: "42x",
            c: 42.42,
        };

        let values = collect_values(&instance);
        assert!(values.is_ok());

        let values = values.unwrap();
        assert_eq!(values, &["42", "42x", "42.42"]);
    }

    #[test]
    pub fn too_complex_names_work() {
        let instance = TooComplex::default();

        let names = collect_names(&instance);
        assert!(names.is_ok());
    }

    #[test]
    pub fn too_complex_values_fails() {
        let instance = TooComplex::default();

        let values = collect_values(&instance);
        assert!(values.is_err());
    }

    #[test]
    pub fn validate_serializability_works() {
        assert_eq!(validate_serializability(&Simple::default()), true);
        assert_eq!(validate_serializability(&TooComplex::default()), false);
    }
}
