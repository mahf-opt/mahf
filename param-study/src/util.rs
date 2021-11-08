pub type ArgsIter = std::iter::Skip<std::env::Args>;

#[derive(Debug, Default)]
pub struct Setup {
    pub instance: String,
    pub instance_information: String,
    pub cutoff_time: f64,
    pub cutoff_length: u32,
    pub seed: u64,
}

/// Returns the base params and an iterator for the remaining params.
pub fn get_parameters() -> (String, Setup, ArgsIter) {
    let mut args = std::env::args().skip(1);

    let heuristic = args.next().unwrap();

    let base = Setup {
        instance: args.next().unwrap(),
        instance_information: args.next().unwrap(),
        cutoff_time: args.next().unwrap().parse().unwrap(),
        cutoff_length: args.next().unwrap().parse().unwrap(),
        seed: args.next().unwrap().parse().unwrap(),
    };

    (heuristic, base, args)
}

#[macro_export]
macro_rules! declare_parameters {
    { $($p_name:ident : $p_type:ty,)* } => {
        #[derive(Debug, Default)]
        struct Parameters {
            $($p_name: $p_type),*
        }

        fn parameters(args: &mut ArgsIter) -> Parameters {
            let mut params = Parameters::default();

            while let Some(param) = args.next() {
                let value = args.next().unwrap();

                match param.as_str() {
                    $(concat!("-", stringify!($p_name)) => params.$p_name = value.parse().unwrap(),)*
                    unknown => panic!("unknown param {}", unknown),
                }
            }

            params
        }
    };
}

/// Prints output for ParamILS.
///
/// This can be called multiple times and the last call will define
/// the final result of this evaluation.
pub fn print_result(sat: bool, runtime: f64, runlength: u32, best: f64, seed: u64) {
    println!(
        "Result for ParamILS: {}, {}, {}, {}, {}",
        if sat { "SAT" } else { "TIMEOUT" },
        runtime,
        runlength,
        best,
        seed
    );
}
