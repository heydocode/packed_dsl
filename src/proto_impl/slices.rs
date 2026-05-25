use crate::proto::{DslVirtualProto, SerializationBuffer, SerializationError};
use crate::hash::hash_str;

impl<'a> DslVirtualProto<'a> for &'a [u8] {
    const HASH: u64 = hash_str("&[u8]");
    
    fn deserialize(buf: &'a mut SerializationBuffer) -> Result<Self, SerializationError> {
        let size = u8::deserialize(buf)? as usize;
        Ok(buf.read(size)?)
    }

    fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), SerializationError> {
        let len = self.len();

        if len > buf.remaining() {
            return Err(SerializationError::UnexpectedEof)
        }
        
        (len as u8).serialize(buf)?;
        buf.write_all(self)?;
        
        Ok(())
    }

    fn size_hint(&self) -> usize {
        self.len() + size_of::<u8>()
    }
}

