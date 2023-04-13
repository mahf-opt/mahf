use mahf::{prelude::*, state::common, tracking::LogSet};
use problems::coco_bound::{suits, CocoInstance};
use tracking::{extractor, trigger};

fn main() -> anyhow::Result<()> {
    let output = "data/coco/iwo";
    let config = iwo::real_iwo(
        iwo::RealProblemParameters {
            initial_population_size: 5,
            max_population_size: 20,
            min_number_of_seeds: 0,
            max_number_of_seeds: 5,
            initial_deviation: 0.5,
            final_deviation: 0.001,
            modulation_index: 3,
        },
        termination::FixedIterations::new(500) & termination::TargetHit::new(),
    );
    let suite = suits::bbob();

    suits::evaluate_suite(suite, config, output, |state| {
        state.insert(
            LogSet::<CocoInstance>::new()
                .with_common_extractors(trigger::Iteration::new(10))
                .with(
                    trigger::Change::<common::Progress<common::Iterations>>::new(0.1),
                    extractor::auto::<common::Progress<common::Iterations>, _>,
                )
                .with(trigger::Iteration::new(50), extractor::best_individual),
        )
    })
}
