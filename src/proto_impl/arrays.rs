use crate::hash::hash_str;
use crate::proto::{DslVirtualProto, Primitive, SerializationBuffer, SerializationError};

/*
TODO! Change the implementation of proto, it depends too much on the
SerializationBuffer, so unify SerializationBuffer and serialization logic
(probably by creating intermediate abstractions to RW from just slices/arrays,
and only then the SerializationBuffer).
*/

// impl<'a, const N: usize> DslVirtualProto<'a> for [u8; N] {
//     const HASH: u64 = hash_str("[u8; N]");

//     fn deserialize(buf: &'a mut SerializationBuffer) -> Result<Self, SerializationError> {
//         let _ = u8::deserialize(buf)? as usize;
//         Ok(buf.read_const::<N>()?)
//     }

//     fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), SerializationError> {
//         if N > buf.remaining() {
//             return Err(SerializationError::UnexpectedEof)
//         }

//         (N as u8).serialize(buf)?;
//         buf.write_bytes::<N>(self)?;

//         Ok(())
//     }

//     fn size_hint(&self) -> usize {
//         self.len() + size_of::<u8>()
//     }
// }

// For arrays, size shouldn't actually be sent, because
impl<'a, const N: usize, T: Primitive + DslVirtualProto<'a>> DslVirtualProto<'a> for [T; N] {
    const HASH: u64 = hash_str("[T; N]");

    fn deserialize(buf: &'a mut SerializationBuffer) -> Result<Self, SerializationError> {
        let slice = buf.read_const::<N>()?;
        let mut array: [T; N];
        for (idx, chunk) in slice.chunks_exact(T::SIZE).enumerate() {
            let len = T::SIZE;
            let rev_chunk: [u8; 1] = chunk.try_into().unwrap();
            u8::from_le_bytes(rev_chunk as <u8 as Primitive>::Bytes);
        }
        todo!("See upper message");
    }

    fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), SerializationError> {
        if N > buf.remaining() {
            return Err(SerializationError::UnexpectedEof);
        }

        // buf.write_bytes::<N>(self)?;

        Ok(())
    }

    fn size_hint(&self) -> usize {
        self.len() + size_of::<u8>()
    }
}
