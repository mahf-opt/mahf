use mahf::prelude::*;
use mahf::state::common;
use problems::coco_bound::{suits, CocoInstance};
use tracking::{functions, trigger};

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
        tracking::Logger::builder()
            .log_set(
                tracking::LogSet::new()
                    .with_trigger(trigger::Iteration::new(50))
                    .with_trigger(trigger::FinalIter::new(500))
                    .with_trigger(trigger::TargetHit::new())
                    .with_auto_logger::<common::Evaluations>()
                    .with_auto_logger::<common::Progress>()
                    .with_logger(functions::best_individual::<CocoInstance>)
                    .with_logger(functions::best_objective_value::<CocoInstance>),
            )
            .build(),
    );
    let suite = suits::bbob();

    suits::evaluate_suite(suite, config, output)
}
