//! Error type for [`StateRegistry`] errors.
//!
//! [`StateRegistry`]: crate::StateRegistry

use std::{
    any::type_name,
    cell::{BorrowError, BorrowMutError},
};

use thiserror::Error;

/// An error returned by fallible [`StateRegistry`] methods like [`StateRegistry::try_borrow`].
///
/// [`StateRegistry`]: crate::StateRegistry
/// [`StateRegistry::try_borrow`]: crate::StateRegistry::try_borrow
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum StateError {
    #[error("`{0}` does not exist in the state")]
    NotFound(&'static str),
    #[error("`{0}` can't be immutably borrowed: {1}")]
    BorrowConflictImm(BorrowError, &'static str),
    #[error("`{0}` can't be mutably borrowed: {1}")]
    BorrowConflictMut(BorrowMutError, &'static str),
    #[error("`{0}` contains the same type multiple times")]
    MultipleBorrowConflict(&'static str),
    #[error("`{0}` is missing, but it is a requirement of `{1}`")]
    RequiredMissing(&'static str, &'static str),
}

impl StateError {
    pub(crate) fn not_found<T>() -> Self {
        Self::NotFound(type_name::<T>())
    }

    pub(crate) fn borrow_conflict<T>(error: BorrowError) -> Self {
        Self::BorrowConflictImm(error, type_name::<T>())
    }

    pub(crate) fn borrow_conflict_mut<T>(error: BorrowMutError) -> Self {
        Self::BorrowConflictMut(error, type_name::<T>())
    }

    pub(crate) fn multiple_borrow_conflict<T>() -> Self {
        Self::MultipleBorrowConflict(type_name::<T>())
    }

    pub(crate) fn required_missing<S, T>() -> Self {
        let t = type_name::<T>();
        let source = type_name::<S>();
        Self::RequiredMissing(t, source)
    }

    pub(crate) fn panic<Ret>(self) -> Ret {
        panic!("{self}")
    }
}

/// A specialized result type for [`StateRegistry`] operations.
///
/// [`StateRegistry`]: crate::StateRegistry
pub type StateResult<T> = Result<T, StateError>;
