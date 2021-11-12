mod instances;
mod util;

fn main() {
    let (heuristic, ref setup, ref mut args) = util::get_parameters();

    match heuristic.as_str() {
        "iwo" => instances::iwo::run(setup, args),
        "pso" => instances::pso::run(setup, args),
        _ => panic!("Unknown heuristic {}", heuristic),
    }
}
