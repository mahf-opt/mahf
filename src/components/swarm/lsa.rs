use itertools::izip;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use serde::Serialize;
use statrs::distribution::{Exp, Normal};
use statrs::statistics::Statistics;
use crate::{
    component::ExecResult,
    components::Component,
    identifier::{Global, Identifier, PhantomId},
    problems::LimitedVectorProblem,
    SingleObjectiveProblem, State,
};
use crate::components::initialization::functional::random_spread;
use crate::population::IntoIndividuals;

/// Updates the positions of particles according to the negatively charged stepped leader mechanism
/// proposed for the Lightning Search Algorithm (LSA).
#[derive(Clone, Serialize)]
pub struct NegativelyChargedSteppedLeader<I: Identifier = Global> {
    /// Number of new individuals to generate.
    pub new_pop: u32,
    /// Solution used as leader.
    pub leader: String,
    id: PhantomId<I>,
}

impl<I: Identifier> NegativelyChargedSteppedLeader<I> {
    pub fn from_params(new_pop: u32, leader: String) -> Self {
        Self {
            new_pop,
            leader,
            id: PhantomId::default(),
        }
    }

    pub fn new_with_id<P>(new_pop: u32, leader: String) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(new_pop, leader))
    }
}

impl NegativelyChargedSteppedLeader<Global> {
    pub fn new<P>(new_pop: u32, leader: String) -> Box<dyn Component<P>>
    where
        P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id(new_pop, leader)
    }
}

impl<P, I> Component<P> for NegativelyChargedSteppedLeader<I>
where
    P: SingleObjectiveProblem + LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, _state: &mut State<P>) -> ExecResult<()> {
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut rng = state.random_mut();

        // Get population from state
        let xs = state.populations_mut().pop();

        // Set center solution
        let mut leader_solution = Vec::new();
        if self.leader.as_str() == "random_new" {
            leader_solution = random_spread(&problem.domain(), 1, &mut *rng)[0].clone();
        } else if self.leader.as_str() == "best" {
            leader_solution = state.best_individual().unwrap().solution().clone();
        } else if self.leader.as_str() == "random_solution" {
            let random_index = &mut rng.gen_range(0..xs.len());
            leader_solution = xs[*random_index].solution().clone();
        } else {
            println!("Invalid leader solution");
        }

        // prepare parameters
        let &Self {
            new_pop, ..
        } = self;

        // Generate new candidate solutions and rate parameters (new_pop specifies how many)
        let mut new_solutions = Vec::new();
        let mut rate_parameters = Vec::new();
        for _ in 0..new_pop {
            let rate_param = problem
                .domain()
                .iter()
                .map(|p| Uniform::from(p.start..p.end).sample(&mut *rng))
                .collect::<Vec<f64>>();
            rate_parameters.push(rate_param.clone());
            
            let new_ind = leader_solution
                .iter()
                .zip(rate_param)
                .map(|(l, p)| l + p)
                .collect::<Vec<f64>>();
            new_solutions.push(new_ind);
        }
        
        // Calculate element-wise mean and std
        // Transpose the data
        let len = xs[0].solution().len();
        let mut transposed: Vec<Vec<f64>> = vec![Vec::new(); len];
        for row in &xs {
            for (i, &val) in row.solution().iter().enumerate() {
                transposed[i].push(val);
            }
        }
        // Compute mean and standard deviation
        let means: Vec<f64> = transposed.iter().map(|v| v.mean()).collect();
        let std_devs: Vec<f64> = transposed.iter().map(|v| v.variance().sqrt()).collect();
        
        // Disperse candidate solutions
        for (i, new_s) in new_solutions.iter_mut().enumerate() {
            izip!(new_s.iter_mut(),
                rate_parameters[i].iter(),
                means.iter(),
                std_devs.iter(),
            ).for_each(|(s, rp, mean, std)| {
                let adjustment = Exp::new(*rp).unwrap().sample(&mut *rng) + Normal::new(*mean, *std).unwrap().sample(&mut *rng);
                if *s < 0.0 {
                    *s -= adjustment;
                } else {
                    *s += adjustment;
                }
            });
        }

        state.populations_mut().push(new_solutions.into_individuals());
        Ok(())
    }
}