//! Identifiers to distinguish between different components of the same type.

use std::{any::type_name, marker::PhantomData};

use serde::{ser::SerializeTupleStruct, Serialize, Serializer};
use trait_set::trait_set;

trait_set! {
    pub trait Identifier = Default + Copy + Clone + Serialize + Send + Sync + 'static;
}

macro_rules! identifier {
    ($name:ident) => {
        /// A default identifier to distinguish between components of the same type.
        #[derive(Default, Copy, Clone, Serialize)]
        pub struct $name;
    };
}

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

identifier!(Global);

identifier!(A);
identifier!(B);
identifier!(C);
identifier!(D);
identifier!(E);

identifier!(I0);
identifier!(I1);
identifier!(I2);
identifier!(I3);
identifier!(I4);
