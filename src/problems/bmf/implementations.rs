//! Collection of test functions from [benchmarkfcns.xyz](http://benchmarkfcns.xyz) without Quartic function and Xin-She Yang Nr. 1 function

use crate::problems::bmf::BenchmarkFunction;
use std::f64::consts::PI;

impl BenchmarkFunction {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn domain(&self) -> [f64; 2] {
        self.domain
    }

    pub fn known_optimum(&self) -> f64 {
        self.known_optimum
    }
}

impl BenchmarkFunction {
    /// The [Sphere](http://benchmarkfcns.xyz/benchmarkfcns/spherefcn.html) function.
    pub fn sphere(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "sphere",
            implementation: scaled_implementations::sphere,
            dimension,
            domain: [-5.12, 5.12],
            known_optimum: 0.0,
        }
    }

    /// The [Rastrigin](http://benchmarkfcns.xyz/benchmarkfcns/rastriginfcn.html) function.
    pub fn rastrigin(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "rstrigin",
            implementation: scaled_implementations::rastrigin,
            dimension,
            domain: [-5.12, 5.12],
            known_optimum: 0.0,
        }
    }

    /// The [Ackley](http://benchmarkfcns.xyz/benchmarkfcns/ackleyfcn.html) function.
    pub fn ackley(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "ackley",
            implementation: scaled_implementations::ackley,
            dimension,
            domain: [-32.0, 32.0],
            known_optimum: 0.0,
        }
    }

    /// The [AckleyN4](http://benchmarkfcns.xyz/benchmarkfcns/ackleyn4fcn.html) function.
    pub fn ackley_n4(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "ackleyN4",
            implementation: scaled_implementations::ackley_n4,
            dimension,
            domain: [-35.0, 35.0],
            known_optimum: -4.590101633799122,
        }
    }

    /// The [AlpineN1](http://benchmarkfcns.xyz/benchmarkfcns/alpinen1fcn.html) function.
    pub fn alpine_n1(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "alpineN1",
            implementation: scaled_implementations::alpine_n1,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: 0.0,
        }
    }

    /// The [AlpineN2](http://benchmarkfcns.xyz/benchmarkfcns/alpinen2fcn.html) function.
    pub fn alpine_n2(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "alpineN2",
            implementation: scaled_implementations::alpine_n2,
            dimension,
            domain: [0.0, 10.0],
            known_optimum: -(2.808_f64).powi(dimension as i32),
        }
    }

    /// The [Brown](http://benchmarkfcns.xyz/benchmarkfcns/brownfcn.html) function.
    pub fn brown(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "brown",
            implementation: scaled_implementations::brown,
            dimension,
            domain: [-1.0, 4.0],
            known_optimum: 0.0,
        }
    }

    /// The [Exponential](http://benchmarkfcns.xyz/benchmarkfcns/exponentialfcn.html) function.
    pub fn exponential(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "exponential",
            implementation: scaled_implementations::exponential,
            dimension,
            domain: [-1.0, 1.0],
            known_optimum: -1.0,
        }
    }

    /// The [Griewank](http://benchmarkfcns.xyz/benchmarkfcns/griewankfcn.html) function.
    pub fn griewank(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "griewank",
            implementation: scaled_implementations::griewank,
            dimension,
            domain: [-600.0, 600.0],
            known_optimum: 0.0,
        }
    }

    /// The [Happy Cat](http://benchmarkfcns.xyz/benchmarkfcns/happycatfcn.html) function.
    pub fn happy_cat(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "happyCat",
            implementation: scaled_implementations::happy_cat,
            dimension,
            domain: [-2.0, 2.0],
            known_optimum: 0.0,
        }
    }

    /// The [Periodic](http://benchmarkfcns.xyz/benchmarkfcns/periodicfcn.html) function.
    pub fn periodic(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "periodic",
            implementation: scaled_implementations::periodic,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: 0.9,
        }
    }

    /// The [Powell Sum](http://benchmarkfcns.xyz/benchmarkfcns/powellsumfcn.html) function.
    pub fn powell_sum(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "powellSum",
            implementation: scaled_implementations::powell_sum,
            dimension,
            domain: [-1.0, 1.0],
            known_optimum: 0.0,
        }
    }

    /// The [Qing](http://benchmarkfcns.xyz/benchmarkfcns/qingfcn.html) function.
    pub fn qing(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "qing",
            implementation: scaled_implementations::qing,
            dimension,
            domain: [-500.0, 500.0],
            known_optimum: 0.0,
        }
    }

    /// The [Ridge](http://benchmarkfcns.xyz/benchmarkfcns/ridgefcn.html) function.
    pub fn ridge(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "ridge",
            implementation: scaled_implementations::ridge,
            dimension,
            domain: [-5.0, 5.0],
            known_optimum: -5.0,
        }
    }

    /// The [Rosenbrock](http://benchmarkfcns.xyz/benchmarkfcns/rosenbrockfcn.html) function.
    pub fn rosenbrock(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "rosenbrock",
            implementation: scaled_implementations::rosenbrock,
            dimension,
            domain: [-5.0, 10.0],
            known_optimum: 0.0,
        }
    }

    /// The [Salomon](http://benchmarkfcns.xyz/benchmarkfcns/salomonfcn.html) function.
    pub fn salomon(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "salomon",
            implementation: scaled_implementations::salomon,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: 0.0,
        }
    }

    /// The [Schwefel 2.20](http://benchmarkfcns.xyz/benchmarkfcns/schwefel220fcn.html) function.
    pub fn schwefel_220(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "schwefel220",
            implementation: scaled_implementations::schwefel_220,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: 0.0,
        }
    }

    /// The [Schwefel 2.21](http://benchmarkfcns.xyz/benchmarkfcns/schwefel221fcn.html) function.
    pub fn schwefel_221(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "schwefel221",
            implementation: scaled_implementations::schwefel_221,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: 0.0,
        }
    }

    /// The [Schwefel 2.22](http://benchmarkfcns.xyz/benchmarkfcns/schwefel222fcn.html) function.
    pub fn schwefel_222(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "schwefel222",
            implementation: scaled_implementations::schwefel_222,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: 0.0,
        }
    }

    /// The [Schwefel 2.23](http://benchmarkfcns.xyz/benchmarkfcns/schwefel223fcn.html) function.
    pub fn schwefel_223(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "schwefel223",
            implementation: scaled_implementations::schwefel_223,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: 0.0,
        }
    }

    /// The [Schwefel](http://benchmarkfcns.xyz/benchmarkfcns/schwefelfcn.html) function.
    pub fn schwefel(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "schwefel",
            implementation: scaled_implementations::schwefel,
            dimension,
            domain: [-500.0, 500.0],
            known_optimum: 0.0,
        }
    }

    /// The [Shubert Nr. 3](http://benchmarkfcns.xyz/benchmarkfcns/shubert3fcn.html) function.
    pub fn shubert_n3(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "shubertN3",
            implementation: scaled_implementations::shubert_n3,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: -24.062499,
        }
    }

    /// The [Shubert Nr. 4](http://benchmarkfcns.xyz/benchmarkfcns/shubert4fcn.html) function.
    pub fn shubert_n4(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "shubertN4",
            implementation: scaled_implementations::shubert_n4,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: -25.740858,
        }
    }

    /// The [Shubert](http://benchmarkfcns.xyz/benchmarkfcns/shubertfcn.html) function.
    pub fn shubert(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "shubert",
            implementation: scaled_implementations::shubert,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: -186.7309,
        }
    }

    /// The [Styblinski-Tang](http://benchmarkfcns.xyz/benchmarkfcns/styblinskitankfcn.html) function.
    pub fn styblinski_tang(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "styblinskiTang",
            implementation: scaled_implementations::styblinski_tang,
            dimension,
            domain: [-5.0, 5.0],
            known_optimum: -39.1662 * dimension as f64,
        }
    }

    /// The [Sum Squares](http://benchmarkfcns.xyz/benchmarkfcns/sumsquaresfcn.html) function.
    pub fn sum_squares(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "sumSquares",
            implementation: scaled_implementations::sum_squares,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: 0.0,
        }
    }

    /// The [Xin-She Yang Nr. 2](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn2fcn.html) function.
    pub fn yang_n2(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "yangN2",
            implementation: scaled_implementations::yang_n2,
            dimension,
            domain: [-2.0 * PI, 2.0 * PI],
            known_optimum: 0.0,
        }
    }

    /// The [Xin-She Yang Nr. 3](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn3fcn.html) function.
    pub fn yang_n3(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "yangN3",
            implementation: scaled_implementations::yang_n3,
            dimension,
            domain: [-2.0 * PI, 2.0 * PI],
            known_optimum: -1.0,
        }
    }

    /// The [Xin-She Yang Nr. 4](http://benchmarkfcns.xyz/benchmarkfcns/xinsheyangn4fcn.html) function.
    pub fn yang_n4(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "yangN4",
            implementation: scaled_implementations::yang_n4,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: -1.0,
        }
    }

    /// The [Zakharov](http://benchmarkfcns.xyz/benchmarkfcns/zakharov.html) function.
    pub fn zakharov(dimension: usize) -> Self {
        BenchmarkFunction {
            name: "zakharov",
            implementation: scaled_implementations::zakharov,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: 0.0,
        }
    }

    /// The [Ackley Nr. 2](http://benchmarkfcns.xyz/benchmarkfcns/ackleyn2fcn.html) function.
    pub fn ackley_n2(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "ackleyN2",
            implementation: scaled_implementations::ackley_n2,
            dimension,
            domain: [-32.0, 32.0],
            known_optimum: -200.0,
        }
    }

    /// The [Ackley Nr. 3](http://benchmarkfcns.xyz/benchmarkfcns/ackleyn3fcn.html) function.
    pub fn ackley_n3(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "ackleyN3",
            implementation: scaled_implementations::ackley_n3,
            dimension,
            domain: [-32.0, 32.0],
            known_optimum: -219.1418,
        }
    }

    /// The [Adjiman](http://benchmarkfcns.xyz/benchmarkfcns/adjimanfcn.html) function.
    pub fn adjiman(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "adjiman",
            implementation: scaled_implementations::adjiman,
            dimension,
            //TODO: find better solution when domains different for x and y
            domain: [-2.0, 2.0],
            known_optimum: -2.02181,
        }
    }

    /// The [Bartels Conn](http://benchmarkfcns.xyz/benchmarkfcns/bartelsconnfcn.html) function.
    pub fn bartels_conn(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "bartelsConn",
            implementation: scaled_implementations::bartels_conn,
            dimension,
            domain: [-500.0, 500.0],
            known_optimum: 1.0,
        }
    }

    /// The [Beale](http://benchmarkfcns.xyz/benchmarkfcns/bealefcn.html) function.
    pub fn beale(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "beale",
            implementation: scaled_implementations::beale,
            dimension,
            domain: [-4.5, 4.5],
            known_optimum: 0.0,
        }
    }

    /// The [Bird](http://benchmarkfcns.xyz/benchmarkfcns/birdfcn.html) function.
    pub fn bird(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "bird",
            implementation: scaled_implementations::bird,
            dimension,
            domain: [-2.0 * PI, 2.0 * PI],
            known_optimum: -106.764537,
        }
    }

    /// The [Bohachevsky Nr. 1](http://benchmarkfcns.xyz/benchmarkfcns/bohachevskyn1fcn.html) function.
    pub fn bohachevsky_n1(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "bohachevskyN1",
            implementation: scaled_implementations::bohachevsky_n1,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: 0.0,
        }
    }

    /// The [Bohachevsky Nr. 2](http://benchmarkfcns.xyz/benchmarkfcns/bohachevskyn2fcn.html) function.
    pub fn bohachevsky_n2(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "bohachevskyN2",
            implementation: scaled_implementations::bohachevsky_n2,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: 0.0,
        }
    }

    /// The [Booth](http://benchmarkfcns.xyz/benchmarkfcns/boothfcn.html) function.
    pub fn booth(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "booth",
            implementation: scaled_implementations::booth,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: 0.0,
        }
    }

    /// The [Brent](http://benchmarkfcns.xyz/benchmarkfcns/brentfcn.html) function.
    pub fn brent(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "brent",
            implementation: scaled_implementations::brent,
            dimension,
            domain: [-20.0, 0.0],
            known_optimum: (-200.0_f64).exp(),
        }
    }

    /// The [Bukin Nr. 6](http://benchmarkfcns.xyz/benchmarkfcns/bukinn6fcn.html) function.
    pub fn bukin_n6(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "bukinN6",
            implementation: scaled_implementations::bukin_n6,
            dimension,
            domain: [-15.0, -5.0],
            known_optimum: 0.0,
        }
    }

    /// The [Cross-in-Tray](http://benchmarkfcns.xyz/benchmarkfcns/crossintrayfcn.html) function.
    pub fn cross_in_tray(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "crossInTray",
            implementation: scaled_implementations::cross_in_tray,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: -2.06261218,
        }
    }

    /// The [Deckkers-Aarts](http://benchmarkfcns.xyz/benchmarkfcns/deckkersaartsfcn.html) function.
    pub fn deckkers_aarts(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "deckkersAarts",
            implementation: scaled_implementations::deckkers_aarts,
            dimension,
            domain: [-20.0, 20.0],
            known_optimum: -24771.09375,
        }
    }

    /// The [Drop-Wave](http://benchmarkfcns.xyz/benchmarkfcns/dropwavefcn.html) function.
    pub fn drop_wave(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "dropWave",
            implementation: scaled_implementations::drop_wave,
            dimension,
            domain: [-5.2, 5.2],
            known_optimum: -1.0,
        }
    }

    /// The [Easom](http://benchmarkfcns.xyz/benchmarkfcns/easomfcn.html) function.
    pub fn easom(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "easom",
            implementation: scaled_implementations::easom,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: -1.0,
        }
    }

    /// The [Egg Crate](http://benchmarkfcns.xyz/benchmarkfcns/eggcratefcn.html) function.
    pub fn egg_crate(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "eggCrate",
            implementation: scaled_implementations::egg_crate,
            dimension,
            domain: [-5.0, 5.0],
            known_optimum: 0.0,
        }
    }

    /// The [Goldstein-Price](http://benchmarkfcns.xyz/benchmarkfcns/goldsteinpricefcn.html) function.
    pub fn goldstein_price(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "goldsteinPrice",
            implementation: scaled_implementations::goldstein_price,
            dimension,
            domain: [-2.0, 2.0],
            known_optimum: 3.0,
        }
    }

    /// The [Gramacy & Lee](http://benchmarkfcns.xyz/benchmarkfcns/gramacyleefcn.html) function.
    pub fn gramacy_lee(dimension: usize) -> Self {
        assert_eq!(dimension, 1);
        BenchmarkFunction {
            name: "gramacyLee",
            implementation: scaled_implementations::gramacy_lee,
            dimension,
            domain: [0.5, 2.5],
            known_optimum: -0.869011134989500,
        }
    }

    /// The [Himmelblau](http://benchmarkfcns.xyz/benchmarkfcns/himmelblaufcn.html) function.
    pub fn himmelblau(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "himmelblau",
            implementation: scaled_implementations::himmelblau,
            dimension,
            domain: [-6.0, 6.0],
            known_optimum: 0.0,
        }
    }

    /// The [Holder-Table](http://benchmarkfcns.xyz/benchmarkfcns/holdertablefcn.html) function.
    pub fn holder_table(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "holderTable",
            implementation: scaled_implementations::holder_table,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: -19.2085,
        }
    }

    /// The [Keane](http://benchmarkfcns.xyz/benchmarkfcns/kealefcn.html) function.
    pub fn keane(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "keane",
            implementation: scaled_implementations::keane,
            dimension,
            domain: [0.0, 10.0],
            known_optimum: -0.673667521146855,
        }
    }

    /// The [Leon](http://benchmarkfcns.xyz/benchmarkfcns/leonfcn.html) function.
    pub fn leon(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "leon",
            implementation: scaled_implementations::leon,
            dimension,
            domain: [0.0, 10.0],
            known_optimum: 0.0,
        }
    }

    /// The [Levi Nr. 13](http://benchmarkfcns.xyz/benchmarkfcns/levin13fcn.html) function.
    pub fn levi_n13(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "leviN13",
            implementation: scaled_implementations::levi_n13,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: 0.0,
        }
    }

    /// The [Matyas](http://benchmarkfcns.xyz/benchmarkfcns/matyasfcn.html) function.
    pub fn matyas(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "matyas",
            implementation: scaled_implementations::matyas,
            dimension,
            domain: [-10.0, 10.0],
            known_optimum: 0.0,
        }
    }

    /// The [McCormick](http://benchmarkfcns.xyz/benchmarkfcns/mccormickfcn.html) function.
    pub fn mccormick(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "mccormick",
            implementation: scaled_implementations::mccormick,
            dimension,
            domain: [-1.5, 4.0],
            known_optimum: -1.9133,
        }
    }

    /// The [Schaffer Nr. 1](http://benchmarkfcns.xyz/benchmarkfcns/schaffern1fcn.html) function.
    pub fn schaffer_n1(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "schafferN1",
            implementation: scaled_implementations::schaffer_n1,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: 0.0,
        }
    }

    /// The [Schaffer Nr. 2](http://benchmarkfcns.xyz/benchmarkfcns/schaffern2fcn.html) function.
    pub fn schaffer_n2(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "schafferN2",
            implementation: scaled_implementations::schaffer_n2,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: 0.0,
        }
    }

    /// The [Schaffer Nr. 3](http://benchmarkfcns.xyz/benchmarkfcns/schaffern3fcn.html) function.
    pub fn schaffer_n3(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "schafferN3",
            implementation: scaled_implementations::schaffer_n3,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: 0.00156685,
        }
    }

    /// The [Schaffer Nr. 4](http://benchmarkfcns.xyz/benchmarkfcns/schaffern4fcn.html) function.
    pub fn schaffer_n4(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "schafferN4",
            implementation: scaled_implementations::schaffer_n4,
            dimension,
            domain: [-100.0, 100.0],
            known_optimum: 0.292579,
        }
    }

    /// The [Three-Hump Camel](http://benchmarkfcns.xyz/benchmarkfcns/threehumpcamelfcn.html) function.
    pub fn three_hump_camel(dimension: usize) -> Self {
        assert_eq!(dimension, 2);
        BenchmarkFunction {
            name: "threeHumpCamel",
            implementation: scaled_implementations::three_hump_camel,
            dimension,
            domain: [-5.0, 5.0],
            known_optimum: 0.0,
        }
    }

    /// The [Wolfe](http://benchmarkfcns.xyz/benchmarkfcns/wolfefcn.html) function.
    pub fn wolfe(dimension: usize) -> Self {
        assert_eq!(dimension, 3);
        BenchmarkFunction {
            name: "wolfe",
            implementation: scaled_implementations::wolfe,
            dimension,
            domain: [0.0, 2.0],
            known_optimum: 0.0,
        }
    }
}

