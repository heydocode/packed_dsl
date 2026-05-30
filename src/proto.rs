use arrayvec::ArrayString;
use bitstream_io::{BitRead, BitWrite};

use crate::{DslString, hash::hash_str};

pub trait DslProto<'a>: Sized {
    type Error;
    
    const HASH: u64;
    fn serialize<W: BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error>;
    fn deserialize<R: BitRead + ?Sized>(r: &mut R, buffer: &'a mut [u8; 255]) -> Result<Self, Self::Error>;
}

macro_rules! impl_numeric {
    // usize and isize aren't implemented, because they have width depending
    // on the compilation target
    ($path:path, $($ty:ty),+) => {
        $(
            impl<'a> DslProto<'a> for $ty {
                type Error = $path;
            
                const HASH: u64 = hash_str(stringify!($ty));
            
                fn serialize<W: BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error> {
                    const BITS_NUM: u32 = size_of::<$ty>() as u32 * 8;
                    w.write::<BITS_NUM, $ty>(*self)
                }
            
                fn deserialize<R: BitRead + ?Sized>(r: &mut R, _buffer: &'a mut [u8; 255]) -> Result<Self, Self::Error> {
                    const BITS_NUM: u32 = size_of::<$ty>() as u32 * 8;
                    r.read::<BITS_NUM, $ty>()
                }
            }
        )+
    };
}

impl_numeric!(std::io::Error, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);


impl<'a> DslProto<'a> for DslString {
    type Error = std::io::Error;

    const HASH: u64 = hash_str(stringify!(DslString));

    fn serialize<W: BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error> {
        const BITS_NUM: u32 = size_of::<u8>() as u32 * 8;
        let size = self.len().min(255);
        w.write::<BITS_NUM, u8>(size as u8)?;
        // Truncation if exceeding 255 bytes
        w.write_bytes(&self.as_bytes()[..size])
    }

    fn deserialize<R: BitRead + ?Sized>(r: &mut R, buffer: &'a mut [u8; 255]) -> Result<Self, Self::Error> {
        const BITS_NUM: u32 = size_of::<u8>() as u32 * 8;
        let size_in_bytes = r.read::<BITS_NUM, u8>()?;
        r.read_bytes(&mut buffer[..size_in_bytes as usize])?;
        if let Ok(str) = ArrayString::from_byte_string(&buffer) {
            Ok(str)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "DslString"))
        }
    }
}

impl<'a> DslProto<'a> for &'a [u8] {
    type Error = std::io::Error;

    const HASH: u64 = hash_str(stringify!(&[u8]));

    fn serialize<W: BitWrite + ?Sized>(&self, w: &mut W) -> Result<(), Self::Error> {
        const BITS_NUM: u32 = size_of::<u8>() as u32 * 8;
        let size = self.len().min(255);
        w.write::<BITS_NUM, u8>(size as u8)?;
        // Truncation if exceeding 255 bytes
        w.write_bytes(&self[..size])
    }

    /// Note that the deserialized data should be cloned/copied, as it gets invalid as soon as the buffer changes.
    fn deserialize<R: BitRead + ?Sized>(r: &mut R, buffer: &'a mut [u8; 255]) -> Result<&'a [u8], Self::Error> {
        const BITS_NUM: u32 = size_of::<u8>() as u32 * 8;
        let size_in_bytes = r.read::<BITS_NUM, u8>()?;
        r.read_bytes(&mut buffer[..size_in_bytes as usize])?;
        Ok(&buffer[..size_in_bytes as usize])
    }
}