use crate::{
    component::ExecResult,
    components::{self, *},
    conditions::{self, *},
    configuration::{Configuration, ConfigurationBuilder},
    identifier::{A, B},
    logging::Logger,
    problems::{Evaluator, LimitedVectorProblem, SingleObjectiveProblem},
    state::lens::common::PopulationSizeLens,
};

/// Parameters for [cro].
pub struct RealProblemParameters {
    pub initial_population_size: u32,
    pub mole_coll: f64,
    pub kinetic_energy_lr: f64,
    pub alpha: u32,
    pub beta: f64,
    pub initial_kinetic_energy: f64,
    pub buffer: f64,
    pub on_wall_deviation: f64,
    pub decomposition_deviation: f64,
}

/// An example single-objective Chemical Reaction Optimization operating on a real search space.
/// Uses the [cro] component internally.
pub fn real_cro<P, O>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    O: Evaluator<Problem = P>,
{
    let RealProblemParameters {
        initial_population_size,
        mole_coll,
        kinetic_energy_lr,
        alpha,
        beta,
        initial_kinetic_energy,
        buffer,
        on_wall_deviation,
        decomposition_deviation,
    } = params;

    Ok(Configuration::builder()
        .do_(initialization::RandomSpread::new(initial_population_size))
        .evaluate::<O>()
        .update_best_individual()
        .do_(cro::<P, O>(
            Parameters {
                mole_coll,
                kinetic_energy_lr,
                initial_kinetic_energy,
                buffer,
                single_mole_selection: selection::RandomWithoutRepetition::new(1),
                decomposition_criterion: conditions::cro::DecompositionCriterion::new(alpha),
                decomposition: Block::new([
                    misc::populations::DuplicatePopulation::new(),
                    mutation::NormalMutation::<A>::new(decomposition_deviation, 0.5),
                ]),
                on_wall_ineffective_collision: mutation::NormalMutation::<B>::new_dev(
                    on_wall_deviation,
                ),
                double_mole_selection: selection::RandomWithoutRepetition::new(2),
                synthesis_criterion: conditions::cro::SynthesisCriterion::new(beta),
                synthesis: recombination::UniformCrossover::new_insert_single(1.),
                intermolecular_ineffective_collision: <mutation::UniformMutation>::new_bound(1.),
                constraints: boundary::Saturation::new(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of an Chemical Reaction Optimization.
pub struct Parameters<P> {
    pub mole_coll: f64,
    pub kinetic_energy_lr: f64,
    pub initial_kinetic_energy: f64,
    pub buffer: f64,
    pub single_mole_selection: Box<dyn Component<P>>,
    pub decomposition_criterion: Box<dyn Condition<P>>,
    pub decomposition: Box<dyn Component<P>>,
    pub on_wall_ineffective_collision: Box<dyn Component<P>>,
    pub double_mole_selection: Box<dyn Component<P>>,
    pub synthesis_criterion: Box<dyn Condition<P>>,
    pub synthesis: Box<dyn Component<P>>,
    pub intermolecular_ineffective_collision: Box<dyn Component<P>>,
    pub constraints: Box<dyn Component<P>>,
}

/// A generic single-objective Chemical Reaction Optimization template.
///
/// # References
/// [doi.org/10.1007/s12293-012-0075-1](https://doi.org/10.1007/s12293-012-0075-1)
pub fn cro<P, O>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    O: Evaluator<Problem = P>,
{
    let Parameters {
        mole_coll,
        kinetic_energy_lr,
        initial_kinetic_energy,
        buffer,
        single_mole_selection,
        decomposition_criterion,
        decomposition,
        on_wall_ineffective_collision,
        double_mole_selection,
        synthesis_criterion,
        synthesis,
        intermolecular_ineffective_collision,
        constraints,
    } = params;

    let elementary_reaction = |builder: ConfigurationBuilder<P>, reaction, update| {
        builder
            .do_(reaction)
            .do_(constraints.clone())
            .evaluate::<O>()
            .update_best_individual()
            .do_(update)
    };

    Configuration::builder()
        .do_(components::cro::ChemicalReactionInit::new(
            initial_kinetic_energy,
            buffer,
        ))
        .while_(condition, |builder| {
            builder.if_else_(
                common::RandomChance::new(mole_coll)
                    | LessThan::new(2, PopulationSizeLens::<P>::new()),
                |builder| {
                    builder
                        .do_(single_mole_selection)
                        .do_(selection::All::new())
                        .if_else_(
                            decomposition_criterion,
                            |builder| {
                                elementary_reaction(
                                    builder,
                                    decomposition,
                                    components::cro::DecompositionUpdate::new(),
                                )
                            },
                            |builder| {
                                elementary_reaction(
                                    builder,
                                    on_wall_ineffective_collision,
                                    components::cro::OnWallIneffectiveCollisionUpdate::new(
                                        kinetic_energy_lr,
                                    ),
                                )
                            },
                        )
                },
                |builder| {
                    builder
                        .do_(double_mole_selection)
                        .do_(selection::All::new())
                        .if_else_(
                            synthesis_criterion,
                            |builder| {
                                elementary_reaction(
                                    builder,
                                    synthesis,
                                    components::cro::SynthesisUpdate::new(),
                                )
                            },
                            |builder| {
                                elementary_reaction(
                                    builder,
                                    intermolecular_ineffective_collision,
                                    components::cro::IntermolecularIneffectiveCollisionUpdate::new(),
                                )
                            },
                        )
                },
            )
                                .do_(Logger::new())
        })
        .build_component()
}
