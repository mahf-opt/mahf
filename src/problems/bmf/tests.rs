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
        let problem = BenchmarkFunction::sphere(dimension);
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
        let problem = BenchmarkFunction::rastrigin(dimension);
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
        let problem = BenchmarkFunction::ackley(dimension);
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
        let problem1 = BenchmarkFunction::ackley_n4(dimension_known);
        let problem2 = BenchmarkFunction::ackley_n4(dimension);
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
        let problem = BenchmarkFunction::alpine_n1(dimension);
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
        let problem = BenchmarkFunction::alpine_n2(dimension);
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
        let problem = BenchmarkFunction::brown(dimension);
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
        let problem = BenchmarkFunction::exponential(dimension);
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
        let problem = BenchmarkFunction::griewank(dimension);
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
        let problem = BenchmarkFunction::happy_cat(dimension);
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
        let problem = BenchmarkFunction::periodic(dimension);
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
        let problem = BenchmarkFunction::powell_sum(dimension);
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
        let problem = BenchmarkFunction::qing(dimension);
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
    fn test_ridge() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::ridge(dimension);
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
        let problem = BenchmarkFunction::rosenbrock(dimension);
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
        let problem = BenchmarkFunction::salomon(dimension);
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
        let problem = BenchmarkFunction::schwefel_220(dimension);
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
    fn test_schwefel_221() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_221(dimension);
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
    fn test_schwefel_222() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_222(dimension);
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
    fn test_schwefel_223() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_223(dimension);
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
    fn test_schwefel() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel(dimension);
        let optimum_position = vec![420.9687 / 500.0; dimension];
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
    fn test_shubert_n3() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::shubert_n3(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = - 29.6733337;
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
    fn test_shubert_n4() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::shubert_n4(dimension);
        let optimum_position = vec![0.0; dimension];
        let optimum_value = - 25.740858;
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
    fn test_shubert() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 2;
        let problem = BenchmarkFunction::shubert(dimension);
        let optimum_position = vec![- 7.0835_f64 / 10.0, 4.8580_f64 / 10.0];
        let optimum_value = - 186.7309;
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
    fn test_styblinski_tank() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::styblinksi_tank(dimension);
        let optimum_position = vec![- 2.903534 / 5.0; dimension];
        let optimum_value = - 39.16599 * dimension as f64;
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
    fn test_sum_sqares() {
        // test for known optimum
        let rng = &mut Random::default();
        let dimension = 10;
        let problem = BenchmarkFunction::sum_squares(dimension);
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