/// The benchmark functions scaled to [-1.0, 1.0].
pub mod scaled_implementations {
    use std::f64::consts::PI;
    //use std::intrinsics::sqrtf64;

    // general scaling from value within [-1,1] to input domain [min,max] of function
    fn scale_domain(value: &f64, min: f64, max: f64) -> f64 {
        (value + 1.0) / 2.0 * (max - min) + min
    }

    /// Sphere function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0), here on input domain [-5.12,5.12]
    pub fn sphere(x: &[f64]) -> f64 {
        x.iter()
            .map(|xi| scale_domain(xi, -5.12, 5.12))
            .map(|xi| xi * xi)
            .sum()
    }

    /// Rastrinin function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0), here on input domain [-5.12,5.12]
    pub fn rastrigin(x: &[f64]) -> f64 {
        let n = x.len() as f64;
        10.0 * n
            + x.iter()
                .map(|xi| scale_domain(xi, -5.12, 5.12))
                .map(|xi| xi * xi - 10.0 * (2.0 * PI * xi).cos())
                .sum::<f64>()
    }

    /// Ackley function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0), here on input domain [-32,32]
    pub fn ackley(x: &[f64]) -> f64 {
        let a = 20.0;
        let b = 0.2;
        let c = 2.0 * PI;

        let n_inverse = 1.0 / x.len() as f64;
        let squared_sum = x
            .iter()
            .map(|xi| scale_domain(xi, -32.0, 32.0))
            .map(|xi| xi.powi(2))
            .sum::<f64>();
        let cosine_sum = x
            .iter()
            .map(|xi| scale_domain(xi, -32.0, 32.0))
            .map(|xi| (c * xi).cos())
            .sum::<f64>();

        a + (1.0_f64).exp() + (-a) * ((-b) * (n_inverse * squared_sum).sqrt()).exp()
            - (n_inverse * cosine_sum).exp()
    }

