use mahf::framework::{components, ConfigurationBuilder};
use mahf::operators::{generation, initialization, replacement, selection, termination};
use mahf::problems::coco_bound::suits;

fn main() -> anyhow::Result<()> {
    let output = "data/coco/iwo";
    #[rustfmt::skip]
    let config = ConfigurationBuilder::new()
        .do_(initialization::RandomSpread::new(5))
        .do_(components::SimpleEvaluator::new())
        .while_(termination::FixedIterations::new(500))
            .do_(selection::DeterministicFitnessProportional::new(0, 5))
            .do_(generation::IWOAdaptiveDeviationDelta::new(0.5, 0.001, 3))
            .do_(components::SimpleEvaluator::new())
            .do_(replacement::MuPlusLambda::new(20))
        .while_end()
        .build();

    let suite = suits::bbob();

    suits::evaluate_suite(suite, config, output)
}
