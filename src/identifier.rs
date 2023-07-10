//! Identifiers to distinguish between different components of the same type.

use std::{any::type_name, marker::PhantomData};

use serde::{ser::SerializeTupleStruct, Serialize, Serializer};
use trait_set::trait_set;

trait_set! {
    /// Collection of traits required by every identifier.
    ///
    /// In general, identifiers should be zero-sized.
    pub trait Identifier = Default + Copy + Clone + Serialize + Send + Sync + 'static;
}

macro_rules! identifier {
    ($name:ident, $doc:tt) => {
        #[derive(Default, Copy, Clone, Serialize)]
        #[doc = $doc]
        pub struct $name;
    };
}

/// A [`PhantomData`] wrapper for [`Identifier`]s, which implements [`Serialize`] using the type name of `I`.
///
/// This makes it possible to infer which identifier was used in serialized [`Component`]s.
///
/// [`Component`]: crate::Component
#[derive(Default, Copy, Clone)]
pub struct PhantomId<I: Identifier>(PhantomData<fn() -> I>);

impl<I: Identifier> Serialize for PhantomId<I> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut id = serializer.serialize_tuple_struct("Id", 1)?;
        id.serialize_field(type_name::<I>())?;
        id.end()
    }
}

#[rustfmt::skip]
mod inner {
    use super::*;

    identifier!(Global, "The default identifier.\n\nIf no identifier is specified, it is assumed the `Global` identifier is used.");

    identifier!(A, "An identifier to distinguish between components of the same type.");
    identifier!(B, "An identifier to distinguish between components of the same type.");
    identifier!(C, "An identifier to distinguish between components of the same type.");
    identifier!(D, "An identifier to distinguish between components of the same type.");
    identifier!(E, "An identifier to distinguish between components of the same type.");

    identifier!(I0, "An identifier to distinguish between components of the same type.");
    identifier!(I1, "An identifier to distinguish between components of the same type.");
    identifier!(I2, "An identifier to distinguish between components of the same type.");
    identifier!(I3, "An identifier to distinguish between components of the same type.");
    identifier!(I4, "An identifier to distinguish between components of the same type.");

    identifier!(Seq, "An identifier to signalize a sequential evaluator.");
    identifier!(Par, "An identifier to signalize a parallel evaluator.");
}

#[doc(inline)]
pub use inner::*;
