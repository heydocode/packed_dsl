use core::{future::ready, marker::PhantomData, ops::{Index, IndexMut}, usize};

use crate::hash::hash_str;

pub enum SerializationError {
    UnexpectedEof,
    /// All DSTs have a maximum size in bytes of 255 (excluding 
    /// the length, which is encoded as a u8).
    DSTLargerThanMaximum,
    BytesToValueConversionFailed,
}

pub struct SerializationBuffer<'a> {
    contents: &'a mut [u8],
    byte_index: usize,
}

impl<'a> SerializationBuffer<'a> {
    pub const fn new(contents: &'a mut [u8]) -> Self {
        Self {
            contents,
            byte_index: 0,
        }
    }

    #[inline(always)]
    pub const fn remaining(&self) -> usize {
        self.contents.len() - self.byte_index
    }

    #[inline(always)]
    const fn advance(&mut self, n: usize) {
        self.byte_index += n;
    }

    pub fn write_all(&mut self, contents: &[u8]) -> Result<(), SerializationError> {
        let len = contents.len();

        if self.remaining() < len {
            return Err(SerializationError::UnexpectedEof);
        }

        let start = self.byte_index;
        let end = start + len;

        self.contents[start..end].copy_from_slice(contents);
        self.advance(len);

        Ok(())
    }

    pub fn write_bytes<const N: usize>(&mut self, bytes: &[u8; N]) -> Result<(), SerializationError> {
        if self.remaining() < N {
            return Err(SerializationError::UnexpectedEof);
        }

        let start = self.byte_index;
        let end = start + N;

        self.contents[start..end].copy_from_slice(bytes);
        self.advance(N);

        Ok(())
    }

    pub fn read(&mut self, n: usize) -> Result<&[u8], SerializationError> {
        if self.remaining() < n {
            return Err(SerializationError::UnexpectedEof);
        }

        let start = self.byte_index;
        let end = start + n;
        
        self.advance(n);

        Ok(&self.contents[start..end])
    }

    /// This copies the needed slice, to avoid panicking when converting this
    /// to a primitive type, while converting from little endian to target endian.
    pub fn read_const<const N: usize>(&mut self) -> Result<[u8; N], SerializationError> {
        if self.remaining() < N {
            return Err(SerializationError::UnexpectedEof);
        }

        let start = self.byte_index;
        let end = start + N;
        
        let mut copy = [0u8; N];
        copy.copy_from_slice(&self.contents[start..end]);
        
        self.advance(N);
        Ok(copy)
    }
}

pub trait DslVirtualProto<'a>: Sized {
    const HASH: u64;

    fn deserialize(buf: &'a mut SerializationBuffer) -> Result<Self, SerializationError>;
    fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), SerializationError>;
    /// Indicates the size that the serialized value will have
    fn size_hint(&self) -> usize;
}

pub trait Primitive: Sized {
    type Bytes;
    
    const SIZE: usize;

    fn from_le_bytes(bytes: Self::Bytes) -> Self;
    fn to_le_bytes(self) -> Self::Bytes;
}