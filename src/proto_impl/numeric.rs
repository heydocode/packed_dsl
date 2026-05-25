use crate::hash::hash_str;
use crate::proto::{DslVirtualProto, Primitive, SerializationBuffer, SerializationError};

macro_rules! impl_numeric {
    (Proto, $($ty: ty),+) => {
        $(
            impl<'a> DslVirtualProto<'a> for $ty {
                const HASH: u64 = hash_str(stringify!($ty));

                fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), SerializationError> {
                    buf.write_bytes(&self.to_le_bytes())
                }

                fn deserialize(buf: &mut SerializationBuffer) -> Result<Self, SerializationError> {
                    const N: usize = size_of::<$ty>();
                    Ok(Self::from_le_bytes(buf.read_const::<N>()?))
                }

                fn size_hint(&self) -> usize {
                    size_of::<Self>()
                }
            }
        )+
    };
    (Primitive, $($ty: ty),+) => {
        $(
            impl Primitive for $ty {
                type Bytes = [u8; size_of::<Self>()];
                const SIZE: usize = size_of::<Self>();

                fn from_le_bytes(bytes: Self::Bytes) -> Self {
                    Self::from_le_bytes(bytes)
                }

                fn to_le_bytes(self) -> Self::Bytes {
                    self.to_le_bytes()
                }
            }
        )+
    };
    () => {
        compile_error!("This macro expects at least one numeric type to implement. Examples: u8, i8, u16, ...");
    };
}

impl_numeric!(
    Proto, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize
);

impl_numeric!(
    Primitive, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize
);