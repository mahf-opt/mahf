//! Common mutation components.

use std::marker::PhantomData;

use eyre::{ensure, WrapErr};
use itertools::multizip;
use rand::{
    distributions::{Distribution, Uniform},
    seq::{IteratorRandom, SliceRandom},
    Rng,
};
use rand_distr::Normal;
use serde::{Deserialize, Serialize};

use crate::{
    component::ExecResult,
    components::{
        mutation::{functional as f, MutationRate, MutationStrength},
        Component,
    },
    identifier::{Global, Identifier},
    population::AsSolutionsMut,
    problems::{LimitedVectorProblem, VectorProblem},
    State,
};

/// Mutates each dimension with a delta from a normal distribution `N(0, std_dev)`
/// depending on the mutation probability `rm`.
///
/// # Adapting parameters
///
/// Adapting the `std_dev` and `rm` is possible through modifying the respective states:
/// - `std_dev`: [`MutationStrength<NormalMutation<I>>`]
/// - `rm`: [`MutationRate<NormalMutation<I>>`]
///
/// # Errors
///
/// Returns an `Err` if the [`MutationStrength`] or [`MutationRate`] contain invalid values.
#[derive(Clone, Serialize, Deserialize)]
pub struct NormalMutation<I: Identifier = Global> {
    /// Standard deviation of the normal distribution.
    pub std_dev: f64,
    /// Mutation rate.
    pub rm: f64,
    phantom: PhantomData<I>,
}

impl<I: Identifier> NormalMutation<I> {
    pub fn from_params(std_dev: f64, rm: f64) -> Self {
        Self {
            std_dev,
            rm,
            phantom: PhantomData,
        }
    }

    pub fn new_with_id<P>(std_dev: f64, rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(std_dev, rm))
    }
}

impl NormalMutation<Global> {
    pub fn new<P>(std_dev: f64, rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Self::new_with_id(std_dev, rm)
    }

    /// Creates the `NormalMutation` with a mutation rate of 1.
    pub fn new_dev<P>(std_dev: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Self::new(std_dev, 1.0)
    }
}

impl<P, I> Component<P> for NormalMutation<I>
where
    P: VectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(MutationStrength::<Self>::new(self.std_dev));
        state.insert(MutationRate::<Self>::new(self.rm));
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let distr = Normal::new(0., state.get_value::<MutationStrength<Self>>())
            .wrap_err("invalid mutation strength")?;

        let rm = state.borrow::<MutationRate<Self>>().value()?;

        for solution in populations.current_mut().as_solutions_mut() {
            for x in solution {
                if rng.gen_bool(rm) {
                    *x += distr.sample(&mut *rng);
                }
            }
        }
        Ok(())
    }
}

/// Mutates each dimension with a delta from a uniform distribution `[-bound, bound]`
/// depending on the mutation probability `rm`.
///
/// # Adapting parameters
///
/// Adapting the `bound` and `rm` is possible through modifying the respective states:
/// - `bound`: [`MutationStrength<UniformMutation<I>>`]
/// - `rm`: [`MutationRate<UniformMutation<I>>`]
///
/// # Errors
///
/// Returns an `Err` if the [`MutationStrength`] or [`MutationRate`] contain invalid values.
#[derive(Clone, Serialize, Deserialize)]
pub struct UniformMutation<I: Identifier = Global> {
    /// Bound of the uniform distribution `[-bound, bound]`.
    pub bound: f64,
    /// Mutation rate.
    pub rm: f64,
    phantom: PhantomData<I>,
}

impl<I: Identifier> UniformMutation<I> {
    pub fn from_params(bound: f64, rm: f64) -> Self {
        Self {
            bound,
            rm,
            phantom: PhantomData,
        }
    }

    pub fn new_with_id<P>(bound: f64, rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(bound, rm))
    }
}

impl UniformMutation<Global> {
    pub fn new<P>(bound: f64, rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Self::new_with_id(bound, rm)
    }

