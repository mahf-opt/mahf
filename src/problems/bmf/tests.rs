#[cfg(test)]
mod bmf_tests {
    use crate::problems::functions::BenchmarkFunction;
    use crate::problem::{Problem, LimitedVectorProblem};
    use crate::random::Random;
    use rand::Rng;
    use float_eq::*;
    use std::f64::consts::PI;

    #[test]
    fn test_sphere() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::sphere(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_rastrigin() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::rastrigin(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_ackley() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::ackley(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value: f64 = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness.abs(), r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_ackley_n4() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension_known = 2;
        let dimension = 10;
        let problem1 = BenchmarkFunction::ackley_n4(dimension_known);
        let problem2 = BenchmarkFunction::ackley_n4(dimension);
        let optimum_position = vec![-1.51 / 35.0, -0.755 / 35.0];
        // Optimum only known for 2 dimensions!
        let optimum_value = -4.590101633799122;
        let optimum_fitness = problem1.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem2.dimension())
            .map(|dimension| rng.gen_range(problem2.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem2.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_alpine_n1() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::alpine_n1(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_alpine_n2() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::alpine_n2(dimension);
        let optimum_position = vec![7.917 / 10.0; dimension];
        let optimum_value = -(2.808_f64).powi(dimension as i32);
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_brown() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::brown(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_exponential() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::exponential(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = -1.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_griewank() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::griewank(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_happy_cat() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::happy_cat(dimension);
        let optimum_position = vec![-1.0 / 2.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_periodic() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::periodic(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.9;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_powell_sum() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::powell_sum(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_qing() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 5;
        let problem = BenchmarkFunction::qing(dimension);
        let optimum_position = vec![(1.0_f64).sqrt() / 500.0, (2.0_f64).sqrt() / 500.0, (3.0_f64).sqrt() / 500.0, (4.0_f64).sqrt() / 500.0, (5.0_f64).sqrt() / 500.0];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_ridge() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::ridge(dimension);
        let mut optimum_position = vec![-5.0 / 5.0];
        let mut opt_rest = vec![0.0; dimension - 1];
        optimum_position.append(&mut opt_rest);
        let optimum_value = -5.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_rosenbrock() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::rosenbrock(dimension);
        let optimum_position = vec![1.0 / 10.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_salomon() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::salomon(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_schwefel_220() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_220(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_schwefel_221() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_221(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_schwefel_222() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_222(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_schwefel_223() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_223(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_schwefel() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel(dimension);
        let optimum_position = vec![420.9687 / 500.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_shubert_n3() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::shubert_n3(dimension);
        // several optima, unknown
        let optimum_position = vec![0.0; dimension];
        let optimum_value = -29.6733337;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        //assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_shubert_n4() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::shubert_n4(dimension);
        // several optima, unknown
        let optimum_position = vec![0.0; dimension];
        let optimum_value = -25.740858;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        //assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_shubert() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::shubert(dimension);
        // several optima, unknown
        let optimum_position = vec![0.0; dimension];
        let optimum_value = -186.7309;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        //assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_styblinski_tank() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::styblinksi_tank(dimension);
        let optimum_position = vec![-2.903534 / 5.0; dimension];
        let optimum_value = -39.16599 * dimension as f64;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_sum_sqares() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::sum_squares(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_yang_n2() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::yang_n2(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_yang_n3() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::yang_n3(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = -1.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_yang_n4() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::yang_n4(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = -1.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_zakharov() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::zakharov(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    // from here on: non-n-dimensional functions
    #[test]
    fn test_ackley_n2() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::ackley_n2(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = -200.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_ackley_n3() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::ackley_n3(dimension);
        let optimum_position = vec![0.0, - 0.4 / 32.0];
        let optimum_value = -219.1418;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_adjiman() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::adjiman(dimension);
        let optimum_position = vec![2.0 / 2.0, 0.10578];
        let optimum_value = -2.02181;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_bartels_conn() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::bartels_conn(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 1.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_beale() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::beale(dimension);
        let optimum_position = vec![3.0 / 4.5, 0.5 / 4.5];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_bird() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::bird(dimension);
        let optimum_position1 = vec![4.70104 / (2.0 * PI), 3.15294 / (2.0 * PI)];
        let optimum_position2 = vec![-1.58214 / (2.0 * PI), -3.13024 / (2.0 * PI)];
        let optimum_value = -106.764537;
        let optimum_fitness1 = problem.evaluate(&optimum_position1);
        let optimum_fitness2 = problem.evaluate(&optimum_position2);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness1, r2nd <= f64::EPSILON);
        assert_float_eq!(optimum_value, optimum_fitness2, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness1);
    }

    #[test]
    fn test_bohachevsky_n1() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::bohachevsky_n1(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_bohachevsky_n2() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::bohachevsky_n2(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_booth() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::booth(dimension);
        let optimum_position = vec![1.0 / 10.0, 3.0 / 10.0];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_brent() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::brent(dimension);
        let optimum_position = vec![-10.0 / 20.0; dimension];
        let optimum_value = (-200.0_f64).exp();
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_bukin_n6() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::bukin_n6(dimension);
        let optimum_position = vec![-10.0 / 15.0, 1.0 / 3.0];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_cross_in_tray() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::cross_in_tray(dimension);
        let optimum_position = vec![1.349406685353340 / 10.0, 1.349406608602084 / 10.0];
        let optimum_value = -2.06261218;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_deckkers_aarts() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::deckkers_aarts(dimension);
        let optimum_position = vec![0.0, 15.0 / 20.0];
        let optimum_value = -24771.09375;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_drop_wave() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::drop_wave(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = -1.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_easom() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::easom(dimension);
        let optimum_position = vec![PI / 100.0; dimension];
        let optimum_value = -1.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_egg_crate() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::egg_crate(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_goldstein_price() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::goldstein_price(dimension);
        let optimum_position = vec![0.0, -1.0 / 2.0];
        let optimum_value = 3.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_gramacy_lee() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 1;
        let problem = BenchmarkFunction::gramacy_lee(dimension);
        let optimum_position = vec![0.548563444114526 / 2.5];
        let optimum_value = -0.869011134989500;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_himmelblau() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::himmelblau(dimension);
        let optimum_position = vec![3.0 / 6.0, 2.0 / 6.0];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_holder_table() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::holder_table(dimension);
        let optimum_position = vec![8.05502 / 10.0, 9.66459 / 10.0];
        let optimum_value = -19.2085;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_keane() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::keane(dimension);
        let optimum_position = vec![1.393249070031784 / 10.0, 0.0];
        let optimum_value = - 0.673667521146855;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_leon() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::leon(dimension);
        let optimum_position = vec![1.0 / 10.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_levi_n13() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::levi_n13(dimension);
        let optimum_position = vec![1.0 / 10.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_matyas() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::matyas(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_mccormick() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::mccormick(dimension);
        let optimum_position = vec![-0.547 / 4.0,-1.547 / 4.0];
        let optimum_value = -1.9133;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_schaffer_n1() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::schaffer_n1(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_schaffer_n2() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::schaffer_n2(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_schaffer_n3() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::schaffer_n3(dimension);
        let optimum_position = vec![0.0, 1.253115 / 100.0];
        let optimum_value = 0.00156685;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_schaffer_n4() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::schaffer_n4(dimension);
        let optimum_position = vec![0.0, 1.253115 / 100.0];
        let optimum_value = 0.292579;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_three_hump_camel() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::three_hump_camel(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    #[test]
    fn test_wolfe() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 3;
        let problem = BenchmarkFunction::wolfe(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        //assert_eq!(optimum_fitness, optimum_value);
        assert_float_eq!(optimum_value, optimum_fitness, r2nd <= f64::EPSILON);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }
}
/*
# [test]
fn test_yang_n1() {
    // test for known optimum
    let rng = Random::default();
    let dimension = 10;
    let problem = BenchmarkFunction::yang_n1(dimension, Some(rng));
    let optimum_position = vec![0.0; dimension];
    let optimum_value = 0.0;
    let optimum_fitness = problem.evaluate(&optimum_position);
    assert_eq!(optimum_fitness, optimum_value);

    // test for random values
    let rng2 = &mut Random::default();
    let random_position = (0..problem.dimension())
        .map(|dimension| rng2.gen_range(problem.range(dimension)))
        .collect::<Vec<f64>>();
    let random_fitness = problem.evaluate(&random_position);
    assert!(random_fitness >= optimum_fitness);
}

   # [test]
    fn test_quartic() {
        // test for known optimum
        let rng = Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::quartic(dimension, Some(rng));
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0 + problem.random_number();
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let rng2 = &mut Random::default();
        let random_position = (0..problem.dimension())
            .map(|dimension| rng2.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

 */