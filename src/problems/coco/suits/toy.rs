use crate::problems::coco::{functions, suits::Suite, CocoInstance, FunctionObject, Problem};

pub fn new() -> Suite {
    Suite::new(
        vec![1, 2, 3, 4, 5, 6],
        vec![1],
        vec![2, 3, 5, 10, 20, 40],
        generator,
    )
}

fn toy_problem(function: FunctionObject) -> Problem {
    Problem {
        input_transformations: vec![],
        function,
        output_transformations: vec![],
        domain: functions::DEFAULT_DOMAIN,
    }
}

fn generator(function: usize, instance: usize, dimension: usize) -> CocoInstance {
    assert_eq!(instance, 1, "Toy suite only contains one instance");

    let problem = match function {
        1 => toy_problem(functions::Sphere.into()),
        2 => toy_problem(functions::Ellipsoid.into()),
        3 => toy_problem(functions::Rastrigin.into()),
        4 => toy_problem(functions::BuecheRastrigin.into()),
        5 => toy_problem(functions::LinearSlope.into()),
        6 => toy_problem(functions::Rosenbrock.into()),
        _ => panic!(
            "Toy suite only contains 6 functions ({} was requested)",
            function
        ),
    };

    CocoInstance {
        suite: "Toy",
        problem,
        function,
        instance,
        dimension,
    }
}
