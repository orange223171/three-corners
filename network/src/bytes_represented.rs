use std::{error, fmt::Display};

use core_3c::{building::Building, vector::Vector};

pub mod build;
pub mod destroy;
pub mod grab;
pub mod set_triangle;

/// A decoder counting current offset of data array
pub(crate) struct Decoder {
    current_byte: usize,
}

impl Decoder {
    /// Returns new decoder with zero offset
    pub fn new() -> Decoder {
        Decoder { current_byte: 0 }
    }

    /// Returns slice with specified length from original slice, if specifies
    pub(crate) fn slice<'a>(&mut self, bytes: &'a [u8], len: usize) -> Option<&'a [u8]> {
        if self.current_byte + len < bytes.len() {
            None
        } else {
            let slice = &bytes[self.current_byte..len];
            self.current_byte += len;
            Some(slice)
        }
    }
}

/// A structure that can be representate as bytes
pub(crate) trait BytesRepresented {
    fn encode(self) -> Vec<u8>;
    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
}

impl BytesRepresented for i8 {
    fn encode(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match decoder.slice(bytes, size_of::<i8>()) {
            None => Result::Err(Error::UnexpectedEndOfBytesArray),
            Some(slice) => {
                Result::Ok(i8::from_be_bytes(slice.try_into().expect(
                    "Type mismatch: len of slice is not equal size_of::<i8>()",
                )))
            }
        }
    }
}

impl BytesRepresented for i16 {
    fn encode(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match decoder.slice(bytes, size_of::<i16>()) {
            None => Result::Err(Error::UnexpectedEndOfBytesArray),
            Some(slice) => {
                Result::Ok(i16::from_be_bytes(slice.try_into().expect(
                    "Type mismatch: len of slice is not equal size_of::<i16>()",
                )))
            }
        }
    }
}

impl BytesRepresented for i32 {
    fn encode(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match decoder.slice(bytes, size_of::<i32>()) {
            None => Result::Err(Error::UnexpectedEndOfBytesArray),
            Some(slice) => {
                Result::Ok(i32::from_be_bytes(slice.try_into().expect(
                    "Type mismatch: len of slice is not equal size_of::<i32>()",
                )))
            }
        }
    }
}

impl BytesRepresented for i64 {
    fn encode(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match decoder.slice(bytes, size_of::<i64>()) {
            None => Result::Err(Error::UnexpectedEndOfBytesArray),
            Some(slice) => {
                Result::Ok(i64::from_be_bytes(slice.try_into().expect(
                    "Type mismatch: len of slice is not equal size_of::<i64>()",
                )))
            }
        }
    }
}

impl BytesRepresented for isize {
    fn encode(self) -> Vec<u8> {
        (self as i64).encode()
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Result::Ok(i64::decode(decoder, bytes)? as isize)
    }
}

impl BytesRepresented for u8 {
    fn encode(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match decoder.slice(bytes, size_of::<u8>()) {
            None => Result::Err(Error::UnexpectedEndOfBytesArray),
            Some(slice) => {
                Result::Ok(u8::from_be_bytes(slice.try_into().expect(
                    "Type mismatch: len of slice is not equal size_of::<u8>()",
                )))
            }
        }
    }
}

impl BytesRepresented for u16 {
    fn encode(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match decoder.slice(bytes, size_of::<u16>()) {
            None => Result::Err(Error::UnexpectedEndOfBytesArray),
            Some(slice) => {
                Result::Ok(u16::from_be_bytes(slice.try_into().expect(
                    "Type mismatch: len of slice is not equal size_of::<u16>()",
                )))
            }
        }
    }
}

impl BytesRepresented for u32 {
    fn encode(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match decoder.slice(bytes, size_of::<u32>()) {
            None => Result::Err(Error::UnexpectedEndOfBytesArray),
            Some(slice) => {
                Result::Ok(u32::from_be_bytes(slice.try_into().expect(
                    "Type mismatch: len of slice is not equal size_of::<u32>()",
                )))
            }
        }
    }
}

impl BytesRepresented for u64 {
    fn encode(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match decoder.slice(bytes, size_of::<u64>()) {
            None => Result::Err(Error::UnexpectedEndOfBytesArray),
            Some(slice) => {
                Result::Ok(u64::from_be_bytes(slice.try_into().expect(
                    "Type mismatch: len of slice is not equal size_of::<u64>()",
                )))
            }
        }
    }
}

impl BytesRepresented for usize {
    fn encode(self) -> Vec<u8> {
        (self as u64).encode()
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Result::Ok(u64::decode(decoder, bytes)? as usize)
    }
}

impl BytesRepresented for bool {
    fn encode(self) -> Vec<u8> {
        if self { vec![1] } else { vec![0] }
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match decoder.slice(bytes, 1) {
            None => Result::Err(Error::UnexpectedEndOfBytesArray),
            Some(slice) => Result::Ok(if slice[0] > 0 { true } else { false }),
        }
    }
}

impl BytesRepresented for String {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.len().encode());
        v.append(&mut self.as_bytes().to_vec());

        v
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let len = usize::decode(decoder, bytes)?;

        match decoder.slice(bytes, len as usize) {
            None => Result::Err(Error::UnexpectedEndOfBytesArray),
            Some(slice) => Result::Ok(match String::from_utf8(slice.to_vec()) {
                Ok(s) => s,
                Err(e) => {
                    return Result::Err(Error::UncorrectFormat(
                        String::from("utf8 string"),
                        e.into_bytes(),
                    ));
                }
            }),
        }
    }
}

impl<T: BytesRepresented> BytesRepresented for Vec<T> {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.len().encode());
        self.into_iter()
            .for_each(|element| v.append(&mut element.encode()));

        v
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let len = usize::decode(decoder, bytes)?;

        let mut v: Vec<T> = Vec::new();
        for _ in 0..len {
            v.push(T::decode(decoder, bytes)?)
        }

        Result::Ok(v)
    }
}

impl BytesRepresented for Vector {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.x.encode());
        v.append(&mut self.y.encode());

        v
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            x: u32::decode(decoder, bytes)?,
            y: u32::decode(decoder, bytes)?,
        })
    }
}

impl BytesRepresented for Building {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.name.encode());
        v.append(&mut self.player.encode());
        v.append(&mut self.build_in_current_round.encode());
        v.append(&mut self.synergies.encode());

        v
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            name: String::decode(decoder, bytes)?,
            player: String::decode(decoder, bytes)?,
            build_in_current_round: bool::decode(decoder, bytes)?,
            synergies: Vec::decode(decoder, bytes)?,
        })
    }
}

#[derive(Debug)]
/// An error from decoding type that can be represented how bytes
pub enum Error {
    /// Unexpeceted end of bytes array
    UnexpectedEndOfBytesArray,
    /// Uncorrect format, contains data type and data
    UncorrectFormat(String, Vec<u8>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedEndOfBytesArray => write!(f, "Unexpeced end of bytes array"),
            Error::UncorrectFormat(data_type, data) => {
                write!(f, "Uncorrect format of {}: {:?}", data_type, data)
            }
        }
    }
}

impl error::Error for Error {}
