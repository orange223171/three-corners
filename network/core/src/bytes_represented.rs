use std::fmt::Display;

use core_3c::{building::Building, vector::Vector};

pub mod build_message;
pub mod destroy_message;
pub mod error_message;
pub mod grab_message;
pub mod log_in_message;
pub mod set_triangle_message;
pub mod version_responce_message;

/// A decoder counting current offset of data array
pub(crate) struct Decoder {
    /// Current decoding byte
    current_byte: usize,
}

impl Decoder {
    /// Returns new decoder with zero offset
    pub fn new() -> Decoder {
        Decoder { current_byte: 0 }
    }

    /// Returns slice with specified length from original slice if original slice contains this data else returns [None]
    pub(crate) fn slice<'a>(&mut self, bytes: &'a [u8], len: usize) -> Option<&'a [u8]> {
        if self.current_byte + len > bytes.len() {
            None
        } else {
            let slice = &bytes[self.current_byte..self.current_byte + len];
            self.current_byte += len;
            Some(slice)
        }
    }
}

/// A structure that can be representate as bytes
pub(crate) trait BytesRepresented {
    /// Encodes struct into bytes array
    fn encode(self) -> Vec<u8>;
    /// Decodes struct from bytes array
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

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoder_slice() {
        let bytes: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];
        let mut decoder = Decoder { current_byte: 2 };

        match decoder.slice(&bytes, 3) {
            Some(slice) => assert_eq!(slice, [2, 3, 4]),
            None => panic!("Decoder didn't return slice"),
        }
    }

    #[test]
    fn decoder_begin_slice() {
        let bytes: [u8; 5] = [0, 1, 2, 3, 4];
        let mut decoder = Decoder::new();

        match decoder.slice(&bytes, 2) {
            Some(slice) => assert_eq!(slice, [0, 1]),
            None => panic!("Decoder didn't return slice"),
        }
    }

    #[test]
    fn decoder_end_slice() {
        let bytes: [u8; 5] = [0, 1, 2, 3, 4];
        let mut decoder = Decoder { current_byte: 3 };

        match decoder.slice(&bytes, 2) {
            Some(slice) => assert_eq!(slice, [3, 4]),
            None => panic!("Decoder didn't return slice"),
        }
    }

    #[test]
    fn decoder_full_slice() {
        let bytes: [u8; 5] = [0, 1, 2, 3, 4];
        let mut decoder = Decoder::new();

        match decoder.slice(&bytes, 5) {
            Some(slice) => assert_eq!(slice, [0, 1, 2, 3, 4]),
            None => panic!("Decoder didn't return slice"),
        }
    }

    #[test]
    fn i8_encode() {
        let value: i8 = 0x12;
        let bytes: Vec<u8> = vec![0x12];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn i8_decode() {
        let value: i8 = 0x12;
        let bytes: [u8; 1] = [0x12];

        let mut decoder = Decoder::new();

        assert_eq!(
            i8::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn i16_encode() {
        let value: i16 = 0x1234;
        let bytes: Vec<u8> = vec![0x12, 0x34];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn i16_decode() {
        let value: i16 = 0x1234;
        let bytes: [u8; 2] = [0x12, 0x34];

        let mut decoder = Decoder::new();

        assert_eq!(
            i16::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn i32_encode() {
        let value: i32 = 0x12345678;
        let bytes: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn i32_decode() {
        let value: i32 = 0x12345678;
        let bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

        let mut decoder = Decoder::new();

        assert_eq!(
            i32::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn i64_encode() {
        let value: i64 = 0x1234567890ABCDEF;
        let bytes: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn i64_decode() {
        let value: i64 = 0x1234567890ABCDEF;
        let bytes: [u8; 8] = [0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef];

        let mut decoder = Decoder::new();

        assert_eq!(
            i64::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn u8_encode() {
        let value: u8 = 0x12;
        let bytes: Vec<u8> = vec![0x12];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn u8_decode() {
        let value: u8 = 0x12;
        let bytes: [u8; 1] = [0x12];

        let mut decoder = Decoder::new();

        assert_eq!(
            u8::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn u16_encode() {
        let value: u16 = 0x1234;
        let bytes: Vec<u8> = vec![0x12, 0x34];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn u16_decode() {
        let value: u16 = 0x1234;
        let bytes: [u8; 2] = [0x12, 0x34];

        let mut decoder = Decoder::new();

        assert_eq!(
            u16::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn u32_encode() {
        let value: u32 = 0x12345678;
        let bytes: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn u32_decode() {
        let value: u32 = 0x12345678;
        let bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

        let mut decoder = Decoder::new();

        assert_eq!(
            u32::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn u64_encode() {
        let value: u64 = 0x1234567890ABCDEF;
        let bytes: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn u64_decode() {
        let value: u64 = 0x1234567890ABCDEF;
        let bytes: [u8; 8] = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];

        let mut decoder = Decoder::new();

        assert_eq!(
            u64::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn usize_encode() {
        let value: usize = 0x1234567890ABCDEF;
        let bytes: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn usize_decode() {
        let value: usize = 0x1234567890ABCDEF;
        let bytes: [u8; 8] = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];

        let mut decoder = Decoder::new();

        assert_eq!(
            usize::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn true_encode() {
        let value: bool = true;
        let bytes: [u8; 1] = [1];

        assert_eq!(value.encode(), bytes);
    }

    #[test]
    fn false_encode() {
        let value: bool = false;
        let bytes: [u8; 1] = [0];

        assert_eq!(value.encode(), bytes);
    }

    #[test]
    fn true_decode() {
        let value: bool = true;
        let bytes: [u8; 1] = [1];

        let mut decoder = Decoder::new();

        assert_eq!(
            bool::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn false_decode() {
        let value: bool = false;
        let bytes: [u8; 1] = [0];

        let mut decoder = Decoder::new();

        assert_eq!(
            bool::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn string_encode() {
        let value: String = String::from("abcd");
        let bytes: [u8; 12] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x61, 0x62, 0x63, 0x64,
        ];

        assert_eq!(value.encode(), bytes);
    }

    #[test]
    fn empty_string_encode() {
        let value: String = String::from("");
        let bytes: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        assert_eq!(value.encode(), bytes);
    }

    #[test]
    fn string_decode() {
        let value: String = String::from("abcd");
        let bytes: [u8; 12] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x61, 0x62, 0x63, 0x64,
        ];

        let mut decoder = Decoder::new();

        assert_eq!(
            String::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn empty_string_decode() {
        let value: String = String::from("");
        let bytes: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let mut decoder = Decoder::new();

        assert_eq!(
            String::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn vec_encode() {
        let value: Vec<u16> = vec![0x1234, 0x3456, 0x5678, 0x7890];
        let bytes: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x12, 0x34, 0x34, 0x56, 0x56, 0x78,
            0x78, 0x90,
        ];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn empty_vec_encode() {
        let value: Vec<u16> = Vec::new();
        let bytes: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        assert_eq!(value.encode(), bytes)
    }

    #[test]
    fn vec_decode() {
        let value: Vec<u16> = vec![0x1234, 0x3456, 0x5678, 0x7890];
        let bytes: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x12, 0x34, 0x34, 0x56, 0x56, 0x78,
            0x78, 0x90,
        ];

        let mut decoder = Decoder::new();

        assert_eq!(
            Vec::<u16>::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }

    #[test]
    fn empty_vec_decode() {
        let value: Vec<u16> = Vec::new();
        let bytes: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let mut decoder = Decoder::new();

        assert_eq!(
            Vec::<u16>::decode(&mut decoder, &bytes).expect("Decoder didn't return slice"),
            value
        )
    }
}
