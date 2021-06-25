pub mod util;

mod iwo;

fn main() {
    let (heuristic, base_params, mut args) = util::get_parameters();

    match heuristic.as_str() {
        "iwo" => iwo::run(base_params, &mut args),
        _ => panic!("Unknown heuristic {}", heuristic),
    }
}
