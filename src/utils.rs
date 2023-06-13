pub fn with_index<T, F>(mut f: F) -> impl FnMut(&T) -> bool
where
    F: FnMut(usize, &T) -> bool,
{
    let mut i = 0;
    move |item| (f(i, item), i += 1).0
}

/// A trait for giving a type a useful default value that may fail in a controlled
/// way under some circumstances.
///
/// This is useful for handling types with a default implementation the same way like types that don't.
///
/// This trait is automatically implemented for types that implement [Default].
///
/// # Examples
///
/// ```
/// use mahf::utils::TryDefault;
///
/// pub struct TypeThatNeedsManualInitialization;
///
/// impl TryDefault for TypeThatNeedsManualInitialization {
///     type Error = ();
///
///     fn try_default() -> Result<Self, Self::Error> {
///         Err(())
///     }
/// }
///
/// ```
pub trait TryDefault: Sized {
    type Error;

    /// Returns the "default value" for a type.
    /// Default values are often some kind of initial value, identity value, or anything else that may make sense as a default.
    ///
    /// # Examples
    ///
    /// ```
    /// use mahf::utils::TryDefault;
    /// assert_eq!(Ok(0), i8::try_default());
    /// assert_eq!(Ok(0.0), f64::try_default());
    /// ```
    fn try_default() -> Result<Self, Self::Error>;
}

impl<T: Default> TryDefault for T {
    type Error = ();

    fn try_default() -> Result<Self, Self::Error> {
        Ok(T::default())
    }
}
