use mahf::framework::{components, ConfigurationBuilder};
use mahf::operators::{generation, initialization, replacement, selection, termination};
use mahf::problems::coco_bound::suits;

fn main() -> anyhow::Result<()> {
    let output = "data/coco/iwo";
    let config = ConfigurationBuilder::new()
        .do_(initialization::RandomSpread::new(5))
        .do_(components::SimpleEvaluator::new())
        .while_(termination::FixedIterations::new(500), |builder| {
            builder
                .do_(selection::DeterministicFitnessProportional::new(0, 5))
                .do_(generation::IWOAdaptiveDeviationDelta::new(0.5, 0.001, 3))
                .do_(components::SimpleEvaluator::new())
                .do_(replacement::MuPlusLambda::new(20))
        })
        .build();

    let suite = suits::bbob();

    suits::evaluate_suite(suite, config, output)
}
