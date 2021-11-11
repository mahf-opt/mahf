fn abs_tol() -> f64 {
    0.0000_1
}

fn r1st_tol() -> f64 {
    1.0
}

fn scale_domain(value: &f64, min: f64, max: f64) -> f64 {
    2.0 * (value - min) / (max - min) - 1.0
}

mod bmf_tests {
    use crate::problems::{
        bmf::tests::{abs_tol, r1st_tol, scale_domain},
        bmf::*,
        Problem,
    };
    use float_eq::*;
    use proptest::prelude::*;
    use std::f64::consts::PI;

    #[test]
    fn test_optimum_sphere() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::sphere(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_sphere(x in prop::collection::vec(-1.0..1.0, 1..30)) {
            let problem = BenchmarkFunction::sphere(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_rastrigin() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::rastrigin(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_rastrigin(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::rastrigin(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_ackley() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::ackley(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_ackley(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::ackley(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_ackley_n4() {
        // test for known optimum
        let dimension_known = 2;
        let problem = BenchmarkFunction::ackley_n4(dimension_known);
        let x1 = scale_domain(&-1.51, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&-0.755, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        // Optimum only known for 2 dimensions!
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_ackley_n4(x in prop::collection::vec(-1.0f64..1.0f64, 1..2)) {
            let problem = BenchmarkFunction::ackley_n4(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_alpine_n1() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::alpine_n1(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_alpine_n1(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::alpine_n1(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_alpine_n2() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::alpine_n2(dimension);
        let xi = scale_domain(&7.917, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_alpine_n2(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::alpine_n2(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_brown() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::brown(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_brown(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::brown(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_exponential() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::exponential(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_exponential(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::exponential(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_griewank() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::griewank(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_griewank(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::griewank(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_happy_cat() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::happy_cat(dimension);
        let xi = scale_domain(&-1.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_happy_cat(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::happy_cat(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_periodic() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::periodic(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_periodic(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::periodic(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_powell_sum() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::powell_sum(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_powell_sum(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::powell_sum(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_qing() {
        // test for known optimum
        let dimension = 5;
        let problem = BenchmarkFunction::qing(dimension);
        let optimum_position = vec![
            (1.0_f64).sqrt() / 500.0,
            (2.0_f64).sqrt() / 500.0,
            (3.0_f64).sqrt() / 500.0,
            (4.0_f64).sqrt() / 500.0,
            (5.0_f64).sqrt() / 500.0,
        ];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_qing(x in prop::collection::vec(-1.0f64..1.0f64, 5)) {
            let problem = BenchmarkFunction::qing(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_ridge() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::ridge(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let mut optimum_position = vec![-5.0 / 5.0];
        let mut opt_rest = vec![xi; dimension - 1];
        optimum_position.append(&mut opt_rest);
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_ridge(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::ridge(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_rosenbrock() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::rosenbrock(dimension);
        let xi = scale_domain(&1.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_rosenbrock(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::rosenbrock(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_salomon() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::salomon(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_salomon(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::salomon(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_schwefel_220() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_220(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_schwefel_220(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::schwefel_220(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_schwefel_221() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_221(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_schwefel_221(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::schwefel_221(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_schwefel_222() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_222(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_schwefel_222(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::schwefel_222(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_schwefel_223() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel_223(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_schwefel_223(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::schwefel_223(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_schwefel() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::schwefel(dimension);
        let xi = scale_domain(&420.9687, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= 0.00_1,
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_schwefel(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::schwefel(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_shubert_n3() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::shubert_n3(dimension);
        // several optima, e.g. (-6.774576,-6.774576)
        let xi = scale_domain(&-6.774576, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_shubert_n3(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::shubert_n3(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_shubert_n4() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::shubert_n4(dimension);
        // several optima, positions of shubert_n3 + PI as cos instead of sin
        let xi = scale_domain(&(-6.774576 + PI), problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_shubert_n4(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::shubert_n4(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_shubert() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::shubert(dimension);
        let x1 = scale_domain(&-7.0835, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&4.8580, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_shubert(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::shubert(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_styblinski_tang() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::styblinski_tang(dimension);
        let xi = scale_domain(&-2.903534, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_styblinski_tang(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::styblinski_tang(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_sum_sqares() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::sum_squares(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_sum_squares(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::sum_squares(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_yang_n2() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::yang_n2(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_yang_n2(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::yang_n2(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_yang_n3() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::yang_n3(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_yang_n3(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::yang_n3(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_yang_n4() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::yang_n4(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_yang_n4(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::yang_n4(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_zakharov() {
        // test for known optimum
        let dimension = 10;
        let problem = BenchmarkFunction::zakharov(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_zakharov(x in prop::collection::vec(-1.0f64..1.0f64, 1..30)) {
            let problem = BenchmarkFunction::zakharov(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    // from here on: non-n-dimensional functions
    #[test]
    fn test_optimum_ackley_n2() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::ackley_n2(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_ackley_n2(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::ackley_n2(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_ackley_n3() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::ackley_n3(dimension);
        let x1 = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&-0.4, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_ackley_n3(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::ackley_n3(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_adjiman() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::adjiman(dimension);
        // domain for x and y differs
        let x1 = scale_domain(&2.0, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&0.10578, -1.0, 1.0);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_adjiman(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::adjiman(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_bartels_conn() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::bartels_conn(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_bartels_conn(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::bartels_conn(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_beale() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::beale(dimension);
        let x1 = scale_domain(&3.0, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&0.5, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_beale(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::beale(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_bird() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::bird(dimension);
        let x1 = scale_domain(&4.70104, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&3.15294, problem.domain()[0], problem.domain()[1]);
        let x3 = scale_domain(&-1.58214, problem.domain()[0], problem.domain()[1]);
        let x4 = scale_domain(&-3.13024, problem.domain()[0], problem.domain()[1]);
        let optimum_position1 = vec![x1, x2];
        let optimum_position2 = vec![x3, x4];
        let optimum_fitness1 = problem.evaluate(&optimum_position1);
        let optimum_fitness2 = problem.evaluate(&optimum_position2);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness1,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness2,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_bird(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::bird(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_bohachevsky_n1() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::bohachevsky_n1(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_bohachevsky_n1(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::bohachevsky_n1(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_bohachevsky_n2() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::bohachevsky_n2(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_bohachevsky_n2(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::bohachevsky_n2(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_booth() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::booth(dimension);
        let x1 = scale_domain(&1.0, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&3.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_booth(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::booth(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_brent() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::brent(dimension);
        let xi = scale_domain(&-10.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_brent(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::brent(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_bukin_n6() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::bukin_n6(dimension);
        // x and y have different domains
        let x1 = scale_domain(&-10.0, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&1.0, -3.0, 3.0);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_bukin_n6(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::bukin_n6(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_cross_in_tray() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::cross_in_tray(dimension);
        let x1 = scale_domain(
            &1.349_406_685_353_34,
            problem.domain()[0],
            problem.domain()[1],
        );
        let x2 = scale_domain(
            &1.349_406_608_602_084,
            problem.domain()[0],
            problem.domain()[1],
        );
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_cross_in_tray(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::cross_in_tray(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_deckkers_aarts() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::deckkers_aarts(dimension);
        let x1 = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&15.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_deckkers_aarts(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::deckkers_aarts(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_drop_wave() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::drop_wave(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_drop_wave(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::drop_wave(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_easom() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::easom(dimension);
        let xi = scale_domain(&PI, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_easom(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::easom(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_egg_crate() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::egg_crate(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_egg_crate(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::egg_crate(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_goldstein_price() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::goldstein_price(dimension);
        let x1 = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&-1.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_goldstein_price(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::goldstein_price(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_gramacy_lee() {
        // test for known optimum
        let dimension = 1;
        let problem = BenchmarkFunction::gramacy_lee(dimension);
        let xi = scale_domain(&0.548563444114526, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_gramacy_lee(x in prop::collection::vec(-1.0f64..1.0f64, 1)) {
            let problem = BenchmarkFunction::gramacy_lee(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_himmelblau() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::himmelblau(dimension);
        let x1 = scale_domain(&3.0, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&2.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_himmelblau(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::himmelblau(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_holder_table() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::holder_table(dimension);
        let x1 = scale_domain(&8.05502, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&9.66459, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_holder_table(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::holder_table(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_keane() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::keane(dimension);
        let x1 = scale_domain(&1.393249070031784, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_keane(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::keane(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_leon() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::leon(dimension);
        let xi = scale_domain(&1.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_leon(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::leon(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_levi_n13() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::levi_n13(dimension);
        let xi = scale_domain(&1.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_levi_n13(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::levi_n13(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_matyas() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::matyas(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_matyas(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::matyas(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_mccormick() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::mccormick(dimension);
        // different domains for x and y
        let x1 = scale_domain(&-0.547, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&-1.547, -3.0, 3.0);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_mccormick(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::mccormick(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_schaffer_n1() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::schaffer_n1(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_schaffer_n1(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::schaffer_n1(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_schaffer_n2() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::schaffer_n2(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_schaffer_n2(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::schaffer_n2(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_schaffer_n3() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::schaffer_n3(dimension);
        let x1 = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&1.253115, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_schaffer_n3(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::schaffer_n3(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_schaffer_n4() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::schaffer_n4(dimension);
        let x1 = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let x2 = scale_domain(&1.253115, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![x1, x2];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_schaffer_n4(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::schaffer_n4(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_three_hump_camel() {
        // test for known optimum
        let dimension = 2;
        let problem = BenchmarkFunction::three_hump_camel(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_three_hump_camel(x in prop::collection::vec(-1.0f64..1.0f64, 2)) {
            let problem = BenchmarkFunction::three_hump_camel(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }

    #[test]
    fn test_optimum_wolfe() {
        // test for known optimum
        let dimension = 3;
        let problem = BenchmarkFunction::wolfe(dimension);
        let xi = scale_domain(&0.0, problem.domain()[0], problem.domain()[1]);
        let optimum_position = vec![xi; dimension];
        let optimum_fitness = problem.evaluate(&optimum_position);
        assert_float_eq!(
            problem.known_optimum(),
            optimum_fitness,
            abs <= abs_tol(),
            r1st <= r1st_tol()
        );
    }

    proptest! {
        #[test]
        fn test_random_input_wolfe(x in prop::collection::vec(-1.0f64..1.0f64, 3)) {
            let problem = BenchmarkFunction::wolfe(x.len());
            let random_fitness = problem.evaluate(&x);
            prop_assert!(random_fitness >= problem.known_optimum());
        }
    }
}