    pub fn new_bound<P>(bound: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = f64>,
    {
        Self::new(bound, 1.0)
    }
}

impl<P, I> Component<P> for UniformMutation<I>
where
    P: VectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(MutationStrength::<Self>::new(self.bound));
        state.insert(MutationRate::<Self>::new(self.rm));
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let bound = state.get_value::<MutationStrength<Self>>();
        ensure!(bound >= 0., "bound must be positive");
        let distr = Uniform::new(0., bound);

        let rm = state.borrow::<MutationRate<Self>>().value()?;

        for solution in populations.current_mut().as_solutions_mut() {
            for x in solution {
                if rng.gen_bool(rm) {
                    let sign = *[-1., 1.].choose(&mut *rng).unwrap();
                    *x += sign * distr.sample(&mut *rng);
                }
            }
        }
        Ok(())
    }
}

/// Flips each bit with probability `rm`.
///
/// # Adapting parameters
///
/// Adapting the `rm` is possible through modifying the respective state:
/// - `rm`: [`MutationRate<BitFlipMutation<I>>`]
///
/// # Errors
///
/// Returns an `Err` if the [`MutationRate`] contains an invalid value.
#[derive(Clone, Serialize, Deserialize)]
pub struct BitFlipMutation<I: Identifier = Global> {
    /// Probability of flipping a bit.
    pub rm: f64,
    phantom: PhantomData<I>,
}

impl<I: Identifier> BitFlipMutation<I> {
    pub fn from_params(rm: f64) -> Self {
        Self {
            rm,
            phantom: PhantomData,
        }
    }

    pub fn new_with_id<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = bool>,
    {
        Box::new(Self::from_params(rm))
    }
}

impl BitFlipMutation<Global> {
    pub fn new<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = bool>,
    {
        Self::new_with_id(rm)
    }
}

impl<P, I> Component<P> for BitFlipMutation<I>
where
    P: VectorProblem<Element = bool>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(MutationRate::<Self>::new(self.rm));
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let rm = state.borrow::<MutationRate<Self>>().value()?;

        for solution in populations.current_mut().as_solutions_mut() {
            for x in solution {
                if rng.gen_bool(rm) {
                    *x = !*x;
                }
            }
        }
        Ok(())
    }
}

/// Applies a random uniform reset of a position in the solution with probability `rm`.
///
/// # Adapting parameters
///
/// Adapting the `rm` is possible through modifying the respective state:
/// - `rm`: [`MutationRate<PartialRandomSpread<I>>`]
///
/// # Errors
///
/// Returns an `Err` if the [`MutationRate`] contains an invalid value.
#[derive(Clone, Serialize, Deserialize)]
pub struct PartialRandomSpread<I: Identifier = Global> {
    /// Mutation rate.
    pub rm: f64,
    phantom: PhantomData<I>,
}

impl<I: Identifier> PartialRandomSpread<I> {
    pub fn from_params(rm: f64) -> Self {
        Self {
            rm,
            phantom: PhantomData,
        }
    }

    pub fn new_with_id<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
    {
        Box::new(Self::from_params(rm))
    }
}

impl PartialRandomSpread<Global> {
    pub fn new<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
    {
        Self::new_with_id(rm)
    }

    /// Creates a new `PartialRandomSpread` which fully re-initializes the population in the search space.
    pub fn new_full<P>() -> Box<dyn Component<P>>
    where
        P: LimitedVectorProblem<Element = f64>,
    {
        Self::new(1.)
    }
}

impl<P, I> Component<P> for PartialRandomSpread<I>
where
    P: LimitedVectorProblem<Element = f64>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(MutationRate::<Self>::new(self.rm));
        Ok(())
    }

    fn execute(&self, problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let rm = state.borrow::<MutationRate<Self>>().value()?;

        for solution in populations.current_mut().as_solutions_mut() {
            for (x, domain) in multizip((solution, problem.domain())) {
                if rng.gen_bool(rm) {
                    *x = rng.gen_range(domain.clone());
                }
            }
        }
        Ok(())
    }
}

