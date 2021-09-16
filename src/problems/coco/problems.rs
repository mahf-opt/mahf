use crate::problems::coco::{functions, transformations, Problem, ProblemFunction};

//
// Functions
//

pub fn sphere() -> Problem {
    Problem {
        name: "sphere",
        function: ProblemFunction::Function(functions::sphere),
    }
}

//
// Transformations
//

pub fn permutation(mapping: Vec<usize>, inner: Problem) -> Problem {
    Problem {
        name: "permutation",
        function: ProblemFunction::new_transformation(
            transformations::Permutation { mapping },
            inner,
        ),
    }
}
