//! Repair solutions for violating boundary constraints.
//!
//! # References
//!
//! \[1\] Anna V. Kononova, Fabio Caraffini, and Thomas Bäck. 2021.
//! Differential evolution outside the box. Information Sciences 581, (December 2021), 587–604. DOI:<https://doi.org/10/grsff3>

use itertools::izip;
use rand::distributions::Distribution;
use rand_distr::Normal;
use serde::{Deserialize, Serialize};

use crate::{
    component::{AnyComponent, ExecResult},
    components::Component,
    population::AsSolutionsMut,
    problems::LimitedVectorProblem,
    state::{random::Random, State},
    Problem,
};

pub trait BoundaryConstraint<P: Problem>: AnyComponent {
    fn constrain(&self, solution: &mut P::Encoding, problem: &P, rng: &mut Random);
}

erased_serde::serialize_trait_object!(<P: Problem> BoundaryConstraint<P>);
dyn_clone::clone_trait_object!(<P: Problem> BoundaryConstraint<P>);

pub fn boundary_constraint<P, T>(component: &T, problem: &P, state: &mut State<P>) -> ExecResult<()>
where
    P: Problem,
    T: BoundaryConstraint<P>,
{
    let mut populations = state.populations_mut();
    for solution in populations.current_mut().as_solutions_mut() {
        component.constrain(solution, problem, &mut state.random_mut());
    }
    Ok(())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Saturation;

impl Saturation {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P> BoundaryConstraint<P> for Saturation
where
    P: LimitedVectorProblem<Element = f64>,
{
    fn constrain(&self, solution: &mut P::Encoding, problem: &P, _rng: &mut Random) {
        for (x, range) in izip!(solution, problem.domain()) {
            *x = x.clamp(range.start, range.end);
        }
    }
}

impl<P> Component<P> for Saturation
where
    P: LimitedVectorProblem<Element = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        boundary_constraint(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Toroidal;

impl Toroidal {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P> BoundaryConstraint<P> for Toroidal
where
    P: LimitedVectorProblem<Element = f64>,
{
    fn constrain(&self, solution: &mut P::Encoding, problem: &P, _rng: &mut Random) {
        for (x, range) in izip!(solution, problem.domain()) {
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

impl<P> Component<P> for Toroidal
where
    P: LimitedVectorProblem<Element = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        boundary_constraint(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Mirror;

impl Mirror {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: Problem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P> BoundaryConstraint<P> for Mirror
where
    P: LimitedVectorProblem<Element = f64>,
{
    fn constrain(&self, solution: &mut P::Encoding, problem: &P, _rng: &mut Random) {
        for (x, range) in izip!(solution, problem.domain()) {
            let a = range.start;
            let b = range.end;

            let mut x_temp = *x;
            while !range.contains(&x_temp) {
                x_temp = match x_temp {
                    v if v < a => a + (a - v),
                    v if v > b => b - (v - b),
                    v => v,
                };
            }

            *x = x_temp;
        }
    }
}

impl<P> Component<P> for Mirror
where
    P: LimitedVectorProblem<Element = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        boundary_constraint(self, problem, state)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CompleteOneTailedNormalCorrection;

impl CompleteOneTailedNormalCorrection {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: Problem + LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params())
    }
}

impl<P> BoundaryConstraint<P> for CompleteOneTailedNormalCorrection
where
    P: LimitedVectorProblem<Element = f64>,
{
    fn constrain(&self, solution: &mut P::Encoding, problem: &P, rng: &mut Random) {
        for (x, range) in izip!(solution, problem.domain()) {
            let a = range.start;
            let b = range.end;

            let dist = Normal::new(0., (b - a) / 3.).unwrap();

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

impl<P> Component<P> for CompleteOneTailedNormalCorrection
where
    P: LimitedVectorProblem<Element = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        boundary_constraint(self, problem, state)
    }
}
