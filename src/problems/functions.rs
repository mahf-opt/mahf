//! Collection of test functions from [benchmarkfcns.xyz](http://benchmarkfcns.xyz)

use crate::{
    problem::{LimitedVectorProblem, Problem, VectorProblem},
    random::Random,
};

use rand::Rng;

/// Wraps the benchmark functions as [`Problem`]s.
///
/// All functions have been scaled to [-1, 1].
#[derive(serde::Serialize)]
pub struct BenchmarkFunction {
    name: &'static str,
    dimension: usize,
    rng: Option<Random>,
    rnd: f64,

    #[serde(skip)]
    implementation: Function,
}

impl BenchmarkFunction {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }
}

impl BenchmarkFunction {
    /// The [Sphere](http://benchmarkfcns.xyz/benchmarkfcns/spherefcn.html) function.
    pub fn sphere(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "sphere",
            implementation: scaled_implementations::sphere,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Rastrigin](http://benchmarkfcns.xyz/benchmarkfcns/rastriginfcn.html) function.
    pub fn rastrigin(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "rstrigin",
            implementation: scaled_implementations::rastrigin,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Ackley](http://benchmarkfcns.xyz/benchmarkfcns/ackleyfcn.html) function.
    pub fn ackley(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "ackley",
            implementation: scaled_implementations::ackley,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [AckleyN4](http://benchmarkfcns.xyz/benchmarkfcns/ackleyn4fcn.html) function.
    pub fn ackley_n4(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "ackleyN4",
            implementation: scaled_implementations::ackley_n4,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [AlpineN1](http://benchmarkfcns.xyz/benchmarkfcns/alpinen1fcn.html) function.
    pub fn alpine_n1(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "alpineN1",
            implementation: scaled_implementations::alpine_n1,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [AlpineN2](http://benchmarkfcns.xyz/benchmarkfcns/alpinen2fcn.html) function.
    pub fn alpine_n2(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "alpineN2",
            implementation: scaled_implementations::alpine_n2,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Brown](http://benchmarkfcns.xyz/benchmarkfcns/brownfcn.html) function.
    pub fn brown(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "brown",
            implementation: scaled_implementations::brown,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Exponential](http://benchmarkfcns.xyz/benchmarkfcns/exponentialfcn.html) function.
    pub fn exponential(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "exponential",
            implementation: scaled_implementations::exponential,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Griewank](http://benchmarkfcns.xyz/benchmarkfcns/griewankfcn.html) function.
    pub fn griewank(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "griewank",
            implementation: scaled_implementations::griewank,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Happy Cat](http://benchmarkfcns.xyz/benchmarkfcns/happycatfcn.html) function.
    pub fn happy_cat(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "happyCat",
            implementation: scaled_implementations::happy_cat,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Periodic](http://benchmarkfcns.xyz/benchmarkfcns/periodicfcn.html) function.
    pub fn periodic(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "periodic",
            implementation: scaled_implementations::periodic,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Powell Sum](http://benchmarkfcns.xyz/benchmarkfcns/powellsumfcn.html) function.
    pub fn powell_sum(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "powellSum",
            implementation: scaled_implementations::powell_sum,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Qing](http://benchmarkfcns.xyz/benchmarkfcns/qingfcn.html) function.
    pub fn qing(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "qing",
            implementation: scaled_implementations::qing,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Quartic](http://benchmarkfcns.xyz/benchmarkfcns/quarticfcn.html) function.
    pub fn quartic(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "quartic",
            implementation: scaled_implementations::quartic,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Ridge](http://benchmarkfcns.xyz/benchmarkfcns/ridgefcn.html) function.
    pub fn ridge(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "ridge",
            implementation: scaled_implementations::ridge,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Rosenbrock](http://benchmarkfcns.xyz/benchmarkfcns/rosenbrockfcn.html) function.
    pub fn rosenbrock(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "rosenbrock",
            implementation: scaled_implementations::rosenbrock,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Salomon](http://benchmarkfcns.xyz/benchmarkfcns/salomonfcn.html) function.
    pub fn salomon(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "salomon",
            implementation: scaled_implementations::salomon,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Schwefel 2.20](http://benchmarkfcns.xyz/benchmarkfcns/schwefel220fcn.html) function.
    pub fn schwefel_220(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "schwefel220",
            implementation: scaled_implementations::schwefel_220,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Schwefel 2.21](http://benchmarkfcns.xyz/benchmarkfcns/schwefel221fcn.html) function.
    pub fn schwefel_221(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "schwefel221",
            implementation: scaled_implementations::schwefel_221,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Schwefel 2.22](http://benchmarkfcns.xyz/benchmarkfcns/schwefel222fcn.html) function.
    pub fn schwefel_222(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "schwefel222",
            implementation: scaled_implementations::schwefel_222,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Schwefel 2.23](http://benchmarkfcns.xyz/benchmarkfcns/schwefel223fcn.html) function.
    pub fn schwefel_223(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "schwefel223",
            implementation: scaled_implementations::schwefel_223,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Schwefel](http://benchmarkfcns.xyz/benchmarkfcns/schwefelfcn.html) function.
    pub fn schwefel(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "schwefel",
            implementation: scaled_implementations::schwefel,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Shubert Nr. 3](http://benchmarkfcns.xyz/benchmarkfcns/shubert3fcn.html) function.
    pub fn shubert_n3(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "shubertN3",
            implementation: scaled_implementations::shubert_n3,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Shubert Nr. 4](http://benchmarkfcns.xyz/benchmarkfcns/shubert4fcn.html) function.
    pub fn shubert_n4(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "shubertN4",
            implementation: scaled_implementations::shubert_n4,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Shubert](http://benchmarkfcns.xyz/benchmarkfcns/shubertfcn.html) function.
    pub fn shubert(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "shubert",
            implementation: scaled_implementations::shubert,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Styblinski-Tank](http://benchmarkfcns.xyz/benchmarkfcns/styblinskitankfcn.html) function.
    pub fn styblinksi_tank(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "styblinskiTank",
            implementation: scaled_implementations::styblinksi_tank,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Sum Squares](http://benchmarkfcns.xyz/benchmarkfcns/sumsquaresfcn.html) function.
    pub fn sum_squares(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "sumSquares",
            implementation: scaled_implementations::sum_squares,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Xin-She Yang Nr. 1](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn1fcn.html) function.
    pub fn yang_n1(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "yangN1",
            implementation: scaled_implementations::yang_n1,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Xin-She Yang Nr. 2](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn2fcn.html) function.
    pub fn yang_n2(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "yangN2",
            implementation: scaled_implementations::yang_n2,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Xin-She Yang Nr. 3](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn3fcn.html) function.
    pub fn yang_n3(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "yangN3",
            implementation: scaled_implementations::yang_n3,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Xin-She Yang Nr. 4](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn4fcn.html) function.
    pub fn yang_n4(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "yangN4",
            implementation: scaled_implementations::yang_n4,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Zakharov](http://benchmarkfcns.xyz/benchmarkfcns/zakharov.html) function.
    pub fn zakharov(dimension: usize, mut rng: Option<Random>) -> Self {
        BenchmarkFunction {
            name: "zakharov",
            implementation: scaled_implementations::zakharov,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Ackley Nr. 2](http://benchmarkfcns.xyz/benchmarkfcns/ackleyn2fcn.html) function.
    pub fn ackley_n2(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "ackleyN2",
            implementation: scaled_implementations::ackley_n2,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Ackley Nr. 3](http://benchmarkfcns.xyz/benchmarkfcns/ackleyn3fcn.html) function.
    pub fn ackley_n3(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "ackleyN3",
            implementation: scaled_implementations::ackley_n3,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Adjiman](http://benchmarkfcns.xyz/benchmarkfcns/adjimanfcn.html) function.
    pub fn adjiman(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "adjiman",
            implementation: scaled_implementations::adjiman,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Bartels Conn](http://benchmarkfcns.xyz/benchmarkfcns/bartelsconnfcn.html) function.
    pub fn bartels_conn(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "bartelsConn",
            implementation: scaled_implementations::bartels_conn,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Beale](http://benchmarkfcns.xyz/benchmarkfcns/bealefcn.html) function.
    pub fn beale(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "beale",
            implementation: scaled_implementations::beale,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Bird](http://benchmarkfcns.xyz/benchmarkfcns/birdfcn.html) function.
    pub fn bird(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "bird",
            implementation: scaled_implementations::bird,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Bohachevsky Nr. 1](http://benchmarkfcns.xyz/benchmarkfcns/bohachevskyn1fcn.html) function.
    pub fn bohachevsky_n1(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "bohachevskyN1",
            implementation: scaled_implementations::bohachevsky_n1,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Bohachevsky Nr. 2](http://benchmarkfcns.xyz/benchmarkfcns/bohachevskyn2fcn.html) function.
    pub fn bohachevsky_n2(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "bohachevskyN2",
            implementation: scaled_implementations::bohachevsky_n2,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Booth](http://benchmarkfcns.xyz/benchmarkfcns/boothfcn.html) function.
    pub fn booth(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "booth",
            implementation: scaled_implementations::booth,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Brent](http://benchmarkfcns.xyz/benchmarkfcns/brentfcn.html) function.
    pub fn brent(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "brent",
            implementation: scaled_implementations::brent,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }

    /// The [Bukin Nr. 6](http://benchmarkfcns.xyz/benchmarkfcns/bukinn6fcn.html) function.
    pub fn bukin_n6(dimension: usize, mut rng: Option<Random>) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "bukinN6",
            implementation: scaled_implementations::bukin_n6,
            dimension,
            rng,
            rnd: rng.gen::<f64>(),
        }
    }
}

impl Problem for BenchmarkFunction {
    type Encoding = Vec<f64>;

    fn evaluate(&self, solution: &Self::Encoding) -> f64 {
        (self.implementation)(solution)
    }

    fn name(&self) -> &str {
        self.name
    }
}

impl VectorProblem for BenchmarkFunction {
    type T = f64;

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl LimitedVectorProblem for BenchmarkFunction {
    fn range(&self, _dimension: usize) -> std::ops::Range<Self::T> {
        0.0..1.0
    }
}

/// A benchmark function.
pub type Function = fn(&[f64], &Option<f64>) -> f64;

/// The benchmark functions scaled to [-1.0, 1.0].
pub mod scaled_implementations {
    use std::f64::consts::PI;
    use std::intrinsics::sqrtf64;
    use rand::Rng;

    /// Sphere function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    pub fn sphere(x: &[f64], rnd: &Option<f64>) -> f64 {
        x.iter().map(|xi| xi * 5.12).map(|xi| xi * xi).sum()
    }

    /// Rastrinin function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    pub fn rastrigin(x: &[f64], rnd: &Option<f64>) -> f64 {
        let n = x.len() as f64;
        10.0 * n
            + x.iter()
                .map(|xi| xi * 5.12)
                .map(|xi| xi * xi - 10.0 * (2.0 * PI * xi).cos())
                .sum::<f64>()
    }

    /// Ackley function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    pub fn ackley(x: &[f64], rnd: &Option<f64>) -> f64 {
        let a = 20.;
        let b = 0.2;
        let c = 2.0 * PI;

        let n_inverse = 1.0 / x.len() as f64;
        let squared_sum = x
            .iter()
            .map(|xi| xi * 32.0)
            .map(|xi| xi.powi(2))
            .sum::<f64>();
        let cosine_sum = x
            .iter()
            .map(|xi| xi * 32.0)
            .map(|xi| (c * xi).cos())
            .sum::<f64>();

        a + (1.0f64).exp() + (-a) * ((-b) * (n_inverse * squared_sum).sqrt()).exp()
            - (n_inverse * cosine_sum).exp()
    }

    /// Ackley Nr. 4 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: on 2-dimensional space -4.590101633799122 at (−1.51,−0.755)
    //TODO: Unit test!
    pub fn ackley_n4(x: &[f64], rnd: &Option<f64>) -> f64 {
        let mut sum = 0.0;
        for i in x.len() {
            for j in x.len() - 1 {
                i = i * 35.0;
                j = j * 35.0;
                sum += (-0.2).exp() * (j.powi(2) + i.powi(2)).sqrt() + 3.0 * ((2.0 * j).cos() + (2.0 * i).sin());
            }
        }
        sum
    }

    /// Alpine Nr. 1 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn alpine_n1(x: &[f64], rnd: &Option<f64>) -> f64 {
        x.iter()
            .map(|xi| (xi * 10.0).abs())
            .map(|xi| xi * xi.sin() + 0.1 * xi)
            .sum::<f64>()
    }

    /// Alpine Nr. 2 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -2.808^n at (7.917,...,7.917) (minimisation by negation)
    //TODO: Unit test!
    pub fn alpine_n2(x: &[f64], rnd: &Option<f64>) -> f64 {
        - x.iter()
            .map(|xi| (xi * 10.0).abs())
            .map(|xi| xi.sqrt() * xi.sin())
            .product()
    }

    /// Brown function
    ///
    /// Scaled to [-1.0, 1.0] (from [-4,4])
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn brown(x: &[f64], rnd: &Option<f64>) -> f64 {
        let mut sum = 0.0;
        for i in x.len() {
            for j in x.len() - 1 {
                i = i * 4.0;
                j = j * 4.0;
                sum += (j.powi(2)).powi(i.powi(2) + 1.0) + (i.powi(2)).powi(j.powi(2) + 1.0);
            }
        }
        sum
    }

    /// Exponential function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum:  at (0,...,0)
    //TODO: Unit test!
    pub fn exponential(x: &[f64], rnd: &Option<f64>) -> f64 {
        let sum = x
            .iter()
            .map(|xi| xi.powi(2))
            .sum::<f64>();
        - ((-0.5) * sum).exp()
    }

    /// Griewank function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn griewank(x: &[f64], rnd: &Option<f64>) -> f64 {
        let sum = x.iter()
            .map(|xi| xi * 600.0)
            .map(|xi| xi.powi(2) / 4000.0)
            .sum();
        let prod = x.iter()
            .enumerate()
            .map(|(i, &xi)| xi * 600.0)
            .map(|(i, &xi)| (xi / i).cos())
            .product();
        1 + sum + prod
    }

    /// Happy Cat function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (-1,...,-1)
    //TODO: Unit test!
    pub fn happy_cat(x: &[f64], rnd: &Option<f64>) -> f64 {
        let n = x.len();
        let alpha = 1 / 8;
        let norm = x.iter()
            .map(|xi| xi * 2.0)
            .map(|xi| xi.powi(2))
            .sum::<f64>()
            .sqrt();

        let sum = x.iter()
            .map(|xi| xi * 2.0)
            .map(|xi| xi)
            .sum::<f64>();

        ((norm - n).powi(2)).powi(alpha) + (1.0 / n) * (0.5 * norm + sum) + 0.5
    }

    /// Periodic function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0.9 at (0,...,0)
    //TODO: Unit test!
    pub fn periodic(x: &[f64], rnd: &Option<f64>) -> f64 {
        let sum = x
            .iter()
            .map(|xi| xi * 10.0)
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        let sine_sum = x
            .iter()
            .map(|xi| xi * 10.0)
            .map(|xi| (xi.sin()).powi(2))
            .sum::<f64>();

        1.0 + sine_sum - 0.1 * (sum).exp()
    }

    /// Powell Sum function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn powell_sum(x: &[f64], rnd: &Option<f64>) -> f64 {
        x.iter()
            .enumerate()
            .map(|(i, xi)| (xi.abs()).powi(((i + 1) as i32)))
            .sum::<f64>()
    }

    /// Qing function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (+-(i).sqrt(),...,+-(i).sqrt())
    //TODO: Unit test!
    pub fn qing(x: &[f64], rnd: &Option<f64>) -> f64 {
        x.iter()
            .enumerate()
            .map(|(i, xi)| xi * 500.0)
            .map(|(i, xi)| (xi.powi(2) - i).powi(2) )
            .sum::<f64>()
    }

    /// Quartic function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 + rnd at (0,...,0)
    //TODO: Unit test!
    pub fn quartic(x: &[f64], rnd: &Option<f64>) -> f64 {
        sum = x.iter()
            .enumerate()
            .map(|(i, xi)| xi * 1.28)
            .map(|(i, xi)| i * xi.powi(4))
            .sum::<f64>();

        sum + rnd
    }

    /// Ridge function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -5 at (-5,0,...,0) for input domain [-5,5]
    //TODO: Unit test!
    pub fn ridge(x: &[f64], rnd: &Option<f64>) -> f64 {
        let d = 1.0;
        let alpha = 0.5;
        let first = x[0];

        let sum = x[1..].iter()
            .enumerate()
            .map(|(i, xi)| xi * 5.0)
            .map(|(i, xi)| xi.powi(2))
            .sum::<f64>();

        first + d * sum.powf(alpha)
    }

    /// Rosenbrock function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (1,...1)
    //TODO: Unit test!
    pub fn rosenbrock(x: &[f64], rnd: &Option<f64>) -> f64 {
        let a = 1.0;
        let b = 100.0;
        let mut sum = 0.0;

        for i in x.len() {
            for j in x.len() - 1 {
                i = i * 10;
                j = j * 10;
                sum += b * (i + j.powi(2)).powi(2) + (a - j).powi(2);
            }
        }
        sum
    }

    /// Salomon function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0)
    //TODO: Unit test!
    pub fn salomon(x: &[f64], rnd: &Option<f64>) -> f64 {
        let sum = x.iter()
            .map(|xi| xi * 100.0)
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        1.0 - (2.0 * PI * sum.sqrt()).cos() + 0.1 * sum.sqrt()
    }

    /// Schwefel 2.20 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0)
    //TODO: Unit test!
    pub fn schwefel_220(x: &[f64], rnd: &Option<f64>) -> f64 {
        x.iter()
            .map(|xi| xi * 100.0)
            .map(|xi| xi.abs())
            .sum()
    }

    /// Schwefel 2.21 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0)
    //TODO: Unit test!
    pub fn schwefel_221(x: &[f64], rnd: &Option<f64>) -> f64 {
        max_elem = x.iter()
            .map(|xi| (xi * 100.0).abs())
            .fold(f64::NEG_INFINITY, f64::max);
            //.max_by(|a, b| a.total_cmp(b));

        max_elem
    }

    /// Schwefel 2.22 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0)
    //TODO: Unit test!
    pub fn schwefel_222(x: &[f64], rnd: &Option<f64>) -> f64 {
        let sum = x.iter()
            .map(|xi| xi * 100.0)
            .map(|xi| xi.abs())
            .sum::<f64>();

        let prod = x.iter()
            .map(|xi| xi * 100.0)
            .map(|xi| xi.abs())
            .product::<f64>();

        sum + prod
    }

    /// Schwefel 2.23 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0)
    //TODO: Unit test!
    pub fn schwefel_223(x: &[f64], rnd: &Option<f64>) -> f64 {
        x.iter()
            .map(|xi| xi * 10.0)
            .map(|xi| xi.powi(10))
            .sum::<f64>()

    }

    /// Schwefel function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (420.9687,...420.9687)
    //TODO: Unit test!
    pub fn schwefel(x: &[f64], rnd: &Option<f64>) -> f64 {
        let sum = x.iter()
            .map(|xi| xi * 500.0)
            .map(|xi| xi * ((xi.abs()).sqrt()).sin())
            .sum::<f64>();

        418.9829 * x.len() - sum
    }

    /// Shubert Nr. 3 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: ~ -29.6733337
    //TODO: Unit test!
    pub fn shubert_n3(x: &[f64], rnd: &Option<f64>) -> f64 {
        let mut sum = 0.0;
        for i in x.len() {
            for j in 1..=5 {
                i = i * 10;
                sum += j * ((j + 1) * i + j).sin();
            }
        }
        sum
    }

    /// Shubert Nr. 4 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: ~ -25.740858
    //TODO: Unit test!
    pub fn shubert_n4(x: &[f64], rnd: &Option<f64>) -> f64 {
        let mut sum = 0.0;
        for i in x.len() {
            for j in 1..=5 {
                i = i * 10;
                sum += j * ((j + 1) * i + j).cos();
            }
        }
        sum
    }

    /// Shubert function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: ~ -186.7309
    //TODO: Unit test!
    pub fn shubert(x: &[f64], rnd: &Option<f64>) -> f64 {
        let mut prod = 1.0;
        for i in x.len() {
            let mut sum = 0.0;
            for j in 1..=5 {
                i = i * 10.0;
                sum += ((j + 1) * i + j).cos();
            }
            prod = prod * sum
        }
        prod
    }

    /// Styblinski-Tank function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -39.16599 * n at (-2.903534,...,-2.903534)
    //TODO: Unit test!
    pub fn styblinksi_tank(x: &[f64], rnd: &Option<f64>) -> f64 {
        0.5 * x.iter()
            .map(|xi| xi * 5.0)
            .map(|xi| xi.powi(4) - 16.0 * xi.powi(2) + 5.0 * xi)
            .sum()
    }

    /// Sum Squares function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn sum_squares(x: &[f64], rnd: &Option<f64>) -> f64 {
        x.iter()
            .enumerate()
            .map(|(i, xi)| xi * 10.0)
            .map(|(i, xi)| i * xi.powi(2))
            .sum()
    }

    /// Xin-She Yang Nr. 1 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn yang_n1(x: &[f64], rnd: &Option<f64>) -> f64 {
        x.iter()
            .enumerate()
            .map(|(i, xi)| xi * 5.0)
            .map(|(i, xi)| rnd * (xi.abs()).powi(i))
            .sum()
    }

    /// Xin-She Yang Nr. 2 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn yang_n2(x: &[f64], rnd: &Option<f64>) -> f64 {
        let sum = x.iter()
          .map(|xi| xi * 2.0 * PI)
          .map(|xi| xi.abs())
          .sum::<f64>();

        let exp_sum = x.iter()
            .map(|xi| xi * 2.0 * PI)
            .map(|xi| (xi.powi(2)).sin())
            .sum::<f64>();

        sum * (-exp_sum).exp()
    }

    /// Xin-She Yang Nr. 3 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -1 at (0,...,0)
    //TODO: Unit test!
    pub fn yang_n3(x: &[f64], rnd: &Option<f64>) -> f64 {
        let beta = 15.0;
        let m = 5.0;

        let beta_sum = x.iter()
            .map(|xi| xi * 2.0 * PI)
            .map(|xi| (xi / beta).powi(2 * m))
            .sum::<f64>();

        let sum = x.iter()
            .map(|xi| xi * 2.0 * PI)
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        let prod = x.iter()
            .map(|xi| xi * 2.0 * PI)
            .map(|xi| (xi.cos()).powi(2))
            .sum::<f64>();

        (beta_sum).exp() - 2.0 * (sum).exp() * prod
    }

    /// Xin-She Yang Nr. 4 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -1 at (0,...,0)
    //TODO: Unit test!
    pub fn yang_n4(x: &[f64], rnd: &Option<f64>) -> f64 {
        let inner_exp_sum = x.iter()
            .map(|xi| xi * 10.0)
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        let outer_exp_sum = x.iter()
            .map(|xi| xi * 10.0)
            .map(|xi| ((xi.abs()).sin()).powi(2))
            .sum::<f64>();

        let sum = x.iter()
            .map(|xi| xi * 10.0)
            .map(|xi| (xi.sin()).powi(2))
            .sum::<f64>();

        (sum - (-inner_exp_sum).exp()) * (-outer_exp_sum).exp()
    }

    /// Zakharov function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0), here on input domain [-10,10]
    //TODO: Unit test!
    pub fn zakharov(x: &[f64], rnd: &Option<f64>) -> f64 {
        let i_sum = x.iter()
            .enumerate()
            .map(|(i, xi)| xi * 10.0)
            .map(|(i, xi)| 0.5 * i * xi)
            .sum::<f64>();

        let sum = x.iter()
            .map(|xi| xi * 10.0)
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        sum + (i_sum).powi(2) + (i_sum).powi(4)
    }

    /// Ackley Nr. 2 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -200 at (0,0), here on input domain [-32,32]
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    pub fn ackley_n2(x: &[f64], rnd: &Option<f64>) -> f64 {
        - 200.0 * (- 0.2 * ((x[0] * 32.0).powi(2) + (x[1] * 32.0).powi(2)).sqrt()).exp()
    }

    /// Ackley Nr. 3 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: −195.629028238419 at (±0.682584587365898,−0.36075325513719), here on input domain [-32,32]
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    pub fn ackley_n3(x: &[f64], rnd: &Option<f64>) -> f64 {
        - 200.0 * (- 0.2 * ((x[0] * 32.0).powi(2) + (x[1] * 32.0).powi(2)).sqrt()).exp() +
            5.0 * ((3 * (x[0] * 32.0)).cos() + (3.0 * (x[1] * 32.0).sin())).exp()
    }

    /// Adjiman function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: −2.02181 at (0,0), here on input domain [-1,1]
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    pub fn adjiman(x: &[f64], rnd: &Option<f64>) -> f64 {
        x[0].cos() + x[1].sin() - (x[0] / (x[1].powi(2) + 1.0))
    }

    /// Bartels Conn function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 1 at (0,0), here on input domain [-500,500]
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    pub fn bartels_conn(x: &[f64], rnd: &Option<f64>) -> f64 {
        (x[0].powi(2) + x[1].powi(2) + (x[0] * x[1])).abs() + (x[0].sin()).abs() + (x[1].cos()).abs()
    }

    /// Beale function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (3,0.5), here on input domain [-4.5,4.5]
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    pub fn beale(x: &[f64], rnd: &Option<f64>) -> f64 {
        (1.5 - x[0] + (x[0] * x[1])).powi(2) +
            (2.25 - x[0] + (x[0] * x[1].powi(2))).powi(2) +
            (2.625 - x[0] + (x[0] * x[1].powi(3))).powi(2)
    }

    /// Bird function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: −106.764537 at (4.70104,3.15294) and (−1.58214,−3.13024), here on input domain [-2 * Pi,2 * Pi]
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    pub fn bird(x: &[f64], rnd: &Option<f64>) -> f64 {
        let a = x[0] * 2.0 * PI;
        let b = x[1] * 2.0 * PI;
        a.sin() * ((1.0 - b.cos()).powi(2)).exp() + b.cos() * ((1.0 - a.sin()).powi(2)).exp() + (a - b).powi(2)
    }

    /// Bohachevsky Nr. 1 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,0), here on input domain [-100,100]
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    pub fn bohachevsky_n1(x: &[f64], rnd: &Option<f64>) -> f64 {
        let a = x[0] * 100.0;
        let b = x[1] * 100.0;
        a.powi(2) + (2.0 * b.powi(2)) - (0.3 * (3.0 * PI * a).cos()) - (0.4 * (4.0 * PI * b).cos()) + 0.7
    }

    /// Bohachevsky Nr. 2 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,0), here on input domain [-100,100]
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    pub fn bohachevsky_n2(x: &[f64], rnd: &Option<f64>) -> f64 {
        let a = x[0] * 100.0;
        let b = x[1] * 100.0;
        a.powi(2) + (2.0 * b.powi(2)) - (0.3 * (3.0 * PI * a).cos() * (4.0 * PI * b).cos()) + 0.3
    }

    /// Booth function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (1,3), here on input domain [-10,10]
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    pub fn booth(x: &[f64], rnd: &Option<f64>) -> f64 {
        let a = x[0] * 10.0;
        let b = x[1] * 10.0;
        (a + (2.0 * b) - 7.0).powi(2) + ((2.0 * a) + b - 5.0).powi(2)
    }

    /// Brent function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: e^(-200) at (-10,-10), here on input domain [-20,0]
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    //TODO: For functions with range not symmetric around 0, look for better scaling options!
    pub fn brent(x: &[f64], rnd: &Option<f64>) -> f64 {
        let a = - (x[0] * 20.0).abs();
        let b = - (x[1] * 20.0).abs();
        (a + 10.0).powi(2) + (b + 10.0).powi(2) + (- a.powi(2) - b.powi(2)).exp()
    }

    /// Bukin Nr. 6 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (-10,1), here on input domain [-15,0] for x and [-3,3] for y (usually [-15,-5] for x and [-3,3] for y)
    /// Defined only on 2-dimensional space.
    //TODO: Unit test!
    //TODO: For functions with range not symmetric around 0, look for better scaling options!
    pub fn bukin_n6(x: &[f64], rnd: &Option<f64>) -> f64 {
        let a = x[0] * 15;
        let b = x[1] * 3;
        100.0 * ((b - 0.01 * a.powi(2)).abs()).sqrt() + 0.01 * (a + 10.0).abs()
    }
}