    /// Ackley Nr. 4 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: on 2-dimensional space -4.590101633799122 at (−1.51,−0.755), here on input domain [-35,35]
    pub fn ackley_n4(x: &[f64]) -> f64 {
        let mut sum = 0.0;
        for i in 1..=(x.len() - 1) {
            sum += (-0.2_f64).exp()
                * ((scale_domain(&x[i - 1], -35.0, 35.0)).powi(2)
                    + (scale_domain(&x[i], -35.0, 35.0)).powi(2))
                .sqrt()
                + 3.0
                    * ((2.0 * (scale_domain(&x[i - 1], -35.0, 35.0))).cos()
                        + (2.0 * (scale_domain(&x[i], -35.0, 35.0))).sin());
        }
        sum
    }

    /// Alpine Nr. 1 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0), here on input domain [-10,10]
    pub fn alpine_n1(x: &[f64]) -> f64 {
        x.iter()
            .map(|xi| scale_domain(xi, -10.0, 10.0))
            .map(|xi| (xi * xi.sin() + 0.1 * xi).abs())
            .sum::<f64>()
    }

    /// Alpine Nr. 2 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -2.808^n at (7.917,...,7.917) (minimisation by negation), here on input domain [0,10]
    pub fn alpine_n2(x: &[f64]) -> f64 {
        -x.iter()
            .map(|xi| scale_domain(xi, 0.0, 10.0))
            .map(|xi| xi.sqrt() * xi.sin())
            .product::<f64>()
    }

