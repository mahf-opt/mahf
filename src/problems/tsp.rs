//! This module allows easy access to instances of the traveling salesman problem taken from tsplib.
#![allow(clippy::upper_case_acronyms)]

use crate::{fitness::Fitness, random::Random};
use itertools::Itertools;
use rand::Rng;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

/// Symmetric TSP
pub mod symmetric;
pub use symmetric::Instances;
pub use symmetric::SymmetricTsp;

/// Asymmetric TSP
pub mod asymmetric {}

type Coordinates = Vec<f64>;
type DistanceMeasure = fn(&[f64], &[f64]) -> Fitness;
type Dimension = usize;

/// Popular distance functions used in TSP.
pub mod distances {
    use std::convert::{TryFrom, TryInto};

    use crate::fitness::Fitness;

    pub fn euclidean_distance(a: &[f64], b: &[f64]) -> Fitness {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
            .try_into()
            .unwrap()
    }

    pub fn manhattan_distance(a: &[f64], b: &[f64]) -> Fitness {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .sum::<f64>()
            .try_into()
            .unwrap()
    }

    pub fn maximum_distance(a: &[f64], b: &[f64]) -> Fitness {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .map(|d| Fitness::try_from(d).unwrap())
            .max()
            .unwrap()
    }
}

pub type Edge = (usize, usize);
pub type Node = usize;
/// Represents a route in a TSP instance.
#[derive(Debug, Clone, Eq)]
pub struct Route(Vec<Node>);
// Allow direct access to nodes
impl Deref for Route {
    type Target = Vec<Node>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Route {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// Simple conversions
impl From<Vec<Node>> for Route {
    fn from(v: Vec<Node>) -> Self {
        Self(v)
    }
}
impl From<Route> for Vec<Node> {
    fn from(val: Route) -> Self {
        val.0
    }
}
// I'm still not sure if this shouldn't be a `Equivalent` implementation instead.
impl PartialEq for Route {
    /// Two routes are considered equal if they contain the same edges.
    fn eq(&self, other: &Self) -> bool {
        self.iter_normalized()
            .zip(other.iter_normalized())
            .all(|(a, b)| a == b)
    }
}
impl Hash for Route {
    // Hashing the edges so that k.hash() == l.hash() => k == l.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // For some reason cloning is faster than iterating...
        let mut route = self.clone();
        route.normalize();
        route.0.hash(state);
    }
}

impl Route {
    /// Executes the 2opt move, i.e. reverses the route between the indices `k` and `l`.
    pub fn _2opt(&self, k: usize, l: usize) -> Route {
        let mut new = self.clone();
        new[k..l].reverse();
        new
    }
    /// Executes a 2opt move with random indices.
    pub fn random_2opt(&self, rng: &mut Random) -> Route {
        let n = self.len();
        let k = rng.gen::<usize>() % n;
        let l = k + rng.gen::<usize>() % (n - k);
        self._2opt(k, l)
    }

    /// Executes the 3opt move, i.e. reverses the route between the indices 0-`k`,
    /// `k`-`l` and `l`-`dim` based on the reverse pattern supplied.
    ///
    /// E.g. the `rev`-pattern `[true, false, true]` reverses 0-`k` and `l`-`dim`, but not
    /// `k`-`l`.
    pub fn _3opt(&self, i: usize, j: usize, rev: &[bool]) -> Route {
        assert_eq!(rev.len(), 3);
        let n = self.len();
        let mut new = self.clone();
        let ranges = vec![0..i, i..j, j..n - 1];
        for (range, &reverse) in ranges.into_iter().zip(rev.iter()) {
            if reverse && range.len() > 1 {
                new[range].reverse();
            }
        }
        new
    }
    /// Executes a random 3opt move and returns the result.
    pub fn random_3opt(&self, rng: &mut Random) -> Route {
        let n = self.len();
        let i = rng.gen::<usize>() % n;
        let j = i + rng.gen::<usize>() % (n - i);
        let rev = (1..=3).map(|_| rng.gen::<bool>()).collect::<Vec<bool>>();
        self._3opt(i, j, &rev)
    }

    /// Returns all edges of this route.
    pub fn edges(&self) -> Vec<Edge> {
        let n = self.len();
        (0usize..n)
            .zip(1usize..n + 1)
            .map(|(prev, next)| (self[prev], self[next % n]))
            .collect()
    }
    /// Swap the nodes `l` and `k`. Note that `l` and `k` are nodes, not indices.
    pub fn swap_nodes(&mut self, k: Node, l: Node) {
        let i_k = self.index_of_node(k);
        let i_l = self.index_of_node(l);
        self.swap(i_k, i_l);
    }
    /// Returns the index of the node `v` in the route.
    pub fn index_of_node(&self, v: Node) -> usize {
        assert!(v < self.len());
        self.iter().position(|&n| n == v).unwrap()
    }
    /// Shifts the route until it begins with Node 0.
    pub fn normalize(&mut self) {
        let (index_of_first, _) = self.iter().find_position(|&&v| v == 0).unwrap();
        self.rotate_left(index_of_first);
    }
    pub fn iter_normalized(&self) -> impl Iterator<Item = usize> + '_ {
        self.iter()
            .copied()
            .cycle()
            .skip_while(|&c| c != 0)
            .take(self.len())
    }
    /// Construct an empty route, e.g. to build a route from scratch.
    /// Note that this route is not valid by design, additional logic is necessary.
    pub fn empty() -> Self {
        Route::from(Vec::new())
    }
}
