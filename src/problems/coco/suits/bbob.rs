#![allow(unused_variables, dead_code)]

use crate::problems::coco::{problems, suits::Suite, Instance};
use std::ops::RangeInclusive;

mod util_2009;

static YEARS: &[usize] = &[2009, 2010, 2012, 2013, 2015, 2016, 2017, 2018];

pub fn new() -> Suite {
    Suite::new(
        vec![1, 2, 3, 4, 5, 6],
        vec![1],
        vec![2, 3, 5, 10, 20, 40],
        generator,
    )
}

fn flaten_ranges(ranges: &[RangeInclusive<usize>]) -> Vec<usize> {
    let mut list = Vec::new();

    for range in ranges {
        for i in range.clone() {
            list.push(i);
        }
    }

    list
}

fn instances_by_year(year: usize) -> Option<Vec<usize>> {
    let ranges: Option<&[_]> = match year {
        2009 => Some(&[1..=5, 1..=5, 1..=5]),
        2010 => Some(&[1..=15]),
        2012 => Some(&[1..=5, 21..=30]),
        2013 => Some(&[1..=5, 31..=40]),
        2015 => Some(&[1..=5, 41..=50]),
        2016 => Some(&[1..=5, 51..=60]),
        2017 => Some(&[1..=5, 61..=70]),
        2018 => Some(&[1..=5, 71..=80]),
        _ => None,
    };

    ranges.map(flaten_ranges)
}

fn generator(function: usize, instance: usize, dimension: usize) -> Instance {
    assert_eq!(instance, 1, "Toy suite only contains one instance");

    let rseed = function + 10000 * instance;
    let rseed_3 = 3 + 10000 * instance;
    let rseed_17 = 17 + 10000 * instance;

    let problem = match function {
        1 => functions::sphere(function, instance, dimension, rseed),
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

mod functions {
    use super::util_2009;
    use crate::problems::coco::{problems, Problem};

    pub fn sphere(function: usize, dimension: usize, instance: usize, rseed: usize) -> Problem {
        let xopt = util_2009::compute_xopt(rseed, dimension);
        let fopt = util_2009::compute_fopt(function, instance);

        let problem = problems::sphere();
        let problem = problems::translate_input(xopt, problem);
        let problem = problems::translate_output(fopt, problem);

        problem
    }
}