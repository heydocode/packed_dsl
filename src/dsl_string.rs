use arrayvec::ArrayString;
use bitstream_io::{BitRead, BitWrite};
use crate::{proto::DslProto, hash::hash_str};

pub type DslString = ArrayString::<255>;

impl<'a> DslProto<'a> for DslString {
    type Error = no_std_io2::io::Error;

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
        r.read_bytes(&mut buffer[..(size_in_bytes as usize)])?;
        if let Ok(array_string) = ArrayString::from_byte_string(&buffer) {
            Ok(array_string)
        } else {
            Err(no_std_io2::io::Error::new(no_std_io2::io::ErrorKind::InvalidData, "DslString"))
        }
    }
}