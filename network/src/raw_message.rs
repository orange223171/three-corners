use std::{error::Error, fmt::Display};

use serde::{
    Serializer,
    ser::{
        self, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
};

/// Raw representation of message for sending and recieving
pub struct RawMessage {
    message: Vec<u8>,
}

impl RawMessage {
    /// Returns empty raw message
    pub fn new() -> RawMessage {
        RawMessage {
            message: Vec::new(),
        }
    }

    /// Returns slice of u8
    pub fn as_bytes(&self) -> &[u8] {
        self.message.as_slice()
    }

    fn push(&mut self, value: u8) {
        self.message.push(value)
    }
}

impl Serializer for &mut RawMessage {
    type Ok = Self;
    type Error = SerializeError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.push(v as u8);
        Result::Ok(self)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let mut new = self;
        new.push(v as u8);
        Result::Ok(new)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let mut new = self;
        v.to_be_bytes().map(|v| new.push(v));
        Result::Ok(new)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let mut new = self;
        v.to_be_bytes().map(|v| new.push(v));
        Result::Ok(new)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let mut new = self;
        v.to_be_bytes().map(|v| new.push(v));
        Result::Ok(new)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let mut new = self;
        new.push(v);
        Result::Ok(new)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let mut new = self;
        v.to_be_bytes().map(|v| new.push(v));
        Result::Ok(new)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let mut new = self;
        v.to_be_bytes().map(|v| new.push(v));
        Result::Ok(new)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let mut new = self;
        v.to_be_bytes().map(|v| new.push(v));
        Result::Ok(new)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let mut new = self;
        v.to_be_bytes().map(|v| new.push(v));
        Result::Ok(new)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let mut new = self;
        v.to_be_bytes().map(|v| new.push(v));
        Result::Ok(new)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut new = self;

        v.to_string().as_bytes().into_iter().map(|v| new.push(*v));

        Result::Ok(new)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let mut new = self;

        v.as_bytes().into_iter().map(|v| new.push(*v));

        Result::Ok(new)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut new = self;

        v.into_iter().map(|v| new.push(*v));

        Result::Ok(new)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        let mut new = self;

        new.push(0);

        Result::Ok(new)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut new = self;

        new.push(1);
        new = value.serialize(new)?;

        Result::Ok(new)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Result::Ok(self)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        name.as_bytes().into_iter().map(|v| self.push(*v));
        Result::Ok(self)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let mut new = self;

        name.as_bytes().into_iter().map(|v| new.push(*v));
        variant_index.to_be_bytes().map(|v| new.push(v));
        variant.as_bytes().into_iter().map(|v| new.push(*v));

        Result::Ok(new)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut new = self;

        name.as_bytes().into_iter().map(|v| new.push(*v));
        new = value.serialize(new)?;

        Result::Ok(new)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let mut new = self;

        name.as_bytes().into_iter().map(|v| new.push(*v));
        variant_index.to_be_bytes().map(|v| new.push(v));
        variant.as_bytes().into_iter().map(|v| new.push(*v));

        new = value.serialize(new)?;

        Result::Ok(new)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Result::Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Result::Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Result::Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Result::Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Result::Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Result::Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Result::Ok(self)
    }
}

impl SerializeSeq for &mut RawMessage {
    type Ok = Self;
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)?;
        Result::Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::Ok(self)
    }
}

impl SerializeTuple for &mut RawMessage {
    type Ok = Self;
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Result::Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::Ok(self)
    }
}

impl SerializeTupleStruct for &mut RawMessage {
    type Ok = Self;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Result::Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::Ok(self)
    }
}

impl SerializeTupleVariant for &mut RawMessage {
    type Ok = Self;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Result::Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::Ok(self)
    }
}

impl SerializeMap for &mut RawMessage {
    type Ok = Self;
    type Error = SerializeError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Result::Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Result::Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::Ok(self)
    }
}

impl SerializeStruct for &mut RawMessage {
    type Ok = Self;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Result::Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::Ok(self)
    }
}

impl SerializeStructVariant for &mut RawMessage {
    type Ok = Self;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Result::Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::Ok(self)
    }
}

#[derive(Debug)]
pub struct SerializeError {
    message: String,
}
impl Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for SerializeError {}
impl ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        SerializeError {
            message: format!("{}", msg),
        }
    }
}