    /// Brown function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0), here on input domain [-1,4]
    pub fn brown(x: &[f64]) -> f64 {
        let mut sum = 0.0;
        for i in 1..=(x.len() - 1) {
            sum += ((scale_domain(&x[i - 1], -1.0, 4.0)).powi(2))
                .powi(((scale_domain(&x[i], -1.0, 4.0)).powi(2) + 1.0) as i32)
                + ((scale_domain(&x[i], -1.0, 4.0)).powi(2))
                    .powi(((scale_domain(&x[i - 1], -1.0, 4.0)).powi(2) + 1.0) as i32);
        }
        sum
    }

    /// Exponential function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: - 1.0 at (0,...,0)
    pub fn exponential(x: &[f64]) -> f64 {
        let sum = x.iter().map(|xi| xi.powi(2)).sum::<f64>();
        -((-0.5) * sum).exp()
    }

    /// Griewank function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0), here on input domain [-600,600]
    pub fn griewank(x: &[f64]) -> f64 {
        let sum = x
            .iter()
            .map(|xi| scale_domain(xi, -600.0, 600.0))
            .map(|xi| xi.powi(2) / 4000.0)
            .sum::<f64>();

        let prod = x
            .iter()
            .map(|&xi| scale_domain(&xi, -600.0, 600.0))
            .enumerate()
            .map(|(i, xi)| (xi / ((i as f64) + 1.0)).cos())
            .product::<f64>();

        1.0 + sum - prod
    }

