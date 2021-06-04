#[cfg(test)]
mod bmf_tests {
    use crate::problems::functions::BenchmarkFunction;
    use crate::problem::{Problem, LimitedVectorProblem};
    use crate::random::Random;
    use rand::Rng;

    # [test]
    fn test_sphere() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::sphere(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_rastrigin() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::rastrigin(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_ackley() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::ackley(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_ackley_n4() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension_known = 2;
        let dimension = 10;
        let problem1 = BenchmarkFunction::ackley_n4(dimension_known, None);
        let problem2 = BenchmarkFunction::ackley_n4(dimension, None);
        let optimum_position = vec![-1.51 / 35.0, -0.755 / 35.0];
        // Optimum only known for 2 dimensions!
        let optimum_value = - 4.590101633799122;
        let optimum_fitness = problem1.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem2.dimension())
            .map(|dimension| rng.gen_range(problem2.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem2.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_alpine_n1() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::alpine_n1(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_alpine_n2() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::alpine_n2(dimension, None);
        let optimum_position = vec![7.917 / 10.0; dimension];
        let optimum_value = - (2.808_f64).powi(dimension as i32);
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_brown() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::brown(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_exponential() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::exponential(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = - 1.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_griewank() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::griewank(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_happy_cat() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::happy_cat(dimension, None);
        let optimum_position = vec![- 1.0 / 2.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_periodic() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::periodic(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.9;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_powell_sum() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::powell_sum(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_qing() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 5;
        let problem = BenchmarkFunction::qing(dimension, None);
        let optimum_position = vec![(1.0_f64).sqrt() / 500.0, (2.0_f64).sqrt() / 500.0, (3.0_f64).sqrt() / 500.0, (4.0_f64).sqrt() / 500.0, (5.0_f64).sqrt() / 500.0];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
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

    # [test]
    fn test_ridge() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::ridge(dimension, None);
        let mut optimum_position = vec![- 5.0 / 5.0];
        let mut opt_rest = vec![0.0; dimension - 1];
        optimum_position.append(&mut opt_rest);
        let optimum_value = - 5.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_rosenbrock() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::rosenbrock(dimension, None);
        let optimum_position = vec![1.0 / 10.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_salomon() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::salomon(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }

    # [test]
    fn test_schwefel_220() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_220(dimension, None);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = 0.0;
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_eq!(optimum_fitness, optimum_value);

        // test for random values
        let random_position = (0..problem.dimension())
            .map(|dimension| rng.gen_range(problem.range(dimension)))
            .collect::<Vec<f64>>();
        let random_fitness = problem.evaluate(&random_position);
        assert!(random_fitness >= optimum_fitness);
    }
}