/// Applies a scramble mutation i.e. shuffling with probability `rm`.
///
/// # Adapting parameters
///
/// Adapting the `rm` is possible through modifying the respective state:
/// - `rm`: [`MutationRate<ScrambleMutation<I>>`]
///
/// # Errors
///
/// Returns an `Err` if the [`MutationRate`] contains an invalid value.
#[derive(Clone, Serialize, Deserialize)]
pub struct ScrambleMutation<I: Identifier = Global> {
    pub rm: f64,
    phantom: PhantomData<I>,
}

impl<I: Identifier> ScrambleMutation<I> {
    pub fn from_params(rm: f64) -> Self {
        Self {
            rm,
            phantom: PhantomData,
        }
    }

    pub fn new_with_id<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem,
        P::Element: 'static,
    {
        Box::new(Self::from_params(rm))
    }
}

impl ScrambleMutation<Global> {
    pub fn new<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem,
        P::Element: 'static,
    {
        Self::new_with_id(rm)
    }

    /// Creates a new `ScrambleMutation` which scrambles all solutions.
    pub fn new_full<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem,
        P::Element: 'static,
    {
        Self::new(1.)
    }
}

impl<P, I> Component<P> for ScrambleMutation<I>
where
    P: VectorProblem,
    P::Element: 'static,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(MutationRate::<Self>::new(self.rm));
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let rm = state.borrow::<MutationRate<Self>>().value()?;

        for solution in populations.current_mut().as_solutions_mut() {
            if rng.gen_bool(rm) {
                solution.shuffle(&mut *rng);
            }
        }
        Ok(())
    }
}

/// Re-samples each bit depending on the mutation rate `rm`, where `p` is the probability
/// of generating a 1 or `true`.
///
/// # Adapting parameters
///
/// Adapting the `rm` is possible through modifying the respective state:
/// - `rm`: [`MutationRate<PartialRandomBitstring<I>>`]
///
/// # Errors
///
/// Returns an `Err` if the [`MutationRate`] contains an invalid value.
#[derive(Clone, Serialize, Deserialize)]
pub struct PartialRandomBitstring<I: Identifier = Global> {
    /// Probability to sample 1 or `true`.
    pub p: f64,
    /// Mutation rate.
    pub rm: f64,
    phantom: PhantomData<I>,
}

impl<I: Identifier> PartialRandomBitstring<I> {
    pub fn from_params(p: f64, rm: f64) -> Self {
        Self {
            p,
            rm,
            phantom: PhantomData,
        }
    }

    pub fn new_with_id<P>(p: f64, rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = bool>,
    {
        Box::new(Self::from_params(p, rm))
    }
}

impl PartialRandomBitstring<Global> {
    pub fn new<P>(p: f64, rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = bool>,
    {
        Self::new_with_id(p, rm)
    }

    pub fn new_uniform<P>(rm: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = bool>,
    {
        Self::new(0.5, rm)
    }

    pub fn new_full<P>(p: f64) -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = bool>,
    {
        Self::new(p, 1.)
    }

    pub fn new_uniform_full<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem<Element = bool>,
    {
        Self::new(0.5, 1.)
    }
}

impl<P, I> Component<P> for PartialRandomBitstring<I>
where
    P: VectorProblem<Element = bool>,
    I: Identifier,
{
    fn init(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        state.insert(MutationRate::<Self>::new(self.rm));
        Ok(())
    }

    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let rm = state.borrow::<MutationRate<Self>>().value()?;

        for solution in populations.current_mut().as_solutions_mut() {
            for x in solution {
                if rng.gen_bool(rm) {
                    *x = rng.gen_bool(self.p);
                }
            }
        }
        Ok(())
    }
}

/// Applies a swap mutation to `num_swap` elements.
///
/// For more than two elements the swap is performed circular.
///
/// # Errors
///
/// Returns an `Err` if `num_swap` is greater than the solution length.
#[derive(Clone, Serialize, Deserialize)]
pub struct SwapMutation {
    pub num_swap: u32,
}

impl SwapMutation {
    pub fn from_params(num_swap: u32) -> ExecResult<Self> {
        ensure!(
            num_swap > 2,
            "at least two indices need to be swapped, while {} was provided",
            num_swap
        );
        Ok(Self { num_swap })
    }