    /// Happy Cat function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (-1,...,-1), here on input domain [-2,2]
    pub fn happy_cat(x: &[f64]) -> f64 {
        let n = x.len() as f64;
        let alpha = 1.0 / 8.0;
        let norm = x
            .iter()
            .map(|xi| scale_domain(xi, -2.0, 2.0))
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        let sum = x.iter().map(|xi| scale_domain(xi, -2.0, 2.0)).sum::<f64>();

        ((norm - n).powi(2)).powf(alpha) + (1.0 / n) * (0.5 * norm + sum) + 0.5
    }

    /// Periodic function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0.9 at (0,...,0), here on input domain [-10,10]
    /// On http://benchmarkfcns.xyz/benchmarkfcns/periodicfcn.html, there is a typo in the mathematical definition!
    pub fn periodic(x: &[f64]) -> f64 {
        let sum = x
            .iter()
            .map(|xi| scale_domain(xi, -10.0, 10.0))
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        let sine_sum = x
            .iter()
            .map(|xi| scale_domain(xi, -10.0, 10.0))
            .map(|xi| (xi.sin()).powi(2))
            .sum::<f64>();

        1.0 + sine_sum - 0.1 * (-sum).exp()
    }

    /// Powell Sum function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0)
    pub fn powell_sum(x: &[f64]) -> f64 {
        x.iter()
            .enumerate()
            .map(|(i, xi)| (xi.abs()).powi((i + 1) as i32))
            .sum::<f64>()
    }

    /// Qing function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (+-(i).sqrt(),...,+-(i).sqrt()), here on input domain [-500,500]
    pub fn qing(x: &[f64]) -> f64 {
        x.iter()
            .map(|xi| scale_domain(xi, -500.0, 500.0))
            .enumerate()
            .map(|(i, xi)| (xi.powi(2) - (i as f64 + 1.0)).powi(2))
            .sum::<f64>()
    }

    /// Ridge function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -5 at (-5,0,...,0), here on input domain [-5,5]
    pub fn ridge(x: &[f64]) -> f64 {
        let d = 1.0;
        let alpha = 0.5;
        let first = scale_domain(&x[0], -5.0, 5.0);

        let sum = x[1..]
            .iter()
            .map(|xi| scale_domain(xi, -5.0, 5.0))
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        first + d * sum.powf(alpha)
    }

    /// Rosenbrock function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (1,...1), here on input domain [-5,10]
    pub fn rosenbrock(x: &[f64]) -> f64 {
        let a = 1.0;
        let b = 100.0;
        let mut sum = 0.0;

        for i in 1..=(x.len() - 1) {
            sum += b
                * ((scale_domain(&x[i], -5.0, 10.0))
                    - (scale_domain(&x[i - 1], -5.0, 10.0)).powi(2))
                .powi(2)
                + (a - (scale_domain(&x[i - 1], -5.0, 10.0))).powi(2);
        }
        sum
    }

    /// Salomon function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0), here on input domain [-100,100]
    pub fn salomon(x: &[f64]) -> f64 {
        let sum = x
            .iter()
            .map(|xi| scale_domain(xi, -100.0, 100.0))
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        1.0 - (2.0 * PI * sum.sqrt()).cos() + 0.1 * sum.sqrt()
    }

    /// Schwefel 2.20 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0), here on input domain [-100,100]
    pub fn schwefel_220(x: &[f64]) -> f64 {
        x.iter()
            .map(|xi| scale_domain(xi, -100.0, 100.0))
            .map(|xi| xi.abs())
            .sum()
    }

    /// Schwefel 2.21 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0), here on input domain [-100,100]
    pub fn schwefel_221(x: &[f64]) -> f64 {
        let max_elem = x
            .iter()
            .map(|xi| (scale_domain(xi, -100.0, 100.0)).abs())
            .fold(f64::NEG_INFINITY, f64::max);
        //.max_by(|a, b| a.total_cmp(b));

        max_elem
    }

    /// Schwefel 2.22 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0), here on input domain [-100,100]
    pub fn schwefel_222(x: &[f64]) -> f64 {
        let sum = x
            .iter()
            .map(|xi| scale_domain(xi, -100.0, 100.0))
            .map(|xi| xi.abs())
            .sum::<f64>();

        let prod = x
            .iter()
            .map(|xi| xi * 100.0)
            .map(|xi| xi.abs())
            .product::<f64>();

        sum + prod
    }

    /// Schwefel 2.23 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...0), here on input domain [-10,10]
    pub fn schwefel_223(x: &[f64]) -> f64 {
        x.iter()
            .map(|xi| scale_domain(xi, -10.0, 10.0))
            .map(|xi| xi.powi(10))
            .sum::<f64>()
    }

    /// Schwefel function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (420.9687,...420.9687), here on input domain [-500,500]
    pub fn schwefel(x: &[f64]) -> f64 {
        let sum = x
            .iter()
            .map(|xi| scale_domain(xi, -500.0, 500.0))
            .map(|xi| xi * ((xi.abs()).sqrt()).sin())
            .sum::<f64>();

        418.9829 * (x.len() as f64) - sum
    }

    /// Shubert Nr. 3 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: ~ -24.062499 at some points (-6.774576,-6.774576), ..., (5.791794,5.791794) here on input domain [-10,10]
    pub fn shubert_n3(x: &[f64]) -> f64 {
        let mut sum = 0.0;
        for i in 1..=(x.len()) {
            for j in 1..=5 {
                sum += j as f64
                    * ((j as f64 + 1.0) * (scale_domain(&x[i - 1], -10.0, 10.0)) + j as f64).sin();
            }
        }
        -sum
    }

    /// Shubert Nr. 4 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: ~ -25.740858 at (-6.774576 + PI,-6.774576 + PI), here on input domain [-10,10]
    pub fn shubert_n4(x: &[f64]) -> f64 {
        let mut sum = 0.0;
        for i in 1..=(x.len()) {
            for j in 1..=5 {
                sum += j as f64
                    * ((j as f64 + 1.0) * (scale_domain(&x[i - 1], -10.0, 10.0)) + j as f64).cos();
            }
        }
        sum
    }

