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

    match function {
        1 => Instance::new(problems::sphere(), dimension),
        2 => Instance::new(problems::ellipsoid(), dimension),
        3 => Instance::new(problems::rastrigin(), dimension),
        4 => Instance::new(problems::bueche_rastrigin(), dimension),
        5 => Instance::new(problems::linear_slope(), dimension),
        6 => Instance::new(problems::rosenbrock(), dimension),
        _ => panic!("Toy suite only contains 6 functions"),
    }
}
