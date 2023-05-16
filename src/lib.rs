#![no_std]

pub use bitcode::{decode, encode};
use bitcode::{Decode, Encode};

/// Media Access Control address.
pub type MacAddress = [u8; 6];

#[derive(Decode, Encode)]
pub struct Header {
    /// MAC address of the packet as a (sufficently) unique ID.
    mac: MacAddress,
    /// Number of seconds since the UNIX epoch.
    timestamp: u64,
}
