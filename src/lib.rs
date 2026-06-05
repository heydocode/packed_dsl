#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod proto;
pub mod hash;
pub mod dsl_string;

#[cfg(test)]
mod tests;

pub use no_std_io2;
pub use bitstream_io;
pub use arrayvec;
pub use packed_dsl_macros as macros;