    /// Shubert function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: ~ -186.7309 (18 optima), e.g. (−7.0835, 4.8580), here on input domain [-10,10]
    pub fn shubert(x: &[f64]) -> f64 {
        let mut prod = 1.0;
        for i in 1..=(x.len()) {
            let mut sum = 0.0;
            for j in 1..=5 {
                sum += ((j as f64 + 1.0) * (scale_domain(&x[i - 1], -10.0, 10.0)) + j as f64).cos();
            }
            prod *= sum
        }
        prod
    }

    /// Styblinski-Tang function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -39.16599 * n at (-2.903534,...,-2.903534), here on input domain [-5,5]
    pub fn styblinski_tang(x: &[f64]) -> f64 {
        let sum = x
            .iter()
            .map(|xi| scale_domain(xi, -5.0, 5.0))
            .map(|xi| xi.powi(4) - 16.0 * xi.powi(2) + 5.0 * xi)
            .sum::<f64>();

        0.5 * sum
    }

    /// Sum Squares function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0), here on input domain [-10,10]
    pub fn sum_squares(x: &[f64]) -> f64 {
        x.iter()
            .map(|xi| scale_domain(xi, -10.0, 10.0))
            .enumerate()
            .map(|(i, xi)| i as f64 * xi.powi(2))
            .sum::<f64>()
    }

    /// Xin-She Yang Nr. 2 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0), here on input domain [-2PI,2PI]
    pub fn yang_n2(x: &[f64]) -> f64 {
        let sum = x
            .iter()
            .map(|xi| scale_domain(xi, -2.0 * PI, 2.0 * PI))
            .map(|xi| xi.abs())
            .sum::<f64>();

        let exp_sum = x
            .iter()
            .map(|xi| scale_domain(xi, -2.0 * PI, 2.0 * PI))
            .map(|xi| (xi.powi(2)).sin())
            .sum::<f64>();

        sum * (-exp_sum).exp()
    }

    /// Xin-She Yang Nr. 3 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -1 at (0,...,0), here on input domain [-2PI,2PI]
    pub fn yang_n3(x: &[f64]) -> f64 {
        let beta = 15.0;
        let m = 5.0;

        let beta_sum = x
            .iter()
            .map(|xi| scale_domain(xi, -2.0 * PI, 2.0 * PI))
            .map(|xi| (xi / beta).powf(2.0 * m))
            .sum::<f64>();

        let sum = x
            .iter()
            .map(|xi| scale_domain(xi, -2.0 * PI, 2.0 * PI))
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        let prod = x
            .iter()
            .map(|xi| scale_domain(xi, -2.0 * PI, 2.0 * PI))
            .map(|xi| (xi.cos()).powi(2))
            .product::<f64>();

        (-beta_sum).exp() - 2.0 * (-sum).exp() * prod
    }

    /// Xin-She Yang Nr. 4 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -1 at (0,...,0), here on input domain [-10,10]
    pub fn yang_n4(x: &[f64]) -> f64 {
        let inner_exp_sum = x
            .iter()
            .map(|xi| scale_domain(xi, -10.0, 10.0))
            .map(|xi| xi.powi(2))
            .sum::<f64>();

        let outer_exp_sum = x
            .iter()
            .map(|xi| scale_domain(xi, -10.0, 10.0))
            .map(|xi| ((xi.abs()).sin()).powi(2))
            .sum::<f64>();

        let sum = x
            .iter()
            .map(|xi| scale_domain(xi, -10.0, 10.0))
            .map(|xi| (xi.sin()).powi(2))
            .sum::<f64>();

        (sum - (-inner_exp_sum).exp()) * (-outer_exp_sum).exp()
    }

    /// Zakharov function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,...,0), here on input domain [-10,10]
    pub fn zakharov(x: &[f64]) -> f64 {
        let i_sum = x
            .iter()
            .map(|xi| scale_domain(xi, -10.0, 10.0))
            .enumerate()
            .map(|(i, xi)| 0.5 * i as f64 * xi)
            .sum::<f64>();

        let sum = x
            .iter()
            .map(|xi| scale_domain(xi, -10.0, 10.0))
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
    pub fn ackley_n2(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -32.0, 32.0);
        let b = scale_domain(&x[1], -32.0, 32.0);
        -200.0 * (-0.2 * (a.powi(2) + b.powi(2)).sqrt()).exp()
    }

