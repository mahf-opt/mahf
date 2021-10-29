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

pub fn ellipsoid() -> Problem {
    Problem {
        name: "ellipsoid",
        function: ProblemFunction::Function(functions::ellipsoid),
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

pub fn bueche_rastrigin() -> Problem {
    Problem {
        name: "bueche_rastrigin",
        function: ProblemFunction::Function(functions::bueche_rastrigin),
        domain: DEFAULT_DOMAIN,
    }
}

pub fn linear_slope() -> Problem {
    Problem {
        name: "linear_slope",
        function: ProblemFunction::Function(functions::linear_slope),
        domain: DEFAULT_DOMAIN,
    }
}

pub fn rosenbrock() -> Problem {
    Problem {
        name: "rosenbrock",
        function: ProblemFunction::Function(functions::rosenbrock),
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

//
// Transformations
//

pub fn permutation(mapping: Vec<usize>, inner: Problem) -> Problem {
    Problem::extend(
        "permutaion",
        inner,
        transformations::input::Permutation { mapping },
    )
}

pub fn translate_input(translation: Vec<f64>, inner: Problem) -> Problem {
    Problem::extend(
        "translate_input",
        inner,
        transformations::input::Translate { translation },
    )
}

pub fn oscillate_input(inner: Problem) -> Problem {
    Problem::extend("oscillate_input", inner, transformations::input::Oscillate)
}

pub fn condition_input(alpha: f64, inner: Problem) -> Problem {
    Problem::extend(
        "condition_input",
        inner,
        transformations::input::Condition { alpha },
    )
}

pub fn asymmetric_input(beta: f64, inner: Problem) -> Problem {
    Problem::extend(
        "asymmetric_input",
        inner,
        transformations::input::Asymmetric { beta },
    )
}

pub fn brs_input(inner: Problem) -> Problem {
    Problem::extend("brs_input", inner, transformations::input::Brs)
}

pub fn translate_output(translation: f64, inner: Problem) -> Problem {
    Problem::extend(
        "translate_output",
        inner,
        transformations::output::Translate { translation },
    )
}
