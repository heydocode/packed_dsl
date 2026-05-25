use std::string::String;
use std::vec::Vec;

use crate::proto::{DslVirtualProto, SerializationBuffer, SerializationError};
use crate::hash::hash_str;

impl<'a> DslVirtualProto<'a> for String {
    const HASH: u64 = hash_str("String");
    
    fn deserialize(buf: &'a mut SerializationBuffer) -> Result<String, SerializationError> {
        let size = u8::deserialize(buf)? as usize;
        let bytes = Vec::from(buf.read(size)?);

        String::from_utf8(bytes)
            .map_err(|_| SerializationError::BytesToValueConversionFailed)
    }

    fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), SerializationError> {
        let len = self.len();

        if len > buf.remaining() {
            return Err(SerializationError::UnexpectedEof)
        }
        
        (len as u8).serialize(buf)?;
        buf.write_all(self.as_bytes())?;
        
        Ok(())
    }

    fn size_hint(&self) -> usize {
        self.len() + size_of::<u8>()
    }
}