    /// Ackley Nr. 3 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: −195.629028238419 at (±0.682584587365898,−0.36075325513719), here on input domain [-32,32]
    /// Defined only on 2-dimensional space.
    pub fn ackley_n3(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -32.0, 32.0);
        let b = scale_domain(&x[1], -32.0, 32.0);
        -200.0 * (-0.2 * (a.powi(2) + b.powi(2)).sqrt()).exp()
            + 5.0 * ((3.0 * a).cos() + (3.0 * b.sin())).exp()
    }

    /// Adjiman function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: −2.02181 at (2, 0.10578), here on input domain [-2,2] for x and [-1,1] for y
    /// Defined only on 2-dimensional space.
    pub fn adjiman(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -2.0, 2.0);
        let b = scale_domain(&x[1], -1.0, 1.0);
        a.cos() * b.sin() - (a / (b.powi(2) + 1.0))
    }

    /// Bartels Conn function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 1 at (0,0), here on input domain [-500,500]
    /// Defined only on 2-dimensional space.
    pub fn bartels_conn(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -500.0, 500.0);
        let b = scale_domain(&x[1], -500.0, 500.0);
        (a.powi(2) + b.powi(2) + (a * b)).abs() + (a.sin()).abs() + (b.cos()).abs()
    }

    /// Beale function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (3,0.5), here on input domain [-4.5,4.5]
    /// Defined only on 2-dimensional space.
    pub fn beale(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -4.5, 4.5);
        let b = scale_domain(&x[1], -4.5, 4.5);
        (1.5 - a + (a * b)).powi(2)
            + (2.25 - a + (a * b.powi(2))).powi(2)
            + (2.625 - a + (a * b.powi(3))).powi(2)
    }

    /// Bird function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: −106.764537 at (4.70104,3.15294) and (−1.58214,−3.13024), here on input domain [-2 * Pi,2 * Pi]
    /// Defined only on 2-dimensional space.
    pub fn bird(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -2.0 * PI, 2.0 * PI);
        let b = scale_domain(&x[1], -2.0 * PI, 2.0 * PI);
        a.sin() * ((1.0 - b.cos()).powi(2)).exp()
            + b.cos() * ((1.0 - a.sin()).powi(2)).exp()
            + (a - b).powi(2)
    }

    /// Bohachevsky Nr. 1 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,0), here on input domain [-100,100]
    /// Defined only on 2-dimensional space.
    pub fn bohachevsky_n1(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -100.0, 100.0);
        let b = scale_domain(&x[1], -100.0, 100.0);
        a.powi(2) + (2.0 * b.powi(2)) - (0.3 * (3.0 * PI * a).cos()) - (0.4 * (4.0 * PI * b).cos())
            + 0.7
    }

    /// Bohachevsky Nr. 2 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,0), here on input domain [-100,100]
    /// Defined only on 2-dimensional space.
    pub fn bohachevsky_n2(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -100.0, 100.0);
        let b = scale_domain(&x[1], -100.0, 100.0);
        a.powi(2) + (2.0 * b.powi(2)) - (0.3 * (3.0 * PI * a).cos() * (4.0 * PI * b).cos()) + 0.3
    }

    /// Booth function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (1,3), here on input domain [-10,10]
    /// Defined only on 2-dimensional space.
    pub fn booth(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -10.0, 10.0);
        let b = scale_domain(&x[1], -10.0, 10.0);
        (a + (2.0 * b) - 7.0).powi(2) + ((2.0 * a) + b - 5.0).powi(2)
    }

    /// Brent function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: e^(-200) at (-10,-10), here on input domain [-20,0]
    /// Defined only on 2-dimensional space.
    pub fn brent(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -20.0, 0.0);
        let b = scale_domain(&x[1], -20.0, 0.0);
        (a + 10.0).powi(2) + (b + 10.0).powi(2) + (-a.powi(2) - b.powi(2)).exp()
    }

    /// Bukin Nr. 6 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (-10,1), here on input domain [-15,-5] for x and [-3,3] for y
    /// Defined only on 2-dimensional space.
    pub fn bukin_n6(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -15.0, -5.0);
        let b = scale_domain(&x[1], -3.0, 3.0);
        100.0 * ((b - 0.01 * a.powi(2)).abs()).sqrt() + 0.01 * (a + 10.0).abs()
    }

    /// Cross-in-Tray function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: - 2.06261218 at (±1.349406685353340,±1.349406608602084), here on input domain [-10,10]
    /// Defined only on 2-dimensional space.
    pub fn cross_in_tray(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -10.0, 10.0);
        let b = scale_domain(&x[1], -10.0, 10.0);
        -0.0001
            * ((a.sin() * b.sin() * ((100.0 - ((a.powi(2) + b.powi(2)).sqrt() / PI)).abs()).exp())
                .abs()
                + 1.0)
                .powf(0.1)
    }

    /// Deckkers-Aarts function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: −24771.09375 at (0,±15), here on input domain [-20,20]
    /// Defined only on 2-dimensional space.
    pub fn deckkers_aarts(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -20.0, 20.0);
        let b = scale_domain(&x[1], -20.0, 20.0);
        (10.0_f64).powi(5) * a.powi(2) + b.powi(2) - (a.powi(2) + b.powi(2)).powi(2)
            + (10.0_f64).powi(-5) * (a.powi(2) + b.powi(2)).powi(4)
    }

    /// Drop-Wave function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -1 at (0,0), here on input domain [-5.2,5.2]
    /// Defined only on 2-dimensional space.
    pub fn drop_wave(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -5.2, 5.2);
        let b = scale_domain(&x[1], -5.2, 5.2);
        -(1.0 + (12.0 * (a.powi(2) + b.powi(2)).sqrt()).cos())
            / (0.5 * (a.powi(2) + b.powi(2)) + 2.0)
    }

    /// Easom function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: -1 at (π,π), here on input domain [-100,100]
    /// Defined only on 2-dimensional space.
    pub fn easom(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -100.0, 100.0);
        let b = scale_domain(&x[1], -100.0, 100.0);
        -a.cos() * b.cos() * (-(a - PI).powi(2) - (b - PI).powi(2)).exp()
    }

    /// Egg Crate function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,0), here on input domain [-5,5]
    /// Defined only on 2-dimensional space.
    pub fn egg_crate(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -5.0, 5.0);
        let b = scale_domain(&x[1], -5.0, 5.0);
        a.powi(2) + b.powi(2) + 25.0 * ((a.sin()).powi(2) + (b.sin()).powi(2))
    }

    /// Goldstein-Price function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 3 at (0,-1), here on input domain [-2,2]
    /// Defined only on 2-dimensional space.
    pub fn goldstein_price(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -2.0, 2.0);
        let b = scale_domain(&x[1], -2.0, 2.0);
        (1.0 + (a + b + 1.0).powi(2)
            * (19.0 - 14.0 * a + 3.0 * a.powi(2) - 14.0 * b + 6.0 * a * b + 3.0 * b.powi(2)))
            * (30.0
                + (2.0 * a - 3.0 * b).powi(2)
                    * (18.0 - 32.0 * a + 12.0 * a.powi(2) + 48.0 * b - 36.0 * a * b
                        + 27.0 * b.powi(2)))
    }

    /// Gramacy & Lee function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: −0.869011134989500 at 0.548563444114526, here on input domain [0.5,2.5]
    /// Defined only on 1-dimensional space.
    pub fn gramacy_lee(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 1);
        let a = scale_domain(&x[0], 0.5, 2.5);
        ((10.0 * PI * a).sin()) / (2.0 * a) + (a - 1.0).powi(4)
    }

    /// Himmelblau function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (3,2), (−2.805118,3.283186), (−3.779310,−3.283186), (3.584458,−1.848126) here on input domain [-6,6]
    /// Defined only on 2-dimensional space.
    pub fn himmelblau(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -6.0, 6.0);
        let b = scale_domain(&x[1], -6.0, 6.0);
        (a.powi(2) + b - 11.0).powi(2) + (a + b.powi(2) - 7.0).powi(2)
    }

    /// Holder-Table function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: −19.2085 at (±8.05502,±9.66459), here on input domain [-10,10]
    /// Defined only on 2-dimensional space.
    pub fn holder_table(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -10.0, 10.0);
        let b = scale_domain(&x[1], -10.0, 10.0);
        -(a.sin() * b.cos() * ((1.0 - (a.powi(2) + b.powi(2)).sqrt() / PI).abs()).exp()).abs()
    }

    /// Keane function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: - 0.673667521146855 at (1.393249070031784,0) and (0,1.393249070031784), here on input domain [0,10]
    /// Defined only on 2-dimensional space.
    pub fn keane(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], 0.0, 10.0);
        let b = scale_domain(&x[1], 0.0, 10.0);
        -(((a - b).sin()).powi(2) * ((a + b).sin()).powi(2)) / ((a.powi(2) + b.powi(2)).sqrt())
    }

    /// Leon function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (1,1), here on input domain [0,10]
    /// Defined only on 2-dimensional space.
    pub fn leon(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], 0.0, 10.0);
        let b = scale_domain(&x[1], 0.0, 10.0);
        100.0 * (b - a.powi(3)).powi(2) + (1.0 - a).powi(2)
    }

    /// Levi Nr. 13 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (1,1), here on input domain [-10,10]
    /// Defined only on 2-dimensional space.
    pub fn levi_n13(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -10.0, 10.0);
        let b = scale_domain(&x[1], -10.0, 10.0);
        ((3.0 * PI * a).sin()).powi(2)
            + (a - 1.0).powi(2) * (1.0 + ((3.0 * PI * b).sin()).powi(2))
            + (b - 1.0).powi(2) * (1.0 + ((2.0 * PI * b).sin()).powi(2))
    }

    /// Matyas function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,0), here on input domain [-10,10]
    /// Defined only on 2-dimensional space.
    pub fn matyas(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -10.0, 10.0);
        let b = scale_domain(&x[1], -10.0, 10.0);
        0.26 * (a.powi(2) + b.powi(2)) - 0.48 * a * b
    }

    /// McCormick function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: −1.9133 at (−0.547,−1.547), here on input domain [-1.5,4] for x and [-3,3] for y
    /// Defined only on 2-dimensional space.
    pub fn mccormick(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -1.5, 4.0);
        let b = scale_domain(&x[1], -3.0, 3.0);
        (a + b).sin() + (a - b).powi(2) - 1.5 * a + 2.5 * b + 1.0
    }

    /// Schaffer Nr. 1 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,0), here on input domain [-100,100]
    /// Defined only on 2-dimensional space.
    pub fn schaffer_n1(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -100.0, 100.0);
        let b = scale_domain(&x[1], -100.0, 100.0);
        0.5 + ((((a.powi(2) + b.powi(2)).powi(2)).sin()).powi(2) - 0.5)
            / (1.0 + 0.001 * (a.powi(2) + b.powi(2))).powi(2)
    }

    /// Schaffer Nr. 2 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,0), here on input domain [-100,100]
    /// Defined only on 2-dimensional space.
    pub fn schaffer_n2(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -100.0, 100.0);
        let b = scale_domain(&x[1], -100.0, 100.0);
        0.5 + (((a.powi(2) - b.powi(2)).sin()).powi(2) - 0.5)
            / (1.0 + 0.001 * (a.powi(2) + b.powi(2))).powi(2)
    }

    /// Schaffer Nr. 3 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0.00156685 at (0,1.253115), here on input domain [-100,100]
    /// Defined only on 2-dimensional space.
    pub fn schaffer_n3(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -100.0, 100.0);
        let b = scale_domain(&x[1], -100.0, 100.0);
        0.5 + (((((a.powi(2) + b.powi(2)).abs()).cos()).sin()).powi(2) - 0.5)
            / (1.0 + 0.001 * (a.powi(2) + b.powi(2))).powi(2)
    }

    /// Schaffer Nr. 4 function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0.292579 at (0,1.253115), here on input domain [-100,100]
    /// Defined only on 2-dimensional space.
    pub fn schaffer_n4(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -100.0, 100.0);
        let b = scale_domain(&x[1], -100.0, 100.0);
        0.5 + (((((a.powi(2) + b.powi(2)).abs()).sin()).cos()).powi(2) - 0.5)
            / (1.0 + 0.001 * (a.powi(2) + b.powi(2))).powi(2)
    }

    /// Three-Hump Camel function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,0), here on input domain [-5,5]
    /// Defined only on 2-dimensional space.
    pub fn three_hump_camel(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 2);
        let a = scale_domain(&x[0], -5.0, 5.0);
        let b = scale_domain(&x[1], -5.0, 5.0);
        2.0 * a.powi(2) - 1.05 * a.powi(4) + (a.powi(6) / 6.0) + a * b + b.powi(2)
    }

    /// Wolfe function
    ///
    /// Scaled to [-1.0, 1.0]
    ///
    /// Optimum: 0 at (0,0,0), here on input domain [0,2]
    /// Defined only on 3-dimensional space.
    pub fn wolfe(x: &[f64]) -> f64 {
        debug_assert_eq!(x.len(), 3);
        let a = scale_domain(&x[0], 0.0, 2.0);
        let b = scale_domain(&x[1], 0.0, 2.0);
        let c = scale_domain(&x[2], 0.0, 2.0);
        4.0 / 3.0 * (a.powi(2) + b.powi(2) - a * b).powf(0.75) + c
    }
}
