#![allow(unused_variables, dead_code)]

use crate::problems::coco::{suits::Suite, Instance};
use std::ops::RangeInclusive;

mod util_2009;

pub enum Years {
    Y2009 = 2009,
    Y2010 = 2010,
    Y2012 = 2012,
    Y2013 = 2013,
    Y2015 = 2015,
    Y2016 = 2016,
    Y2017 = 2017,
    Y2018 = 2018,
}

pub fn new(year: Years) -> Suite {
    Suite::new(
        flatten_ranges(&[1..=24]),
        instances_by_year(year),
        vec![2, 3, 5, 10, 20, 40],
        generator,
    )
}

fn flatten_ranges(ranges: &[RangeInclusive<usize>]) -> Vec<usize> {
    let mut list = Vec::new();

    for range in ranges {
        for i in range.clone() {
            list.push(i);
        }
    }

    list
}

fn instances_by_year(year: Years) -> Vec<usize> {
    let ranges: &[_] = match year {
        Years::Y2009 => &[1..=5, 1..=5, 1..=5],
        Years::Y2010 => &[1..=15],
        Years::Y2012 => &[1..=5, 21..=30],
        Years::Y2013 => &[1..=5, 31..=40],
        Years::Y2015 => &[1..=5, 41..=50],
        Years::Y2016 => &[1..=5, 51..=60],
        Years::Y2017 => &[1..=5, 61..=70],
        Years::Y2018 => &[1..=5, 71..=80],
    };

    flatten_ranges(ranges)
}

fn generator(function: usize, instance: usize, dimension: usize) -> Instance {
    assert_eq!(instance, 1, "Toy suite only contains one instance");

    let rseed = function + 10000 * instance;
    let rseed_3 = 3 + 10000 * instance;
    let rseed_17 = 17 + 10000 * instance;

    let problem = match function {
        0 => panic!("Suite functions start at 1, 0 was requested"),
        1 => functions::sphere(function, instance, dimension, rseed),
        2 => functions::ellipsoid(function, instance, dimension, rseed),
        3 => functions::rastrigin(function, instance, dimension, rseed),
        4 => functions::bueche_rastrigin(function, instance, dimension, rseed_3),
        5 => functions::linear_slope(function, instance, dimension, rseed),
        6..=24 => todo!("These are not implemented yet"),
        _ => panic!(
            "BBOB suite only contains 24 functions ({} was requested)",
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

    pub fn ellipsoid(function: usize, dimension: usize, instance: usize, rseed: usize) -> Problem {
        let xopt = util_2009::compute_xopt(rseed, dimension);
        let fopt = util_2009::compute_fopt(function, instance);

        let problem = problems::ellipsoid();
        let problem = problems::oscillate_input(problem);
        let problem = problems::translate_input(xopt, problem);
        let problem = problems::translate_output(fopt, problem);

        problem
    }

    pub fn rastrigin(function: usize, dimension: usize, instance: usize, rseed: usize) -> Problem {
        let xopt = util_2009::compute_xopt(rseed, dimension);
        let fopt = util_2009::compute_fopt(function, instance);

        let problem = problems::rastrigin();
        let problem = problems::condition_input(10.0, problem);
        let problem = problems::asymmetric_input(0.2, problem);
        let problem = problems::oscillate_input(problem);
        let problem = problems::translate_input(xopt, problem);
        let problem = problems::translate_output(fopt, problem);

        problem
    }

    pub fn bueche_rastrigin(
        function: usize,
        dimension: usize,
        instance: usize,
        rseed: usize,
    ) -> Problem {
        let xopt = util_2009::compute_xopt(rseed, dimension);
        let fopt = util_2009::compute_fopt(function, instance);

        let problem = problems::rastrigin();
        let problem = problems::brs_input(problem);
        let problem = problems::oscillate_input(problem);
        let problem = problems::translate_input(xopt, problem);
        let problem = problems::translate_output(fopt, problem);

        problem
    }

    pub fn linear_slope(
        function: usize,
        dimension: usize,
        instance: usize,
        rseed: usize,
    ) -> Problem {
        let xopt = util_2009::compute_xopt(rseed, dimension);
        let fopt = util_2009::compute_fopt(function, instance);

        let problem = problems::linear_slope();
        let problem = problems::translate_input(xopt, problem);
        let problem = problems::translate_output(fopt, problem);

        problem
    }
}
