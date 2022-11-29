use crate::{
    framework::components::AnyComponent,
    problems::{LimitedVectorProblem, Problem, VectorProblem},
};

pub trait BoundaryConstraint<P: Problem>: AnyComponent {
    fn constrain(&self, problem: &P, solution: &mut P::Encoding);
}

#[derive(serde::Serialize, Clone)]
pub struct Saturation;
impl Saturation {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>(
    ) -> Box<dyn BoundaryConstraint<P>> {
        Box::new(Self)
    }
}
impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>
    BoundaryConstraint<P> for Saturation
{
    fn constrain(&self, problem: &P, solution: &mut P::Encoding) {
        for (d, x) in solution.iter_mut().enumerate() {
            let range = problem.range(d);
            *x = x.clamp(range.start, range.end);
        }
    }
}

#[derive(serde::Serialize, Clone)]
pub struct Toroidal;
impl Toroidal {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>(
    ) -> Box<dyn BoundaryConstraint<P>> {
        Box::new(Self)
    }
}
impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>
    BoundaryConstraint<P> for Toroidal
{
    fn constrain(&self, problem: &P, solution: &mut P::Encoding) {
        for (d, x) in solution.iter_mut().enumerate() {
            let range = problem.range(d);
            let delta = range.end - range.start;
            *x = match *x {
                // Note that, e.g., with a bound of [-5, 5], 15 is mirrored to -5, and not 5, because of the mod operation.
                v if v > range.end => range.start + (v - range.end) % delta,
                v if v < range.start => range.end + (v + range.start) % delta,
                v => v,
            };
        }
    }
}

#[derive(serde::Serialize, Clone)]
pub struct Mirror;
impl Mirror {
    pub fn new<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>(
    ) -> Box<dyn BoundaryConstraint<P>> {
        Box::new(Self)
    }
}
impl<P: Problem<Encoding = Vec<f64>> + VectorProblem<T = f64> + LimitedVectorProblem>
    BoundaryConstraint<P> for Mirror
{
    fn constrain(&self, problem: &P, solution: &mut P::Encoding) {
        for (d, x) in solution.iter_mut().enumerate() {
            let range = problem.range(d);
            let delta = range.end - range.start;
            *x = match *x {
                v if v > range.end => range.end - (v - range.end) % delta,
                v if v < range.start => range.start - (v + range.start) % delta,
                v => v,
            };
        }
    }
}
