use mahf::{
    framework::components,
    heuristics::iwo,
    operators::termination,
    problems::coco_bound::{suits, CocoInstance},
    tracking::{self, trigger, LogSet},
};

fn main() -> anyhow::Result<()> {
    let output = "data/coco/iwo";
    let config = iwo::iwo(
        iwo::Parameters {
            initial_population_size: 5,
            max_population_size: 20,
            min_number_of_seeds: 0,
            max_number_of_seeds: 5,
            initial_deviation: 0.5,
            final_deviation: 0.001,
            modulation_index: 3,
        },
        components::And::new(vec![
            termination::FixedIterations::new(500),
            termination::TargetHit::new(),
        ]),
        tracking::Logger::builder()
            .log_common_sets()
            .log_set(
                LogSet::new()
                    .with_trigger(trigger::Iteration::new(50))
                    .with_logger(tracking::functions::best_individual::<CocoInstance>),
            )
            .build(),
    );
    let suite = suits::bbob();

    suits::evaluate_suite(suite, config, output)
}
