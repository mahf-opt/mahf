use std::ops;

use better_any::{Tid, TidAble};
use eyre::ensure;
use rand::distributions::{Distribution, WeightedIndex};
use serde::Serialize;

use crate::{
    component::ExecResult, components::Component, population::IntoIndividuals,
    problems::TravellingSalespersonProblem, state::StateReq, CustomState, State,
};

#[derive(Clone, Tid)]
pub struct PheromoneMatrix {
    dimension: usize,
    inner: Vec<f64>,
}

impl PheromoneMatrix {
    pub fn new(dimension: usize, initial_value: f64) -> Self {
        PheromoneMatrix {
            dimension,
            inner: vec![initial_value; dimension * dimension],
        }
    }
}

impl ops::Index<usize> for PheromoneMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.dimension);
        let start = index * self.dimension;
        let end = start + self.dimension;
        &self.inner[start..end]
    }
}

impl ops::IndexMut<usize> for PheromoneMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.dimension);
        let start = index * self.dimension;
        let end = start + self.dimension;
        &mut self.inner[start..end]
    }
}

impl ops::MulAssign<f64> for PheromoneMatrix {
    fn mul_assign(&mut self, rhs: f64) {
        for x in &mut self.inner {
            *x *= rhs;
        }
    }
}

impl CustomState<'_> for PheromoneMatrix {}

#[derive(Clone, Serialize)]
pub struct AcoGeneration {
    pub num_ants: usize,
    pub alpha: f64,
    pub beta: f64,
    pub default_pheromones: f64,
}

impl AcoGeneration {
    pub fn from_params(num_ants: usize, alpha: f64, beta: f64, default_pheromones: f64) -> Self {
        Self {
            num_ants,
            alpha,
            beta,
            default_pheromones,
        }
    }

    pub fn new<P: TravellingSalespersonProblem>(
        num_ants: usize,
        alpha: f64,
        beta: f64,
        default_pheromones: f64,
    ) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(num_ants, alpha, beta, default_pheromones))
    }
}

impl<P: TravellingSalespersonProblem> Component<P> for AcoGeneration {
    fn init(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(PheromoneMatrix::new(
            problem.dimension(),
            self.default_pheromones,
        ));
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let pm = state.borrow::<PheromoneMatrix>();
        let mut rng = state.random_mut();

        let mut routes = Vec::new();

        // Greedy route
        {
            let mut remaining: Vec<_> = (1..problem.dimension()).collect();
            let mut route = Vec::with_capacity(problem.dimension());
            route.push(0);
            while !remaining.is_empty() {
                let last = *route.last().unwrap();
                let pheromones = remaining.iter().map(|&r| pm[last][r]);
                let next_index = pheromones
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                    .unwrap()
                    .0;
                let next = remaining.remove(next_index);
                route.push(next);
            }
            routes.push(route);
        }

        // Probabilistic routes
        for _ in 0..self.num_ants {
            let mut remaining: Vec<_> = (1..problem.dimension()).collect();
            let mut route = Vec::with_capacity(problem.dimension());
            route.push(0);
            while !remaining.is_empty() {
                let last = *route.last().unwrap();
                let distances = remaining.iter().map(|&r| problem.distance((last, r)));
                let pheromones = remaining.iter().map(|&r| pm[last][r]);
                let weights = pheromones.zip(distances).map(|(m, d)| {
                    // TODO: This should not be zero.
                    m.powf(self.alpha) * (1.0 / d).powf(self.beta) + 1e-15
                });
                let dist = WeightedIndex::new(weights).unwrap();
                let next_index = dist.sample(&mut *rng);
                let next = remaining.remove(next_index);
                route.push(next);
            }
            routes.push(route);
        }

        *state.populations_mut().current_mut() = routes.into_individuals();
        Ok(())
    }
}

#[derive(Clone, Serialize)]
pub struct AsPheromoneUpdate {
    pub evaporation: f64,
    pub decay_coefficient: f64,
}

impl AsPheromoneUpdate {
    pub fn from_params(evaporation: f64, decay_coefficient: f64) -> Self {
        Self {
            evaporation,
            decay_coefficient,
        }
    }

    pub fn new<P: TravellingSalespersonProblem>(
        evaporation: f64,
        decay_coefficient: f64,
    ) -> Box<dyn Component<P>> {
        Box::new(Self::from_params(evaporation, decay_coefficient))
    }
}

impl<P: TravellingSalespersonProblem> Component<P> for AsPheromoneUpdate {
    fn require(&self, _problem: &P, state_req: &StateReq) -> ExecResult<()> {
        state_req.require::<Self, PheromoneMatrix>()?;
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let populations = state.populations();
        let mut pm = state.borrow_mut::<PheromoneMatrix>();

        // Evaporation
        *pm *= 1.0 - self.evaporation;

        // Update pheromones for probabilistic routes
        for individual in populations.current().iter().skip(1) {
            let objective = individual.objective().value();
            let route = individual.solution();
            let delta = self.decay_coefficient / objective;
            for (&a, &b) in route.iter().zip(route.iter().skip(1)) {
                pm[a][b] += delta;
                pm[b][a] += delta;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Serialize)]
pub struct MinMaxPheromoneUpdate {
    pub evaporation: f64,
    pub max_pheromones: f64,
    pub min_pheromones: f64,
}

impl MinMaxPheromoneUpdate {
    pub fn from_params(
        evaporation: f64,
        max_pheromones: f64,
        min_pheromones: f64,
    ) -> ExecResult<Self> {
        ensure!(
            min_pheromones < max_pheromones,
            "min_pheromones must be less than max_pheromones"
        );
        Ok(Self {
            evaporation,
            max_pheromones,
            min_pheromones,
        })
    }

    pub fn new<P: TravellingSalespersonProblem>(
        evaporation: f64,
        max_pheromones: f64,
        min_pheromones: f64,
    ) -> ExecResult<Box<dyn Component<P>>> {
        Ok(Box::new(Self::from_params(
            evaporation,
            max_pheromones,
            min_pheromones,
        )?))
    }
}

impl<P: TravellingSalespersonProblem> Component<P> for MinMaxPheromoneUpdate {
    fn require(&self, _problem: &P, state_req: &StateReq) -> ExecResult<()> {
        state_req.require::<Self, PheromoneMatrix>()?;
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let populations = state.populations();
        let mut pm = state.borrow_mut::<PheromoneMatrix>();

        // Evaporation
        *pm *= 1.0 - self.evaporation;

        // Update pheromones for probabilistic routes
        let individual = populations
            .current()
            .iter()
            .skip(1)
            .min_by_key(|i| i.objective())
            .unwrap();

        let fitness = individual.objective().value();
        let route = individual.solution();
        let delta = 1.0 / fitness;
        for (&a, &b) in route.iter().zip(route.iter().skip(1)) {
            pm[a][b] = (pm[a][b] + delta).clamp(self.min_pheromones, self.max_pheromones);
            pm[b][a] = (pm[b][a] + delta).clamp(self.min_pheromones, self.max_pheromones);
        }

        Ok(())
    }
}
