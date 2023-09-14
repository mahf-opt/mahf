//! A collection of utilities.

use std::marker::PhantomData;

use derivative::Derivative;
use crate::problems::VectorProblem;
use crate::{SingleObjective, SingleObjectiveProblem};

/// Allows enumeration for functions which normally don't support enumeration, e.g. [`Vec::retain`].
///
/// # Examples
///
/// ```
/// use mahf::utils::with_index;
///
/// let mut v = vec![1, 2, 3, 4, 5];
/// // Remove every second element.
/// v.retain(with_index(|index, _value| index % 2 == 0));
/// assert_eq!(v, vec![1, 3, 5]);
/// ```
pub fn with_index<T, F>(mut f: F) -> impl FnMut(&T) -> bool
where
    F: FnMut(usize, &T) -> bool,
{
    let mut i = 0;
    move |item| (f(i, item), i += 1).0
}

/// Returns if all elements in `arr` are equal.
pub fn all_eq<T: PartialEq>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] == w[1])
}

/// Wrapper around [`PhantomData`] that serializes the type name of `T`.
///
/// It additionally implements `Send` + `Sync` even if `T` doesn't.
#[derive(Derivative)]
#[derivative(Default(bound = ""), Copy(bound = ""), Clone(bound = ""))]
pub struct SerializablePhantom<T>(PhantomData<fn() -> T>);

impl<T> serde::Serialize for SerializablePhantom<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit_struct(std::any::type_name::<T>())
    }
}

/// Calculates squared Euclidean distance between two vectors.
pub fn squared_euclidean<P> (a: Vec<P>, b: Vec<P>) -> f64
where
    P: VectorProblem<Element = f64, Objective = SingleObjective>,
{
    a.iter().zip(b).map(|p, q| (p - q).powf(2.0)).sum::<f64>()
}