//! Marker trait for custom state.

/// Marker trait to represent a custom state that is `Send + 'a`.
///
/// This trait is **not** automatically implemented for arbitrary types.
///
/// Custom state is stored in the [State][crate::State] and can be accessed according to
/// the typical borrowing model (one writer xor multiple readers).
///
/// # Requires
///
/// Requires to implement [better_any::Tid], which can be done using the `derive` macro.
///
/// # Examples
///
/// Custom state wrapping a owned type:
/// ```
/// use better_any::{Tid, TidAble};
/// use mahf::CustomState;
///
/// #[derive(Tid)]
/// pub struct Owned(usize);
/// impl CustomState<'_> for Owned {}
/// ```
///
/// Custom state wrapping a reference with a lifetime:
/// ```
/// use better_any::{Tid, TidAble};
/// use mahf::CustomState;
///
/// #[derive(Tid)]
/// pub struct Reference<'a>(&'a mut usize);
/// impl<'a> CustomState<'a> for Reference<'a> {}
/// ```
pub trait CustomState<'a>: better_any::Tid<'a> + Send {}
