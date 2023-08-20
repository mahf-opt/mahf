//! A framework for the modular construction and evaluation of metaheuristics.
//!
//! MAHF allows easy construction and experimental analysis of metaheuristics by
//! decomposing them into their fundamental components.
//!
//! The framework supports not only evolutionary algorithms, but also any other
//! metaheuristic frameworks, including non-population-based,
//! constructive, and especially hybrid approaches.
//!
//! # Overview
//!
//! MAHF aims to make the construction and modification of metaheuristics as simple and reliable as possible.
//! It provides a comprehensive set of utilities for logging, evaluation, and comparison of these heuristics.
//!
//! Key features include:
//! - Simple and modular metaheuristic construction
//! - Effortless state management and tracking
//! - Ready-to-use collection of common operators
//! - Templates for popular metaheuristics
//! - Flexible logging of runtime information
//!
//! Although MAHF has been developed primarily as a research tool, it can be used to solve real-world problems.
//!
//! ## Example
//!
//! Defining a simple genetic algorithm for real-valued black-box optimization:
//!
//! ```
//! use mahf::prelude::*;
//!
//! # fn example<P: problems::SingleObjectiveProblem + problems::LimitedVectorProblem<Element = f64>>(
//! #    population_size: u32,
//! #    n: u32,
//! #    num_selected: u32,
//! #    size: u32,
//! #    pc: f64,
//! #    std_dev: f64,
//! #    rm: f64,
//! #    max_population_size: u32,
//! # ) -> Configuration<P> {
//! let ga = Configuration::builder()
//!     .do_(initialization::RandomSpread::new(population_size))
//!     .evaluate()
//!     .update_best_individual()
//!     .while_(conditions::LessThanN::iterations(n), |builder| {
//!        builder
//!            .do_(selection::Tournament::new(num_selected, size))
//!            .do_(recombination::ArithmeticCrossover::new_insert_both(pc))
//!            .do_(mutation::NormalMutation::new(std_dev, rm))
//!            .do_(boundary::Saturation::new())
//!            .evaluate()
//!            .update_best_individual()
//!            .do_(replacement::MuPlusLambda::new(max_population_size))
//!     })
//!     .build();
//! # ga
//! # }
//! ```
//!
//! # [`Component`]-based design
//!
//! MAHF is based on the concept of *unified metaheuristics*, which means that we interpret
//! metaheuristics to be composed of components and conditions that can be mixed and matched.
//! They are represented by the [`Component`] and [`Condition`] traits.
//!
//! Putting components together into a metaheuristic is as easy as writing pseudo-code thanks to
//! the [`Configuration::builder`], as you can see in the [example](crate#example).
//!
//!
//! # Runtime [`state`]
//!
//! Components can only realize complex functionality if they can communicate freely,
//! and the [`State`] offers a way to do it.
//! The [`State`] is a shared blackboard where components can insert, read, write, and remove
//! so-called [`CustomState`].
//!
//! # Optimization [`problems`]
//!
//! Optimization [`problems`] are represented by the [`Problem`] and [`Evaluate`] traits:
//! - [`Problem`] and subtraits offer a way to provide any problem-specific information to
//!  components and allow them to be as generic as possible by only specifying the
//!  minimal trait bounds they need to function
//! - [`Evaluate`] provides means of evaluating the objective function, e.g. in parallel
//!
//! Optimizing some problem is then as easy as calling [`Configuration::optimize`].
//!
//! [`Evaluate`]: problems::Evaluate
//!
//! # Meta[`heuristic`] templates
//!
//! [`heuristic`]: heuristics
//!
//! MAHF offers pre-built modular templates for a dozen of common meta[`heuristics`] as a starting point
//! for more complex hybrids.
//! Take a look at their implementation if you want to get a feel
//! for how different metaheuristics are represented in MAHF.
//!
//! # Citing MAHF
//!
//! If you use MAHF in a scientific publication, we would appreciate citations to the following paper or the technical report:
//!
//! #### Conference Paper
//!
//! Jonathan Wurth, Helena Stegherr, Michael Heider, Leopold Luley, and Jörg Hähner. 2023.
//! Fast, Flexible, and Fearless: A Rust Framework for the Modular Construction of Metaheuristics.
//! In Proceedings of the Companion Conference on Genetic and Evolutionary Computation (GECCO ’23 Companion),
//! Association for Computing Machinery, New York, NY, USA, 1900–1909.
//! DOI:<https://doi.org/10.1145/3583133.3596335>
//!
//! ```bibtex
//! @inproceedings{wurth2023,
//!   title = {Fast, {{Flexible}}, and {{Fearless}}: {{A Rust Framework}} for the {{Modular Construction}} of {{Metaheuristics}}},
//!   booktitle = {Proceedings of the {{Companion Conference}} on {{Genetic}} and {{Evolutionary Computation}}},
//!   author = {Wurth, Jonathan and Stegherr, Helena and Heider, Michael and Luley, Leopold and Hähner, Jörg},
//!   date = {2023-07-24},
//!   series = {{{GECCO}} '23 {{Companion}}},
//!   pages = {1900--1909},
//!   publisher = {{Association for Computing Machinery}},
//!   location = {{New York, NY, USA}},
//!   doi = {10.1145/3583133.3596335},
//!   url = {https://dl.acm.org/doi/10.1145/3583133.3596335},
//!   isbn = {9798400701207},
//! }
//! ```
//!
//! #### Technical Report
//!
//! Helena Stegherr, Leopold Luley, Jonathan Wurth, Michael Heider, and Jörg Hähner. 2023. A framework for modular
//! construction and evaluation of metaheuristics. Fakultät für Angewandte
//! Informatik. <https://opus.bibliothek.uni-augsburg.de/opus4/103452>
//!
//! ```bibtex
//! @report{stegherr2023,
//!   title = {A Framework for Modular Construction and Evaluation of Metaheuristics},
//!   author = {Stegherr, Helena and Luley, Leopold and Wurth, Jonathan and Heider, Michael and Hähner, Jörg},
//!   date = {2023},
//!   pages = {25},
//!   institution = {{Fakultät für Angewandte Informatik}},
//!   url = {https://opus.bibliothek.uni-augsburg.de/opus4/103452},
//! }
//! ```

// TODO: #![warn(missing_docs)]

// Enable macros from `mahf-derive` inside `mahf`.
extern crate self as mahf;

#[doc(hidden)]
pub use derive_more;
#[doc(hidden)]
pub use float_eq;
#[doc(hidden)]
pub use rand;
#[doc(hidden)]
pub use rand_distr;
#[doc(hidden)]
pub use serde;

pub mod component;
pub mod components;
pub mod conditions;
pub mod configuration;
pub mod experiments;
pub mod heuristics;
pub mod identifier;
pub mod lens;
pub mod logging;
pub mod params;
pub mod population;
pub mod prelude;
pub mod problems;
pub mod state;
pub(crate) mod testing;
pub mod utils;

#[doc(hidden)]
pub use component::ExecResult;
#[doc(hidden)]
pub use components::Component;
#[doc(hidden)]
pub use conditions::Condition;
#[doc(hidden)]
pub use configuration::Configuration;
#[doc(hidden)]
pub use problems::{
    individual::Individual,
    objective::{MultiObjective, Objective, SingleObjective},
    MultiObjectiveProblem, Problem, SingleObjectiveProblem,
};
#[doc(hidden)]
pub use state::{CustomState, Random, State, StateError, StateRegistry};
