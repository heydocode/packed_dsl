#![no_std]

pub mod hash;
mod proto;
mod proto_impl;

#[cfg(feature = "std")]
extern crate std;
