use mahf::prelude::*;
use problems::coco_bound::{suits, CocoInstance};
use tracking::{functions, trigger};
use framework::{components::Component, conditions::Condition, Configuration};
use mahf::components::generation::recombination::{NPointCrossover, UniformCrossover};
use mahf::state::common;
use problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem};
use mahf::state::diversity::*;


fn main() -> anyhow::Result<()> {
    let folder = "data/diversity/es";
    let measures = [DiversityMeasure::DW,
        DiversityMeasure::PW,
        DiversityMeasure::TD,
        DiversityMeasure::DTAP];
    for (i, measure) in measures.into_iter().enumerate() {
        let output = format!("{}{}{}{}", folder, "/", i.to_string(), "UX");
        let config = diversity_mu_plus_lambda(
            RealProblemParameters {
                population_size: 5,
                lambda: 15,
                deviation: 0.2,
                crossover: UniformCrossover::new(0.9),
                div_measure: measure,
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
                        .with_logger(functions::auto::<DiversityState>),
                )
                .build(),
        );
        let suite = suits::bbob();

        suits::evaluate_suite(suite, config, &output).expect("TODO: panic message");
    }

    for (i, measure) in measures.into_iter().enumerate() {
        let output = format!("{}{}{}{}", folder, "/", i.to_string(), "NPX");
        let config = diversity_mu_plus_lambda(
            RealProblemParameters {
                population_size: 5,
                lambda: 15,
                deviation: 0.2,
                crossover: NPointCrossover::new(0.9, 1),
                div_measure: measure,
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
                        .with_logger(functions::auto::<DiversityState>),
                )
                .build(),
        );
        let suite = suits::bbob();

        suits::evaluate_suite(suite, config, &output).expect("TODO: panic message");
    }
    Ok(())
}


// Parameters
pub struct RealProblemParameters<P> {
    pub population_size: u32,
    pub lambda: u32,
    pub deviation: f64,
    pub crossover: Box<dyn Component<P>>,
    pub div_measure: DiversityMeasure,
}

/* An example single-objective (μ+λ)-Evolution-Strategy operating on a real search space.
Uses [state\diversity].
Uses the [custom_es] component internally. */
pub fn diversity_mu_plus_lambda<P>(
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
        div_measure,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(population_size))
        .evaluate_sequential()
        .update_best_individual()
        .do_(custom_es(
            Parameters {
                selection: selection::FullyRandom::new(lambda),
                mutation: generation::mutation::FixedDeviationDelta::new(deviation),
                crossover,
                replacement: replacement::MuPlusLambda::new(population_size),
                diversity: DiversityState::for_float_vector(div_measure),
            },
            termination,
            logger,
        ))
        .build()
}

// Basic building blocks of an Evolution Strategy.
pub struct Parameters<P> {
    pub selection: Box<dyn Component<P>>,
    pub mutation: Box<dyn Component<P>>,
    pub crossover: Box<dyn Component<P>>,
    pub replacement: Box<dyn Component<P>>,
    pub diversity: Box<dyn Component<P>>,
}

// A single-objective Evolution Strategy template using diversity metrics.
pub fn custom_es<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>> {
    let Parameters {
        selection,
        mutation,
        crossover,
        replacement,
        diversity,
    } = params;

    Configuration::builder()
        .while_(termination, |builder| {
            builder
                .do_(selection)
                .do_(mutation)
                .do_(crossover)
                .evaluate_sequential()
                .update_best_individual()
                .do_(replacement)
                .do_(diversity)
                .do_(logger)
        })
        .build_component()
}
