use bitcode::{Decode, Encode};

/// This message is a ping from the unit on the detected flow rate.
#[derive(Debug, Decode, Encode, PartialEq, Eq)]
pub struct Ping {
    /// Common header is the ping message.
    pub addr: crate::MacAddress,
    /// Detected flow rate measured in revolutions per second
    /// (according to the water flow sensor).
    pub flow: u16,
    /// Checks whether a leak was detected since the last ping.
    pub leak: bool,
}
