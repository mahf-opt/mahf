//! Serializable and deserializable trait objects.
//!
//! Based on `serde_trait_object`:
//! **[Crates.io](https://crates.io/crates/serde_traitobject) │ [Repo](https://github.com/alecmocatta/serde_traitobject)**
//!

use std::marker;

#[derive(serde::Serialize)]
pub struct DynSerializable<'a, T: Serialize + ?Sized + 'static>(
    #[serde(with = "crate::dynser")] pub &'a Box<T>,
);

/// Any trait with this as a supertrait can be serialized as a trait object.
///
/// It is automatically implemented for all `T: serde::Serialize`, i.e. you should not implement it manually.
///
/// To use, simply add it as a supertrait to your trait:
/// ```
/// use serde::Serialize;
///
/// trait MyTrait: mahf::dynser::Serialize {
///     fn my_method(&self);
/// }
/// ```
///
/// Now your trait object is serializable!
/// ```
/// # use serde::Serialize;
/// #
/// # trait MyTrait: mahf::dynser::Serialize {
/// #     fn my_method(&self);
/// # }
/// #[derive(Serialize)]
/// struct Message(#[serde(with = "mahf::dynser")] Box<dyn MyTrait>);
///
/// // Woohoo, `Message` is now serializable!
/// ```
pub trait Serialize: serialize::Sealed {}
impl<T: serde::ser::Serialize + ?Sized> Serialize for T {}

mod serialize {
    pub trait Sealed: erased_serde::Serialize {
        fn serialize_sized<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
            Self: Sized;
    }

    impl<T: serde::ser::Serialize + ?Sized> Sealed for T {
        #[inline]
        fn serialize_sized<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
            Self: Sized,
        {
            let _ = serializer;
            unreachable!()
        }
    }
}

struct Serializer<T: Serialize + ?Sized + 'static>(marker::PhantomData<fn(T)>);
trait SerializerTrait<T: Serialize + ?Sized> {
    fn serialize<S>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}
impl<T: Serialize + ?Sized + 'static> SerializerTrait<T> for Serializer<T> {
    #[inline]
    fn serialize<S>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::ser::Serialize::serialize(&SerializeErased(t), serializer)
    }
}
struct SerializeErased<'a, T: Serialize + ?Sized + 'a>(&'a T);
impl<'a, T: Serialize + ?Sized> serde::ser::Serialize for SerializeErased<'a, T> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self.0, serializer)
    }
}

/// Serialize a value by reference.
///
/// This is intended to enable:
/// ```
/// # use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct MyStruct {
///     #[serde(with = "mahf::dynser")]
///     field: Box<dyn mahf::dynser::Serialize>,
/// }
/// ```
pub fn serialize<T: Serialize + ?Sized + 'static, B: AsRef<T> + ?Sized, S>(
    t: &B,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    Serializer::<T>::serialize(t.as_ref(), serializer)
}
