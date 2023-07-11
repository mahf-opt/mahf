//! Optimization problems.
//!
//! The `problems` module provides traits and structures for defining and working with
//! optimization problems. An optimization problem involves finding the best solution
//! from a set of possible solutions, where the quality of a solution is determined by
//! an objective function.
//!
//! ## Minimization
//!
//! In MAHF, "optimizing" is currently equivalent to "minimizing", i.e. finding the solution
//! that minimizes some objective function.
//!
//! # Key Concepts
//!
//! - `Problem`: The [`Problem`] trait (hierarchy) provides information about the problem
//! to components. Traits built upon [`Problem`] (e.g. [`VectorProblem`]) allow making
//! *any* information accessible to components, while being as generic as possible.
//! - `Individual`: An [`Individual`] is an encoded solution to the problem along with an associated
//! (optional) objective value, which qualifies how "good" of a solution it is to the problem.
//! In single-objective optimization, this objective value is also referred to as "fitness".
//! - `Evaluate`: The [`Evaluate`] trait allows evaluating [`Individual`]s according to some
//! objective function.
//!
//! # Usage
//!
//! ## Pre-implemented problems
//!
//! There exist several pre-implemented problems in the [MAHF ecosystem], which provide a good
//! starting point.
//!
//! [MAHF ecosystem]: https://github.com/mahf-opt#problems-libraries
//!
//! ## Implement custom problems
//!
//! To define your own optimization problem, the minimum requirement is to implement
//! [`Problem`] and some sort of evaluator for it, i.e. implement [`Evaluate`] for
//! some struct.
//!
//! Note that for most problems with a *non-mutable* objective function,
//! the [`ObjectiveFunction`] trait should be preferred over [`Evaluate`].
//! See [`ObjectiveFunction`] for more information.
//!
//! Then implement the traits built on top of [`Problem`] found in this module where sensible,
//! e.g. [`VectorProblem`] for an optimization problem with a vector-based solution encoding.
//! You can similarly define own traits based on [`Problem`] to allow your custom components
//! to access any problem-specific information.

use std::ops::Range;

use trait_set::trait_set;

pub mod encoding;
pub mod evaluate;
pub mod individual;
pub mod objective;

pub use encoding::AnyEncoding;
pub use evaluate::{Evaluate, ObjectiveFunction, Parallel, Sequential};
pub use individual::Individual;
pub use objective::{MultiObjective, Objective, SingleObjective};

/// An optimization (minimization) problem.
///
/// This trait is the base trait for all problems, and itself only defines
/// - a encoding to solutions to the problem ([`Problem::Encoding`]),
/// - the type of objective to minimize ([`Problem::Objective`]), and
/// - the name of the problem ([`Problem::name`]).
///
/// # Problem-specific information
///
/// `Problem` (along with traits that build upon it, e.g. [`VectorProblem`]) make problem-specific
/// information accessible to components, and should only provide exactly as much information
/// as the components need to function.
///
/// [`Component`]s are generic over the problem type `P`, and adding traits bounds to `P`
/// symbolizes that the [`Component`] requires the information that the traits offer.
/// For example, a component that only works on problems with a single objective adds
/// a `P: `[`SingleObjective`] trait bound to its [`Component`] implementation.
///
///
/// [`Component`]: crate::Component
///
/// # Examples
///
/// A simple implementation of the real-valued sphere function `f(x) = x^2`:
///
/// ```
/// use mahf::{Problem, SingleObjective};
///
/// pub struct Sphere {
///     pub dim: usize,
/// }
///
/// impl Problem for Sphere {
///     type Encoding = Vec<f64>; // Real-valued vector
///     type Objective = SingleObjective;
///
///     fn name(&self) -> &str {
///         "Sphere"
///     }
/// }
/// ```
pub trait Problem: 'static {
    /// The encoding of a solution to the optimization problem (genotype).
    type Encoding: AnyEncoding;

    /// The objective type to minimize.
    ///
    /// See [`SingleObjective`] and [`MultiObjective`] for the default options.
    type Objective: Objective;

    /// The name of the optimization problem.
    fn name(&self) -> &str;
}

trait_set! {
    /// An optimization problem with a single objective or multiple combined objectives.
    ///
    /// This trait should be used in favor over specifying the objective type
    /// of [`Problem`] directly, and is automatically implemented for all problems
    /// with [`SingleObjective`] as [`Problem::Objective`].
    pub trait SingleObjectiveProblem = Problem<Objective = SingleObjective>;

    /// An optimization problem with multiple distinct objectives.
    ///
    /// This trait should be used in favor over specifying the objective type
    /// of [`Problem`] directly, and is automatically implemented for all problems
    /// with [`MultiObjective`] as [`Problem::Objective`].
    pub trait MultiObjectiveProblem = Problem<Objective = MultiObjective>;
}

