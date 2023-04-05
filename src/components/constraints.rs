use rand::prelude::*;
use serde::Serialize;

use crate::{
    components::Component,
    framework::AnyComponent,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
    state::State,
};

/// Specialized component trait to constrain solutions to a domain.
///
/// # Implementing [Component]
///
/// Types implementing this trait can implement [Component] by wrapping the type in a [BoundaryConstrainer].
pub trait BoundaryConstraint<P: Problem> {
    fn constrain(&self, solution: &mut P::Encoding, problem: &P, state: &mut State<P>);
}

#[derive(serde::Serialize, Clone)]
pub struct BoundaryConstrainer<T: Clone>(pub T);

impl<T, P> Component<P> for BoundaryConstrainer<T>
where
    P: Problem,
    T: AnyComponent + BoundaryConstraint<P> + Serialize + Clone,
{
    fn execute(&self, problem: &P, state: &mut State<P>) {
        let mut population = state.populations_mut().pop();
        for individual in population.iter_mut() {
            self.0.constrain(individual.solution_mut(), problem, state);
        }
        state.populations_mut().push(population);
    }
}

/// Clamps the values to the domain boundaries.
#[derive(serde::Serialize, Clone)]
pub struct Saturation;

impl Saturation {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>(
    ) -> Box<dyn Component<P>> {
        Box::new(BoundaryConstrainer(Self))
    }
}

impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>
    BoundaryConstraint<P> for Saturation
{
    fn constrain(&self, solution: &mut Vec<f64>, problem: &P, _state: &mut State<P>) {
        for (d, x) in solution.iter_mut().enumerate() {
            let range = problem.range(d);
            *x = x.clamp(range.start, range.end);
        }
    }
}

/// Reflects values outside the domain off the opposite domain boundary inwards,
/// as if the boundaries are connected and the domain forms a ring.
#[derive(serde::Serialize, Clone)]
pub struct Toroidal;

impl Toroidal {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>(
    ) -> Box<dyn Component<P>> {
        Box::new(BoundaryConstrainer(Self))
    }
}

impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>
    BoundaryConstraint<P> for Toroidal
{
    fn constrain(&self, solution: &mut Vec<f64>, problem: &P, _state: &mut State<P>) {
        for (d, x) in solution.iter_mut().enumerate() {
            let range = problem.range(d);
            let a = range.start;
            let b = range.end;
            let d = b - a;
            let norm = (*x - a) / d;

            *x = match *x {
                v if v < range.start => a + (1. - (norm - norm.floor()).abs()) * d,
                v if v > range.end => a + (norm - norm.floor()) * d,
                v => v,
            };
        }
    }
}

/// The amount exceeding the boundary is reflected inwards at the same boundary.
#[derive(serde::Serialize, Clone)]
pub struct Mirror;

impl Mirror {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>(
    ) -> Box<dyn Component<P>> {
        Box::new(BoundaryConstrainer(Self))
    }
}

impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>
    BoundaryConstraint<P> for Mirror
{
    fn constrain(&self, solution: &mut Vec<f64>, problem: &P, _state: &mut State<P>) {
        for (d, x) in solution.iter_mut().enumerate() {
            let range = problem.range(d);
            let a = range.start;
            let b = range.end;

            let mut new_x = *x;
            while !range.contains(&new_x) {
                new_x = match new_x {
                    v if v < a => a + (a - v),
                    v if v > b => b - (v - b),
                    v => v,
                };
            }

            *x = new_x;
        }
    }
}

/// The values outside the bounds \[a, b\] are re-sampled from
/// - `a + |N(0, (b - a)/3)|` for the lower bound,
/// - `b - |N(0, (b - a)/3)|` for the upper bound.
///
/// Re-sampling is performed until the value is within the domain.
#[derive(serde::Serialize, Clone)]
pub struct CompleteOneTailedNormalCorrection;

impl CompleteOneTailedNormalCorrection {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>(
    ) -> Box<dyn Component<P>> {
        Box::new(BoundaryConstrainer(Self))
    }
}

impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>
    BoundaryConstraint<P> for CompleteOneTailedNormalCorrection
{
    fn constrain(&self, solution: &mut Vec<f64>, problem: &P, state: &mut State<P>) {
        for (d, x) in solution.iter_mut().enumerate() {
            let range = problem.range(d);
            let a = range.start;
            let b = range.end;

            let rng = state.random_mut();
            let dist = rand_distr::Normal::new(0., (b - a) / 3.).unwrap();

            let mut new_x = *x;
            while !range.contains(&new_x) {
                new_x = match new_x {
                    v if v < a => a + dist.sample(rng).abs(),
                    v if v > b => b - dist.sample(rng).abs(),
                    v => v,
                }
            }

            *x = new_x;
        }
    }
}
