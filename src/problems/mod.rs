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
//! There exist several pre-implemented problems in the MAHF ecosystem, which provide a good
//! starting point.
//!
//! TODO: Reference mahf-problems repo(s) here.
//!
//! ## Implement custom problems
//!
//! To define your own optimization problem, the minimum requirement is to implement
//! [`Problem`] and some sort of [`Evaluator`] for it, i.e. implement [`Evaluate`] for
//! some struct.
//!
//! Note that for most problems with a *static* objective function,
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

use crate::state::common::EvaluatorInstance;
pub use encoding::AnyEncoding;
pub use evaluate::{Evaluate, Evaluator, ObjectiveFunction};
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
/// TODO: Reference component (module) documentation here.
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
///     type Encoding = Vec<f64>; // real-valued vector
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

    /// Returns the default evaluator for the problem.
    ///
    /// To be removed in the future in favor of a [`TryDefault`] bound on [`Evaluate`].
    ///
    /// [`TryDefault`]: crate::utils::TryDefault
    #[deprecated]
    fn default_evaluator<'a>(&self) -> EvaluatorInstance<'a, Self> {
        unimplemented!()
    }
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
/// use mahf::{Problem, SingleObjective, problems::VectorProblem};
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
/// use mahf::{Problem, SingleObjective, problems::{VectorProblem, LimitedVectorProblem}};
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

/// A single-objective optimization problem with an indicator for reaching the optimum.
///
/// If the objective value of the optimum is known directly, implement [`KnownOptimumProblem`]
/// instead, which provides a blanket implementation for this trait.
///
/// # Examples
///
/// A simple implementation of the real-valued sphere function `f(x) = x^2`, where
/// the optimum is known to be 0.
///
/// Note that a implementation of [`KnownOptimumProblem`] is more sensible in this case because
/// the value is known directly.
///
/// ```
/// use std::ops::Range;
/// use mahf::{Problem, SingleObjective, problems::OptimumReachedProblem};
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
/// impl OptimumReachedProblem for Sphere {
///     fn optimum_reached(&self, objective: SingleObjective) -> bool {
///         // Approximately 0 is accepted as optimum.
///         objective.value() < 1e-8
///     }
/// }
///
/// // A value smaller as 1e-8 counts as optimum.
/// let sphere = Sphere { dim: 1 };
/// assert!(sphere.optimum_reached(1e-9.try_into().unwrap()));
/// ```
pub trait OptimumReachedProblem: SingleObjectiveProblem {
    /// Checks whether the objective value has reached the optimum.
    fn optimum_reached(&self, objective: SingleObjective) -> bool;
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
/// use mahf::{Problem, SingleObjective, problems::{KnownOptimumProblem, OptimumReachedProblem}};
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
///     const DELTA: f64 = 1e-10;
///
///     fn known_optimum(&self) -> SingleObjective {
///         0.0.try_into().unwrap()
///     }
/// }
///
/// // A value greater than 1e-10 does not counts as optimum.
/// let sphere = Sphere { dim: 1 };
/// assert!(!sphere.optimum_reached(1e-9.try_into().unwrap()));
/// ```
pub trait KnownOptimumProblem: SingleObjectiveProblem {
    /// A constant representing the tolerance level for comparing objective values.
    ///
    /// This value is used as tolerance to automatically implement [`OptimumReachedProblem`].
    const DELTA: f64 = 1e-8;

    /// Retrieves the known optimum objective value for the optimization problem.
    fn known_optimum(&self) -> SingleObjective;
}

impl<P: KnownOptimumProblem> OptimumReachedProblem for P {
    fn optimum_reached(&self, objective: SingleObjective) -> bool {
        let provided = objective.value();
        let known = self.known_optimum().value();
        debug_assert!(
            provided >= known,
            "the provided objective value is smaller than the known optimum"
        );

        (provided - known).abs() <= P::DELTA
    }
}

/// The Travelling Salesperson Problem (TSP) optimization problem.
///
/// TSP is a single-objective optimization problem that involves finding
/// the shortest possible route that visits a given set of vertices and
/// returns to the starting vertex.
///
/// # Examples
///
/// A simple implementation of the Travelling Salesperson Problem given a edge weight matrix:
///
/// ```
/// use mahf::{Problem, problems::{VectorProblem, TravellingSalespersonProblem}, SingleObjective};
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
