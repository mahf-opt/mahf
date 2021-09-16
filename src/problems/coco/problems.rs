use std::ops::Range;

use crate::problems::coco::{functions, transformations, Problem, ProblemFunction};

pub const DEFAULT_DOMAIN: Range<f64> = -5.0..5.0;

//
// Functions
//

pub fn sphere() -> Problem {
    Problem {
        name: "sphere",
        function: ProblemFunction::Function(functions::sphere),
        domain: DEFAULT_DOMAIN,
    }
}

//
// Transformations
//

pub fn permutation(mapping: Vec<usize>, inner: Problem) -> Problem {
    Problem::extend(
        "permutaion",
        inner,
        transformations::Permutation { mapping },
    )
}
