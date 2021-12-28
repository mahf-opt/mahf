use mahf::{heuristics::iwo, problems::coco_bound::suits};

fn main() -> anyhow::Result<()> {
    let output = "data/coco/iwo";
    let config = iwo::iwo(5, 20, 0, 5, 0.5, 0.001, 3, 500);
    let suite = suits::bbob();

    suits::evaluate_suite(suite, config, output)
}
