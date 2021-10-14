use crate::problems::coco::{problems, suits::Suite, Instance};

pub fn new() -> Suite {
    Suite::new(
        vec![1, 2, 3, 4, 5, 6],
        vec![1],
        vec![2, 3, 5, 10, 20, 40],
        generator,
    )
}

fn generator(function: usize, instance: usize, dimension: usize) -> Instance {
    assert_eq!(instance, 1, "Toy suite only contains one instance");

    let problem = match function {
        1 => problems::sphere(),
        2 => problems::ellipsoid(),
        3 => problems::rastrigin(),
        4 => problems::bueche_rastrigin(),
        5 => problems::linear_slope(),
        6 => problems::rosenbrock(),
        _ => panic!(
            "Toy suite only contains 6 functions ({} was requested)",
            function
        ),
    };

    Instance {
        problem,
        function,
        instance,
        dimension,
    }
}
