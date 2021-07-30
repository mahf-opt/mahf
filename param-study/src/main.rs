pub mod util;

mod iwo;
mod pso;

fn main() {
    let (heuristic, ref setup, ref mut args) = util::get_parameters();

    match heuristic.as_str() {
        "iwo" => iwo::run(setup, args),
        "pso" => pso::run(setup, args),
        _ => panic!("Unknown heuristic {}", heuristic),
    }
}