/// A vector-based optimization problem.
///
/// This trait extends the [`Problem`] trait and represents an optimization problem
/// whose solutions are encoded as vectors with some [`Element`] type.
///
/// [`Element`]: VectorProblem::Element
///
/// # Examples
///
/// A simple implementation of the n-dimensional real-valued sphere function `f(x) = x^2`:
///
/// ```
/// use mahf::{problems::VectorProblem, Problem, SingleObjective};
///
/// pub struct Sphere {
///     pub dim: usize,
/// }
///
/// impl Problem for Sphere {
///     type Encoding = Vec<f64>;
///     type Objective = SingleObjective;
///
///     fn name(&self) -> &str {
///         "Sphere"
///     }
/// }
///
/// impl VectorProblem for Sphere {
///     type Element = f64;
///
///     fn dimension(&self) -> usize {
///         self.dim
///     }
/// }
/// ```
pub trait VectorProblem: Problem<Encoding = Vec<Self::Element>> {
    /// The element type of the vector encoding the solutions.
    type Element: Clone;

    /// Returns the dimension of the optimization problem.
    ///
    /// The dimension represents the length of the vector encoding the solutions.
    fn dimension(&self) -> usize;
}

/// A vector-based optimization problem with limited search space.
///
/// This trait extends the [`VectorProblem`] trait and represents an optimization problem
/// whose solutions are encoded as vectors with a limited search space defined by some domain.
///
/// # Examples
///
/// A simple implementation of the n-dimensional real-valued sphere function `f(x) = x^2`
/// restricted to the \[-1, 1) domain on each dimension:
///
/// ```
/// use std::ops::{Range, RangeInclusive};
///
/// use mahf::{
///     problems::{LimitedVectorProblem, VectorProblem},
///     Problem, SingleObjective,
/// };
///
/// pub struct Sphere {
///     pub dim: usize,
/// }
///
/// impl Problem for Sphere {
///     type Encoding = Vec<f64>;
///     type Objective = SingleObjective;
///
///     fn name(&self) -> &str {
///         "Sphere"
///     }
/// }
///
/// impl VectorProblem for Sphere {
///     type Element = f64;
///
///     fn dimension(&self) -> usize {
///         self.dim
///     }
/// }
///
/// impl LimitedVectorProblem for Sphere {
///     fn domain(&self) -> Vec<Range<Self::Element>> {
///         std::iter::repeat(-1.0..1.0).take(self.dim).collect()
///     }
/// }
/// ```
pub trait LimitedVectorProblem: VectorProblem {
    /// Returns the bounds of the search space for each element in the vector encoding.
    ///
    /// The bounds specify the range of valid values for each element in the vector.
    fn domain(&self) -> Vec<Range<Self::Element>>;
}

/// A single-objective optimization problem with a known optimum value.
///
/// # Examples
///
/// A simple implementation of the real-valued sphere function `f(x) = x^2`, where
/// the optimum is known to be 0.
///
/// ```
/// use std::ops::Range;
///
/// use mahf::{problems::KnownOptimumProblem, Problem, SingleObjective};
///
/// pub struct Sphere {
///     pub dim: usize,
/// }
///
/// impl Problem for Sphere {
///     type Encoding = Vec<f64>;
///     type Objective = SingleObjective;
///
///     fn name(&self) -> &str {
///         "Sphere"
///     }
/// }
///
/// impl KnownOptimumProblem for Sphere {
///     fn known_optimum(&self) -> SingleObjective {
///         0.0.try_into().unwrap()
///     }
/// }
/// ```
pub trait KnownOptimumProblem: SingleObjectiveProblem {
    /// Retrieves the known optimum objective value for the optimization problem.
    fn known_optimum(&self) -> SingleObjective;
}

/// The [Travelling Salesperson Problem (TSP)].
///
/// TSP is a single-objective optimization problem that involves finding
/// the shortest possible route that visits a given set of vertices and
/// returns to the starting vertex.
///
/// [Travelling Salesperson Problem (TSP)]: https://en.wikipedia.org/wiki/Travelling_salesman_problem
///
/// # Examples
///
/// A simple implementation of the Travelling Salesperson Problem given a edge weight matrix:
///
/// ```
/// use mahf::{
///     problems::{TravellingSalespersonProblem, VectorProblem},
///     Problem, SingleObjective,
/// };
///
/// pub struct TSP {
///     pub instance: String,
///     pub edge_weights: Vec<Vec<f64>>,
/// }
///
/// impl Problem for TSP {
///     type Encoding = Vec<usize>;
///     type Objective = SingleObjective;
///
///     fn name(&self) -> &str {
///         &self.instance
///     }
/// }
///
/// impl VectorProblem for TSP {
///     type Element = usize;
///
///     fn dimension(&self) -> usize {
///         self.edge_weights.len()
///     }
/// }
///
/// impl TravellingSalespersonProblem for TSP {
///     fn distance(&self, edge: (usize, usize)) -> f64 {
///         let (source, target) = edge;
///         self.edge_weights[source][target]
///     }
/// }
/// ```
pub trait TravellingSalespersonProblem:
    SingleObjectiveProblem + VectorProblem<Element = usize>
{
    /// Calculates the distance between two locations,
    /// i.e. the edge weight between two vertices identified by their indices.
    fn distance(&self, edge: (usize, usize)) -> f64;
}
