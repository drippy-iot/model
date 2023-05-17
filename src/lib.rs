#![no_std]

pub use bitcode::{decode, encode};
use bitcode::{Decode, Encode};

/// Media Access Control address.
pub type MacAddress = [u8; 6];

#[derive(Decode, Encode)]
pub struct Header {
    /// MAC address of the packet as a (sufficently) unique ID.
    pub mac: MacAddress,
    /// Number of seconds since the UNIX epoch.
    pub timestamp: u64,
}

#[derive(Decode, Encode)]
pub enum Payload {
    /// This message informs the Cloud server that the
    /// metro reading conflicts with that of the tap.
    Conflict,
    /// This message is a ping from the unit on the detected flow rate.
    Flow {
        ticks: u64,
    },
}

/// A unit of message sent to the Cloud server.
#[derive(Decode, Encode)]
pub struct Message {
    head: Header,
    data: Payload,
}
