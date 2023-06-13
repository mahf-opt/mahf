use std::collections::btree_set::BTreeSet;

use trait_set::trait_set;

trait_set! {
    /// Collection of traits required by every solution encoding.
    pub trait AnyEncoding = Clone + PartialEq + Send;
}

pub fn valid_permutation<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Ord,
{
    let mut uniq = BTreeSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
