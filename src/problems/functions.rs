//! Collection of test functions from [benchmarkfcns.xyz](http://benchmarkfcns.xyz)

use crate::problem::{LimitedVectorProblem, Problem, VectorProblem};

/// Wraps the benchmark functions as [`Problem`]s.
///
/// All functions have been scaled to [-1, 1].
#[derive(serde::Serialize)]
pub struct BenchmarkFunction {
    name: &'static str,
    dimension: usize,

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
    pub fn sphere(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "sphere",
            implementation: scaled_implementations::sphere,
            dimension,
        }
    }

    /// The [Rastrigin](http://benchmarkfcns.xyz/benchmarkfcns/rastriginfcn.html) function.
    pub fn rastrigin(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "rstrigin",
            implementation: scaled_implementations::rastrigin,
            dimension,
        }
    }

    /// The [Ackley](http://benchmarkfcns.xyz/benchmarkfcns/ackleyfcn.html) function.
    pub fn ackley(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "ackley",
            implementation: scaled_implementations::ackley,
            dimension,
        }
    }

    /// The [AckleyN4](http://benchmarkfcns.xyz/benchmarkfcns/ackleyn4fcn.html) function.
    pub fn ackley_n4(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "ackleyN4",
            implementation: scaled_implementations::ackley_n4,
            dimension
        }
    }

    /// The [AlpineN1](http://benchmarkfcns.xyz/benchmarkfcns/alpinen1fcn.html) function.
    pub fn alpine_n1(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "alpineN1",
            implementation: scaled_implementations::alpine_n1,
            dimension
        }
    }

    /// The [AlpineN2](http://benchmarkfcns.xyz/benchmarkfcns/alpinen2fcn.html) function.
    pub fn alpine_n2(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "alpineN2",
            implementation: scaled_implementations::alpine_n2,
            dimension
        }
    }

    /// The [Brown](http://benchmarkfcns.xyz/benchmarkfcns/brownfcn.html) function.
    pub fn brown(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "brown",
            implementation: scaled_implementations::brown,
            dimension
        }
    }

    /// The [Exponential](http://benchmarkfcns.xyz/benchmarkfcns/exponentialfcn.html) function.
    pub fn exponential(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "exponential",
            implementation: scaled_implementations::exponential,
            dimension
        }
    }

    /// The [Griewank](http://benchmarkfcns.xyz/benchmarkfcns/griewankfcn.html) function.
    pub fn griewank(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "griewank",
            implementation: scaled_implementations::griewank,
            dimension
        }
    }

    /// The [Happy Cat](http://benchmarkfcns.xyz/benchmarkfcns/happycatfcn.html) function.
    pub fn happy_cat(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "happyCat",
            implementation: scaled_implementations::happy_cat,
            dimension
        }
    }

    /// The [Periodic](http://benchmarkfcns.xyz/benchmarkfcns/periodicfcn.html) function.
    pub fn periodic(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "periodic",
            implementation: scaled_implementations::periodic,
            dimension
        }
    }

    /// The [Powell Sum](http://benchmarkfcns.xyz/benchmarkfcns/powellsumfcn.html) function.
    pub fn powell_sum(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "powellSum",
            implementation: scaled_implementations::powell_sum,
            dimension
        }
    }

    /// The [Qing](http://benchmarkfcns.xyz/benchmarkfcns/qingfcn.html) function.
    pub fn qing(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "qing",
            implementation: scaled_implementations::qing,
            dimension
        }
    }

    /// The [Quartic](http://benchmarkfcns.xyz/benchmarkfcns/quarticfcn.html) function.
    pub fn quartic(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "quartic",
            implementation: scaled_implementations::quartic,
            dimension
        }
    }

    /// The [Ridge](http://benchmarkfcns.xyz/benchmarkfcns/ridgefcn.html) function.
    pub fn ridge(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "ridge",
            implementation: scaled_implementations::ridge,
            dimension
        }
    }

    /// The [Rosenbrock](http://benchmarkfcns.xyz/benchmarkfcns/rosenbrockfcn.html) function.
    pub fn rosenbrock(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "rosenbrock",
            implementation: scaled_implementations::rosenbrock,
            dimension
        }
    }

    /// The [Salomon](http://benchmarkfcns.xyz/benchmarkfcns/salomonfcn.html) function.
    pub fn salomon(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "salomon",
            implementation: scaled_implementations::salomon,
            dimension
        }
    }

    /// The [Schwefel 2.20](http://benchmarkfcns.xyz/benchmarkfcns/schwefel220fcn.html) function.
    pub fn schwefel_220(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "schwefel220",
            implementation: scaled_implementations::schwefel_220,
            dimension
        }
    }

    /// The [Schwefel 2.21](http://benchmarkfcns.xyz/benchmarkfcns/schwefel221fcn.html) function.
    pub fn schwefel_221(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "schwefel221",
            implementation: scaled_implementations::schwefel_221,
            dimension
        }
    }

    /// The [Schwefel 2.22](http://benchmarkfcns.xyz/benchmarkfcns/schwefel222fcn.html) function.
    pub fn schwefel_222(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "schwefel222",
            implementation: scaled_implementations::schwefel_222,
            dimension
        }
    }

    /// The [Schwefel 2.23](http://benchmarkfcns.xyz/benchmarkfcns/schwefel223fcn.html) function.
    pub fn schwefel_223(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "schwefel223",
            implementation: scaled_implementations::schwefel_223,
            dimension
        }
    }

    /// The [Schwefel](http://benchmarkfcns.xyz/benchmarkfcns/schwefelfcn.html) function.
    pub fn schwefel(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "schwefel",
            implementation: scaled_implementations::schwefel,
            dimension
        }
    }

    /// The [Shubert Nr. 3](http://benchmarkfcns.xyz/benchmarkfcns/shubert3fcn.html) function.
    pub fn shubert_n3(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "shubertN3",
            implementation: scaled_implementations::shubert_n3,
            dimension
        }
    }

    /// The [Shubert Nr. 4](http://benchmarkfcns.xyz/benchmarkfcns/shubert4fcn.html) function.
    pub fn shubert_n4(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "shubertN4",
            implementation: scaled_implementations::shubert_n4,
            dimension
        }
    }

    /// The [Shubert](http://benchmarkfcns.xyz/benchmarkfcns/shubertfcn.html) function.
    pub fn shubert(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "shubert",
            implementation: scaled_implementations::shubert,
            dimension
        }
    }

    /// The [Styblinski-Tank](http://benchmarkfcns.xyz/benchmarkfcns/styblinskitankfcn.html) function.
    pub fn styblinksi_tank(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "styblinskiTank",
            implementation: scaled_implementations::styblinksi_tank,
            dimension
        }
    }

    /// The [Sum Squares](http://benchmarkfcns.xyz/benchmarkfcns/sumsquaresfcn.html) function.
    pub fn sum_squares(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "sumSquares",
            implementation: scaled_implementations::sum_squares,
            dimension
        }
    }

    /// The [Xin-She Yang Nr. 1](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn1fcn.html) function.
    pub fn yang_n1(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "yangN1",
            implementation: scaled_implementations::yang_n1,
            dimension
        }
    }

    /// The [Xin-She Yang Nr. 2](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn2fcn.html) function.
    pub fn yang_n2(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "yangN2",
            implementation: scaled_implementations::yang_n2,
            dimension
        }
    }

    /// The [Xin-She Yang Nr. 3](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn3fcn.html) function.
    pub fn yang_n3(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "yangN3",
            implementation: scaled_implementations::yang_n3,
            dimension
        }
    }

    /// The [Xin-She Yang Nr. 4](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn4fcn.html) function.
    pub fn yang_n4(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "yangN4",
            implementation: scaled_implementations::yang_n4,
            dimension
        }
    }

    /// The [Zakharov](http://benchmarkfcns.xyz/benchmarkfcns/zakharov.html) function.
    pub fn zakharov(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "zakharov",
            implementation: scaled_implementations::zakharov,
            dimension
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
pub type Function = fn(&[f64]) -> f64;

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
    pub fn sphere(x: &[f64]) -> f64 {
        x.iter().map(|xi| xi * 5.12).map(|xi| xi * xi).sum()
    }

    /// Rastrinin function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    pub fn rastrigin(x: &[f64]) -> f64 {
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
    pub fn ackley(x: &[f64]) -> f64 {
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
    pub fn ackley_n4(x: &[f64]) -> f64 {
        let mut sum = 0.0;
        for i in x.len() {
            for j in x.len() - 1 {
                i = i * 35.0;
                j = j * 35.0;
                sum += (-0.2).exp() * (j.powi(2) + i.powi(2)).sqrt() + 3 * ((2 * j).cos() + (2 * i).sin());
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
    pub fn alpine_n1(x: &[f64]) -> f64 {
        x.iter()
            .map(|xi| (xi * 10).abs())
            .map(|xi| xi * xi.sin() + 0.1 * xi)
            .sum::<f64>()
    }

    /// Alpine Nr. 2 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -2.808^n at (7.917,...,7.917) (minimisation by negation)
    //TODO: Unit test!
    pub fn alpine_n2(x: &[f64]) -> f64 {
        - x.iter()
            .map(|xi| (xi * 10).abs())
            .map(|xi| xi.sqrt() * xi.sin())
            .product()
    }

    /// Brown function
    ///
    /// Scaled to [-1.0, 1.0] (from [-4,4])
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn brown(x: &[f64]) -> f64 {
        let mut sum = 0.0;
        for i in x.len() {
            for j in x.len() - 1 {
                i = i * 4.0;
                j = j * 4.0;
                sum += (j.powi(2)).powi(i.powi(2) + 1) + (i.powi(2)).powi(j.powi(2) + 1);
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
    pub fn exponential(x: &[f64]) -> f64 {
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
    pub fn griewank(x: &[f64]) -> f64 {
        let sum = x.iter()
            .map(|xi| xi * 600)
            .map(|xi| xi.powi(2) / 4000)
            .sum();
        let prod = x.iter()
            .enumerate()
            .map(|(i, &xi)| xi * 600)
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
    pub fn happy_cat(x: &[f64]) -> f64 {
        let n = x.len();
        let alpha = 1 / 8;
        let norm = x.iter()
            .map(|xi| xi * 2)
            .map(|xi| xi.powi(2))
            .sum::<f64>()
            .sqrt();

        let sum = x.iter()
            .map(|xi| xi * 2)
            .map(|xi| xi)
            .sum::<f64>();

        ((norm - n).powi(2)).powi(alpha) + (1 / n) * (0.5 * norm + sum) + 0.5
    }

    /// Periodic function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0.9 at (0,...,0)
    //TODO: Unit test!
    pub fn periodic(x: &[f64]) -> f64 {
        let sum = x
            .iter()
            .map(|xi| xi * 10)
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        let sine_sum = x
            .iter()
            .map(|xi| xi * 10)
            .map(|xi| (xi.sin()).powi(2))
            .sum::<f64>();

        1 + sine_sum - 0.1 * (sum).exp()
    }

    /// Powell Sum function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn powell_sum(x: &[f64]) -> f64 {
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
    pub fn qing(x: &[f64]) -> f64 {
        x.iter()
            .enumerate()
            .map(|(i, xi)| xi * 500)
            .map(|(i, xi)| (xi.powi(2) - i).powi(2) )
            .sum::<f64>()
    }

    /// Quartic function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 + rnd at (0,...,0)
    //TODO: Unit test!
    //TODO: Handle random number!
    pub fn quartic(x: &[f64]) -> f64 {
        let rnd = rand::thread_rng().gen::<f64>();
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
    pub fn ridge(x: &[f64]) -> f64 {
        let d = 1.0;
        let alpha = 0.5;
        let first = x[0];

        let sum = x[1..].iter()
            .enumerate()
            .map(|(i, xi)| xi * 5)
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
    pub fn rosenbrock(x: &[f64]) -> f64 {
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
    pub fn salomon(x: &[f64]) -> f64 {
        let sum = x.iter()
            .map(|xi| xi * 100)
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        1 - (2 * PI * sum.sqrt()).cos() + 0.1 * sum.sqrt()
    }

    /// Schwefel 2.20 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0)
    //TODO: Unit test!
    pub fn schwefel_220(x: &[f64]) -> f64 {
        x.iter()
            .map(|xi| xi * 100)
            .map(|xi| xi.abs())
            .sum()
    }

    /// Schwefel 2.21 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0)
    //TODO: Unit test!
    pub fn schwefel_221(x: &[f64]) -> f64 {
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
    pub fn schwefel_222(x: &[f64]) -> f64 {
        let sum = x.iter()
            .map(|xi| xi * 100)
            .map(|xi| xi.abs())
            .sum::<f64>();

        let prod = x.iter()
            .map(|xi| xi * 100)
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
    pub fn schwefel_223(x: &[f64]) -> f64 {
        x.iter()
            .map(|xi| xi * 10)
            .map(|xi| xi.powi(10))
            .sum::<f64>()

    }

    /// Schwefel function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (420.9687,...420.9687)
    //TODO: Unit test!
    pub fn schwefel(x: &[f64]) -> f64 {
        let sum = x.iter()
            .map(|xi| xi * 500)
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
    pub fn shubert_n3(x: &[f64]) -> f64 {
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
    pub fn shubert_n4(x: &[f64]) -> f64 {
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
    pub fn shubert(x: &[f64]) -> f64 {
        let mut prod = 1.0;
        for i in x.len() {
            let mut sum = 0.0;
            for j in 1..=5 {
                i = i * 10;
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
    pub fn styblinksi_tank(x: &[f64]) -> f64 {
        0.5 * x.iter()
            .map(|xi| xi * 5)
            .map(|xi| xi.powi(4) - 16 * xi.powi(2) + 5 * xi)
            .sum()
    }

    /// Sum Squares function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn sum_squares(x: &[f64]) -> f64 {
        x.iter()
            .enumerate()
            .map(|(i, xi)| xi * 10)
            .map(|(i, xi)| i * xi.powi(2))
            .sum()
    }

    /// Xin-She Yang Nr. 1 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    //TODO: Handle random number!
    pub fn yang_n1(x: &[f64]) -> f64 {
        let rnd = rand::thread_rng().gen::<f64>();
        x.iter()
            .enumerate()
            .map(|(i, xi)| xi * 5)
            .map(|(i, xi)| rnd * (xi.abs()).powi(i))
            .sum()
    }

    /// Xin-She Yang Nr. 2 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    //TODO: Unit test!
    pub fn yang_n2(x: &[f64]) -> f64 {
        let sum = x.iter()
          .map(|xi| xi * 2 * PI)
          .map(|xi| xi.abs())
          .sum::<f64>();

        let exp_sum = x.iter()
            .map(|xi| xi * 2 * PI)
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
    pub fn yang_n3(x: &[f64]) -> f64 {
        let beta = 15.0;
        let m = 5.0;

        let beta_sum = x.iter()
            .map(|xi| xi * 2 * PI)
            .map(|xi| (xi / beta).powi(2 * m))
            .sum::<f64>();

        let sum = x.iter()
            .map(|xi| xi * 2 * PI)
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        let prod = x.iter()
            .map(|xi| xi * 2 * PI)
            .map(|xi| (xi.cos()).powi(2))
            .sum::<f64>();

        (beta_sum).exp() - 2 * (sum).exp() * prod
    }

    /// Xin-She Yang Nr. 4 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -1 at (0,...,0)
    //TODO: Unit test!
    pub fn yang_n4(x: &[f64]) -> f64 {
        let inner_exp_sum = x.iter()
            .map(|xi| xi * 10)
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        let outer_exp_sum = x.iter()
            .map(|xi| xi * 10)
            .map(|xi| ((xi.abs()).sin()).powi(2))
            .sum::<f64>();

        let sum = x.iter()
            .map(|xi| xi * 10)
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
    pub fn zakharov(x: &[f64]) -> f64 {
        let i_sum = x.iter()
            .enumerate()
            .map(|(i, xi)| xi * 10)
            .map(|(i, xi)| 0.5 * i * xi)
            .sum::<f64>();

        let sum = x.iter()
            .map(|xi| xi * 10)
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        sum + (i_sum).powi(2) + (i_sum).powi(4)
    }
}
