use mahf::prelude::*;
use problems::coco_bound::{suits, CocoInstance};
use tracking::{functions, trigger};
use framework::{components::Component, conditions::Condition, Configuration};
use mahf::components::generation::recombination::{NPointCrossover, UniformCrossover};
use mahf::state::common;
use problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem};
use mahf::state::diversity::*;


fn main() -> anyhow::Result<()> {
    let folder = "data/diversity/ea";
    // set the crossover probabilities to use
    let crossover_prob = [0.2, 0.5, 0.9];
    // set the mutation deviations to use
    let mutation_dev = [0.01, 0.2, 0.5];
    for probability in crossover_prob {
        for dev in mutation_dev {
            let output = format!("{}{}{}{}{}{}", folder, "/", "UX_", probability.to_string(), "_", dev);
            let config = diversity_ea(
                RealProblemParameters {
                    population_size: 5,
                    lambda: 15,
                    deviation: dev,
                    crossover: UniformCrossover::new(probability, true),
                },
                termination::FixedEvaluations::new(1000) & termination::TargetHit::new(),
                tracking::Logger::builder()
                    .log_set(
                        tracking::LogSet::new()
                            .with_trigger(trigger::Iteration::new(50))
                            .with_trigger(trigger::FinalEval::new(1000))
                            .with_auto_logger::<common::Evaluations>()
                            .with_auto_logger::<common::Progress>()
                            .with_logger(functions::best_individual::<CocoInstance>)
                            .with_logger(functions::best_objective_value::<CocoInstance>)
                            .with_logger(functions::normalized_diversity::<DimensionWiseDiversity>)
                            .with_logger(functions::normalized_diversity::<PairwiseDistanceDiversity>)
                            .with_logger(functions::normalized_diversity::<DistanceToAveragePointDiversity>)
                            .with_logger(functions::normalized_diversity::<TrueDiversity>),
                    )
                    .build(),
            );
            let suite = suits::bbob();

            suits::evaluate_suite(suite, config, &output).expect("TODO: panic message");

            let output = format!("{}{}{}{}{}{}", folder, "/", "NPX_", probability.to_string(), "_", dev);
            let config = diversity_ea(
                RealProblemParameters {
                    population_size: 5,
                    lambda: 15,
                    deviation: dev,
                    crossover: NPointCrossover::new(probability, 1, true),
                },
                termination::FixedEvaluations::new(1000) & termination::TargetHit::new(),
                tracking::Logger::builder()
                    .log_set(
                        tracking::LogSet::new()
                            .with_trigger(trigger::Iteration::new(50))
                            .with_trigger(trigger::FinalEval::new(1000))
                            .with_auto_logger::<common::Evaluations>()
                            .with_auto_logger::<common::Progress>()
                            .with_logger(functions::best_individual::<CocoInstance>)
                            .with_logger(functions::best_objective_value::<CocoInstance>)
                            .with_logger(functions::normalized_diversity::<DimensionWiseDiversity>)
                            .with_logger(functions::normalized_diversity::<PairwiseDistanceDiversity>)
                            .with_logger(functions::normalized_diversity::<DistanceToAveragePointDiversity>)
                            .with_logger(functions::normalized_diversity::<TrueDiversity>),
                    )
                    .build(),
            );

            let suite2 = suits::bbob();

            suits::evaluate_suite(suite2, config, &output).expect("TODO: panic message");
        }
    }
    Ok(())
}


// Parameters
pub struct RealProblemParameters<P> {
    pub population_size: u32,
    pub lambda: u32,
    pub deviation: f64,
    pub crossover: Box<dyn Component<P>>,
}

/* An example single-objective evolutionary algorithm operating on a real search space.
Uses [state\diversity].
Uses the [custom_ea] component internally. */
pub fn diversity_ea<P>(
    params: RealProblemParameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
    where
        P: SingleObjectiveProblem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    let RealProblemParameters {
        population_size,
        lambda,
        deviation,
        crossover,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(population_size))
        .evaluate_sequential()
        .update_best_individual()
        .do_(custom_ea(
            Parameters {
                selection: selection::FullyRandom::new(lambda),
                crossover,
                mutation: generation::mutation::FixedDeviationDelta::new(deviation),
                constraints: constraints::Saturation::new(),
                replacement: replacement::MuPlusLambda::new(population_size),
                diversity_1: DimensionWiseDiversity::new(),
                diversity_2: PairwiseDistanceDiversity::new(),
                diversity_3: DistanceToAveragePointDiversity::new(),
                diversity_4: TrueDiversity::new(),
            },
            termination,
            logger,
        ))
        .build()
}

// Basic building blocks of an Evolution Strategy.
pub struct Parameters<P> {
    pub selection: Box<dyn Component<P>>,
    pub crossover: Box<dyn Component<P>>,
    pub mutation: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
    pub replacement: Box<dyn Component<P>>,
    pub diversity_1: Box<dyn Component<P>>,
    pub diversity_2: Box<dyn Component<P>>,
    pub diversity_3: Box<dyn Component<P>>,
    pub diversity_4: Box<dyn Component<P>>,
}

// A single-objective evolutionary algorithm template using diversity metrics.
pub fn custom_ea<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>> {
    let Parameters {
        selection,
        crossover,
        mutation,
        constraints,
        replacement,
        diversity_1,
        diversity_2,
        diversity_3,
        diversity_4,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(selection)
                .do_(crossover)
                .do_(mutation)
                .do_(constraints)
                .evaluate_sequential()
                .update_best_individual()
                .do_(replacement)
                .do_(diversity_1)
                .do_(diversity_2)
                .do_(diversity_3)
                .do_(diversity_4)
                .do_(logger)
        })
        .build_component()
}
