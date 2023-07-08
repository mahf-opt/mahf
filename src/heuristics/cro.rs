//! Chemical Reaction Optimization (CRO).
//!
//! # References
//!
//! \[1\] Albert Y. S. Lam and Victor O. K. Li. 2010.
//! Chemical-Reaction-Inspired Metaheuristic for Optimization.
//! IEEE Transactions on Evolutionary Computation 14, 3 (June 2010), 381–399.
//! DOI:<https://doi.org/10/bktgqf>
//!
//! \[2\] Albert Y. S. Lam and Victor O. K. Li. 2012.
//! Chemical Reaction Optimization: a tutorial.
//! Memetic Comp. 4, 1 (March 2012), 3–17.
//! DOI:<https://doi.org/10/ggbmwk>
//!
//! \[3\] Albert Y. S. Lam, Victor O. K. Li, and James J. Q. Yu. 2012.
//! Real-Coded Chemical Reaction Optimization.
//! IEEE Transactions on Evolutionary Computation 16, 3 (June 2012), 339–353.
//! DOI:<https://doi.org/10/fm8zqt>

use crate::{
    components::{
        boundary, initialization, misc, mutation, recombination, selection, utils, Block,
    },
    conditions::{self, LessThanN, RandomChance},
    configuration::ConfigurationBuilder,
    identifier::{Global, Identifier, A, B},
    lens::common::PopulationSizeLens,
    logging::Logger,
    problems::LimitedVectorProblem,
    Component, Condition, Configuration, ExecResult, SingleObjectiveProblem,
};

/// Parameters for [`real_cro`].
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

/// An example single-objective CRO operating on a real search space.
///
/// Uses the [`cro`] component internally.
pub fn real_cro<P>(
    params: RealProblemParameters,
    condition: Box<dyn Condition<P>>,
) -> ExecResult<Configuration<P>>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
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
        .evaluate()
        .update_best_individual()
        .do_(cro::<P, Global>(
            Parameters {
                mole_coll,
                kinetic_energy_lr,
                initial_kinetic_energy,
                buffer,
                single_mole_selection: selection::RandomWithoutRepetition::new(1),
                decomposition_criterion: conditions::cro::DecompositionCriterion::new(alpha),
                decomposition: Block::new([
                    utils::populations::DuplicatePopulation::new(),
                    mutation::NormalMutation::<A>::new_with_id(decomposition_deviation, 0.5),
                ]),
                on_wall_ineffective_collision: mutation::NormalMutation::<B>::new_with_id(
                    on_wall_deviation,
                    1.0,
                ),
                double_mole_selection: selection::RandomWithoutRepetition::new(2),
                synthesis_criterion: conditions::cro::SynthesisCriterion::new(beta),
                synthesis: recombination::UniformCrossover::new_insert_single(1.),
                intermolecular_ineffective_collision: mutation::UniformMutation::new_bound(1.),
                constraints: boundary::Saturation::new(),
            },
            condition,
        ))
        .build())
}

/// Basic building blocks of [`cro`].
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

/// A generic single-objective Chemical Reaction Optimization (CRO) template.
pub fn cro<P, I>(params: Parameters<P>, condition: Box<dyn Condition<P>>) -> Box<dyn Component<P>>
where
    P: SingleObjectiveProblem,
    I: Identifier,
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
            .evaluate_with::<I>()
            .update_best_individual()
            .do_(update)
    };

    Configuration::builder()
        .do_(misc::cro::ChemicalReactionInit::new(
            initial_kinetic_energy,
            buffer,
        ))
        .while_(condition, |builder| {
            builder
                .if_else_(
                    RandomChance::new(mole_coll)
                        | LessThanN::new(2, PopulationSizeLens::<P>::new()),
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
                                        misc::cro::DecompositionUpdate::new(),
                                    )
                                },
                                |builder| {
                                    elementary_reaction(
                                        builder,
                                        on_wall_ineffective_collision,
                                        misc::cro::OnWallIneffectiveCollisionUpdate::new(
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
                                        misc::cro::SynthesisUpdate::new(),
                                    )
                                },
                                |builder| {
                                    elementary_reaction(
                                        builder,
                                        intermolecular_ineffective_collision,
                                        misc::cro::IntermolecularIneffectiveCollisionUpdate::new(),
                                    )
                                },
                            )
                    },
                )
                .do_(Logger::new())
        })
        .build_component()
}
