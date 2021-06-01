use crate::{
    problem,
    problems::functions,
    random::Random,
};

#[cfg(test)]
mod bmf_tests {
    use super::*;
    use crate::problems::functions::BenchmarkFunction;
    use crate::problem::Problem;

    # [test]
    fn test_sphere() {
        let dimension = 10;
        let problem = BenchmarkFunction::sphere(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let fitness = problem.evaluate(&optimum_position);
        assert_eq!(fitness, optimum_value);

    }
}