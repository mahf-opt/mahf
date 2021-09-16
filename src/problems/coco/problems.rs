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

pub fn attractive_sector() -> Problem {
    Problem {
        name: "attractive_sector",
        function: ProblemFunction::Function(functions::attractive_sector),
        domain: DEFAULT_DOMAIN,
    }
}

pub fn rastrigin() -> Problem {
    Problem {
        name: "rastrigin",
        function: ProblemFunction::Function(functions::rastrigin),
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