    /// Creates a new `SwapMutation`, or returns an `Err` if `num_swap` is less than two,
    /// as it is not possible to swap less than two elements.
    pub fn new<P>(num_swap: u32) -> ExecResult<Box<dyn Component<P>>>
    where
        P: VectorProblem,
        P::Element: 'static,
    {
        Ok(Box::new(Self::from_params(num_swap)?))
    }
}

impl<P> Component<P> for SwapMutation
where
    P: VectorProblem,
    P::Element: 'static,
{
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        let num_swap = self.num_swap as usize;

        for solution in populations.current_mut().as_solutions_mut() {
            ensure!(
                num_swap < solution.len(),
                "more than {} swaps are not possible on a solution of length {}",
                num_swap,
                solution.len()
            );
            let indices = (0..solution.len()).choose_multiple(&mut *rng, num_swap);
            f::circular_swap(solution, &indices)
        }
        Ok(())
    }
}

/// Applies a inversion mutation to the solution, i.e. taking a random slice of the solution
/// and inverting it.
#[derive(Clone, Serialize, Deserialize)]
pub struct InversionMutation;

impl InversionMutation {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P, D>() -> Box<dyn Component<P>>
    where
        P: VectorProblem,
        P::Element: 'static,
    {
        Box::new(Self::from_params())
    }
}

impl<P> Component<P> for InversionMutation
where
    P: VectorProblem,
    P::Element: 'static,
{
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        for solution in populations.current_mut().as_solutions_mut() {
            let [start, end]: [_; 2] = (0..solution.len())
                .choose_multiple(&mut *state.random_mut(), 2)
                .try_into()
                .unwrap();
            solution[start..end].reverse();
        }
        Ok(())
    }
}

/// Applies a insertion mutation to the solution, i.e. removing a random element
/// from the solution and inserting it on a random position.
#[derive(Clone, Serialize, Deserialize)]
pub struct InsertionMutation;

impl InsertionMutation {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem,
        P::Element: 'static,
    {
        Box::new(Self::from_params())
    }
}

impl<P> Component<P> for InsertionMutation
where
    P: VectorProblem,
    P::Element: 'static,
{
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        for solution in populations.current_mut().as_solutions_mut() {
            let element = rng.gen_range(0..solution.len());
            let index = rng.gen_range(0..solution.len());
            f::translocate_slice(solution, element..element + 1, index);
        }
        Ok(())
    }
}

/// Applies a translocation mutation to the solution, i.e. removing a random slice
/// from solution and inserting it on a random position.
#[derive(Clone, Serialize, Deserialize)]
pub struct TranslocationMutation;

impl TranslocationMutation {
    pub fn from_params() -> Self {
        Self
    }

    pub fn new<P>() -> Box<dyn Component<P>>
    where
        P: VectorProblem,
        P::Element: 'static,
    {
        Box::new(Self::from_params())
    }
}

impl<P> Component<P> for TranslocationMutation
where
    P: VectorProblem,
    P::Element: 'static,
{
    fn execute(&self, _problem: &P, state: &mut State<P>) -> ExecResult<()> {
        let mut populations = state.populations_mut();
        let mut rng = state.random_mut();

        for solution in populations.current_mut().as_solutions_mut() {
            let [start, end]: [_; 2] = (0..solution.len())
                .choose_multiple(&mut *state.random_mut(), 2)
                .try_into()
                .unwrap();
            let index = rng.gen_range(0..start);
            f::translocate_slice(solution, start..end, index);
        }
        Ok(())
    }
}
