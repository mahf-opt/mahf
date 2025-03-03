use rand::distributions::{Distribution, Uniform};
use serde::Serialize;

use crate::{
    component::ExecResult,
    components::Component,
    identifier::{Global, Identifier, PhantomId},
    problems::LimitedVectorProblem,
    SingleObjectiveProblem, State,
};
use crate::population::IntoIndividuals;

/// Updates the positions of particles similar to the nuclear reaction mechanism proposed for the
/// Nuclear Fission-Nuclear Fusion (NFNF/N2F) algorithm.
#[derive(Clone, Serialize)]
pub struct NuclearReactionMechanism<I: Identifier = Global> {
    /// Number of new individuals to generate.
    pub new_pop: u32,
    /// Magnification factor.
    pub mu: f64,
    /// Amplification factor.
    pub rho: f64,
    /// Type of termination criterion counter.
    pub termination_type: String,
    /// Termination criterion.
    pub termination_value: usize,
    id: PhantomId<I>,
}

impl<I: Identifier> NuclearReactionMechanism<I> {
    pub fn from_params(new_pop: u32, mu: f64, rho: f64, termination_type: String, termination_value: usize) -> Self {
        Self {
            new_pop,
            mu,
            rho,
            termination_type,
            termination_value,
            id: PhantomId::default(),
        }
    }

    pub fn new_with_id<P>(new_pop: u32, mu: f64, rho: f64, termination_type: String, termination_value: usize) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(new_pop,
                                   mu,
                                   rho,
                                   termination_type,
                                   termination_value))
    }
}

impl NuclearReactionMechanism<Global> {
    pub fn new<P>(new_pop: u32, mu: f64, rho: f64, termination_type: String, termination_value: usize) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id(new_pop,
                          mu,
                          rho,
                          termination_type,
                          termination_value)
    }
}

impl<P, I> Component<P> for NuclearReactionMechanism<I>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, _state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut rng = state.random_mut();
        let distribution = Uniform::new(0., 1.0);

        // Get population from state
        let xs = state.populations_mut().pop();

        // prepare parameters
        let &Self {
            new_pop, mu, rho, termination_value, ..
        } = self;

        // Calculate magnification exponent
        let mut m_exponent = 0.0;
        if self.termination_type.as_str() == "iterations" {
            m_exponent = - (state.iterations() as f64 / termination_value as f64);
        } else if self.termination_type.as_str() == "evaluations" {
            m_exponent = - (state.evaluations() as f64 / termination_value as f64);
        } else {
            println!("Invalid termination type");
        }

        // Shift fitness values to always be > 0; use best_objective_value as minimum that is always
        // at least EPSILON bigger than 0
        let min = state.best_objective_value().unwrap().value();
        let objective_values: Vec<_> = if min > 0.0 {
            xs.iter().map(|i| i.objective().value()).collect()
        } else {
            xs.iter().map(|i| i.objective().value() + min.abs() + f64::EPSILON).collect()
        };
        let best_objective = if min > 0.0 {
            min
        } else {
            min + min.abs() + f64::EPSILON
        };
        
        // Calculate equivalent to center of mass
        let inverse_fitness_sum = objective_values
            .iter()
            .map(|o| best_objective / o)
            .sum::<f64>();
        
        let mut positions = Vec::new();
        for (o, i) in xs.iter().enumerate() {
            let weighted_position = i.solution()
                .iter()
                .map(|x| (best_objective / objective_values[o]) * x)
                .collect::<Vec<f64>>();
            positions.push(weighted_position);
        }

        let sum_positions = positions.iter()
            .map(|v| v.iter()) // Convert each vector into an iterator
            .fold(None, |acc: Option<Vec<f64>>, v_iter| {
                Some(match acc {
                    None => v_iter.cloned().collect(), // First vector initializes the result
                    Some(mut acc_vec) => {
                        acc_vec.iter_mut().zip(v_iter).for_each(|(a, &b)| *a += b);
                        acc_vec
                    }
                })
            }).unwrap_or_default();

        let center = sum_positions.iter().map(|p| p / inverse_fitness_sum).collect::<Vec<f64>>();

        // Generate new candidate solutions (new_pop specifies how many)
        let mut new_solutions = Vec::new();
        for _ in 0..new_pop {
            let new_ind = center
                .iter()
                .zip(problem.domain())
                .map(|(c, p)| c + distribution.sample(&mut *rng) * (p.end - p.start) * mu.powf(m_exponent))
                .collect::<Vec<f64>>();
            new_solutions.push(new_ind);
        }
        
        state.populations_mut().push(new_solutions.into_individuals());
        Ok(())
    }
}