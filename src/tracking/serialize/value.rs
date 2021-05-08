use super::error::*;
use serde::{ser, Serialize};
use std::fmt;

pub struct Serializer {
    itoa_buffer: itoa::Buffer,
    ryu_buffer: ryu::Buffer,
}

impl Serializer {
    pub fn new() -> Self {
        Serializer {
            itoa_buffer: itoa::Buffer::new(),
            ryu_buffer: ryu::Buffer::new(),
        }
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<String> {
        let val = if v { "true" } else { "false" };
        Ok(String::from(val))
    }

    fn serialize_i8(self, v: i8) -> Result<String> {
        Ok(self.itoa_buffer.format(v).to_string())
    }

    fn serialize_i16(self, v: i16) -> Result<String> {
        Ok(self.itoa_buffer.format(v).to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<String> {
        Ok(self.itoa_buffer.format(v).to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<String> {
        Ok(self.itoa_buffer.format(v).to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<String> {
        Ok(self.itoa_buffer.format(v).to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<String> {
        Ok(self.itoa_buffer.format(v).to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<String> {
        Ok(self.itoa_buffer.format(v).to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<String> {
        Ok(self.itoa_buffer.format(v).to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<String> {
        Ok(self.ryu_buffer.format(v).to_string())
    }

    fn serialize_f64(self, v: f64) -> Result<String> {
        Ok(self.ryu_buffer.format(v).to_string())
    }

    fn serialize_char(self, v: char) -> Result<String> {
        Ok(String::from(v))
    }

    fn serialize_str(self, v: &str) -> Result<String> {
        Ok(String::from(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<String> {
        Ok(format!("{:?}", v))
    }

    fn serialize_none(self) -> Result<String> {
        Ok(String::new())
    }

    fn serialize_some<T>(self, value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<String> {
        Err(Error::from("unit values can not be serialized"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<String> {
        Err(Error::from("unit structs can not be serialized"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String> {
        Err(Error::from("unit variants can not be serialized"))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::from("sequences can not be serialized"))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::from("tuples can not be serialized"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::from("tuple structs can not be serialized"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::from("tuple variants can not be serialized"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::from("maps can not be serialized"))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::from("sub structs can not be serialized"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::from("sub structs can not be serialized"))
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<String> {
        unreachable!()
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<String> {
        unreachable!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<String> {
        unreachable!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<String> {
        unreachable!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<String> {
        unreachable!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<String> {
        unreachable!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<String> {
        unreachable!()
    }
}
