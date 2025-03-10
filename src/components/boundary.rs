//! Repair solutions for violating boundary constraints.
//!
//! # References
//!
//! \[1\] Anna V. Kononova, Fabio Caraffini, and Thomas Bäck. 2021.
//! Differential evolution outside the box. Information Sciences 581, (December 2021), 587–604.
//! DOI:<https://doi.org/10/grsff3>

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

/// Trait for representing a component that repairs solutions that violate boundary constraints.
pub trait BoundaryConstraint<P: Problem>: AnyComponent {
    /// Repairs the `solution` such that it no longer violates any boundary constraints.
    fn constrain(&self, solution: &mut P::Encoding, problem: &P, rng: &mut Random);
}

/// A default implementation of [`Component::execute`] for types implementing [`BoundaryConstraint`].
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

/// Clamps the values to the domain boundaries.
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

/// Reflects values outside the domain off the opposite domain boundary inwards,
/// as if the boundaries are connected and the domain forms a ring.
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

/// The amount exceeding the boundary is reflected inwards at the same boundary.
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

/// Re-samples the values outside the bounds.
///
/// Given the bounds \[a, b\], it will resample from
/// - `a + P(a, b)` for the lower bound,
/// - `b - P(a, b)` for the upper bound,
///
/// where `P(a, b) ~ |N(0, (b - a)/3)|`.
///
/// Re-sampling is performed until the value is within the domain.
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

/// The amount exceeding the boundary is relocated inwards.
/// 
/// Given the bounds \[a, b\], the relocation is performed as `a + (b - a) cos(x)`
#[derive(Clone, Serialize, Deserialize)]
pub struct CosineCorrection;

impl CosineCorrection {
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

impl<P> BoundaryConstraint<P> for CosineCorrection
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
                    v if v < a || v > b => a + (b - a) * v.cos(),
                    v => v,
                };
            }

            *x = x_temp;
        }
    }
}

impl<P> Component<P> for CosineCorrection
where
    P: LimitedVectorProblem<Element = f64>,
{
    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        boundary_constraint(self, problem, state)
    }
}