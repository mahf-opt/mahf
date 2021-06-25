pub type ArgsIter = std::iter::Skip<std::env::Args>;

#[derive(Debug, Default)]
pub struct BaseParams {
    pub instance: String,
    pub instance_information: String,
    pub cutoff_time: f64,
    pub cutoff_length: u32,
    pub seed: u64,
}

/// Returns the base params and an iterator for the remaining params.
pub fn get_parameters() -> (String, BaseParams, ArgsIter) {
    let mut args = std::env::args().skip(1);

    let heuristic = args.next().unwrap();

    let base = BaseParams {
        instance: args.next().unwrap(),
        instance_information: args.next().unwrap(),
        cutoff_time: args.next().unwrap().parse().unwrap(),
        cutoff_length: args.next().unwrap().parse().unwrap(),
        seed: args.next().unwrap().parse().unwrap(),
    };

    (heuristic, base, args)
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
