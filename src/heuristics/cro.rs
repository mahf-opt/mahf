//! Chemical Reaction Optimization

use crate::{
    components::*,
    conditions::*,
    framework::{Configuration, ConfigurationBuilder},
    problems::{LimitedVectorProblem, SingleObjectiveProblem, VectorProblem},
    state,
};

/// Parameters for [cro].
pub struct RealProblemParameters {
    pub initial_population_size: u32,
    pub mole_coll: f64,
    pub kinetic_energy_loss_rate: f64,
    pub alpha: u32,
    pub beta: f64,
    pub initial_kinetic_energy: f64,
    pub buffer: f64,
    pub on_wall_deviation: f64,
    pub decomposition_deviation: f64,
}

/// An example single-objective Chemical Reaction Optimization operating on a real search space.
/// Uses the [cro] component internally.
pub fn real_cro<P>(
    params: RealProblemParameters,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Configuration<P>
where
    P: SingleObjectiveProblem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem,
{
    let RealProblemParameters {
        initial_population_size,
        mole_coll,
        kinetic_energy_loss_rate,
        alpha,
        beta,
        initial_kinetic_energy,
        buffer,
        on_wall_deviation,
        decomposition_deviation,
    } = params;

    Configuration::builder()
        .do_(initialization::RandomSpread::new_init(
            initial_population_size,
        ))
        .evaluate()
        .update_best_individual()
        .do_(cro(
            Parameters {
                mole_coll,
                kinetic_energy_loss_rate,
                initial_kinetic_energy,
                buffer,
                single_mole_selection: selection::RandomWithoutRepetition::new(1),
                decomposition_criterion: branching::DecompositionCriterion::new(alpha),
                decomposition: Block::new([
                    generation::DuplicatePopulation::new(),
                    generation::mutation::UniformPartialMutation::new(
                        0.5,
                        generation::mutation::FixedDeviationDelta::new(decomposition_deviation),
                    ),
                ]),
                on_wall_ineffective_collision: generation::mutation::FixedDeviationDelta::new(
                    on_wall_deviation,
                ),
                double_mole_selection: selection::RandomWithoutRepetition::new(2),
                synthesis_criterion: branching::SynthesisCriterion::new(beta),
                synthesis: generation::recombination::UniformCrossover::new_single(1.),
                intermolecular_ineffective_collision: generation::mutation::UniformMutation::new(
                    1.,
                ),
                constraints: constraints::Saturation::new(),
            },
            termination,
            logger,
        ))
        .build()
}

/// Basic building blocks of an Chemical Reaction Optimization.
pub struct Parameters<P> {
    pub mole_coll: f64,
    pub kinetic_energy_loss_rate: f64,
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
pub fn cro<P: SingleObjectiveProblem>(
    params: Parameters<P>,
    termination: Box<dyn Condition<P>>,
    logger: Box<dyn Component<P>>,
) -> Box<dyn Component<P>> {
    let Parameters {
        mole_coll,
        kinetic_energy_loss_rate,
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
            .evaluate()
            .update_best_individual()
            .do_(update)
    };

    Configuration::builder()
        .do_(state::CroState::initializer(initial_kinetic_energy, buffer))
        .while_(termination, |builder| {
            builder
                .if_else_(branching::RandomChance::new(mole_coll) | branching::LessThanNIndividuals::new(2),
                          |builder| {
                              builder
                                  .do_(single_mole_selection)
                                  .do_(selection::All::new())
                                  .if_else_(decomposition_criterion,
                                            |builder| {
                                                elementary_reaction(
                                                    builder,
                                                    decomposition,
                                                    state::CroState::decomposition_update()
                                                )
                                            },
                                            |builder| {
                                                elementary_reaction(
                                                    builder,
                                                    on_wall_ineffective_collision,
                                                    state::CroState::on_wall_ineffective_collision_update(
                                                        kinetic_energy_loss_rate,
                                                    ))
                                            },
                                  )
                          },
                          |builder| {
                              builder
                                  .do_(double_mole_selection)
                                  .do_(selection::All::new())
                                  .if_else_(synthesis_criterion,
                                            |builder| {
                                                elementary_reaction(
                                                    builder,
                                                    synthesis,
                                                    state::CroState::synthesis_update()
                                                )
                                            },
                                            |builder| {
                                                elementary_reaction(
                                                    builder,
                                                    intermolecular_ineffective_collision,
                                                    state::CroState::intermolecular_ineffective_collision_update()
                                                )
                                            },
                                  )
                          },
                )
                .do_(logger)
        })
        .build_component()
}
