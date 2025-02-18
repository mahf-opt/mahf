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

/// Updates the positions of particles according to the nuclear reaction mechanism proposed for the
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
            m_exponent = - (termination_value as f64 / state.iterations() as f64);
        } else if self.termination_type.as_str() == "evaluations" {
            m_exponent = - (termination_value as f64 / state.evaluations() as f64);
        } else {
            println!("Invalid termination type");
        }

        // Calculate equivalent to center of mass
        let inverse_fitness_sum = xs
            .iter()
            .map(|f| (state.best_objective_value().unwrap().value() / f.objective().value())
                .powf(rho.powi((state.iterations() - 1) as i32)))
            .sum::<f64>();

        println!("Calculated inverse_fitness_sum");
        // TODO find endless loop?
        let mut positions = Vec::new();
        for i in xs.iter() {
            let weighted_position = i.solution()
                .iter()
                .map(|x| (state.best_objective_value().unwrap().value() / i.objective().value())
                    .powf(rho.powi((state.iterations() - 1) as i32)) * x)
                .collect::<Vec<f64>>();
            positions.push(weighted_position);
        }
        println!("Calculated weighted positions");

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
        println!("Calculated sum_positions");

        let center = sum_positions.iter().map(|p| p / inverse_fitness_sum).collect::<Vec<f64>>();

        println!("Calculated center");
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