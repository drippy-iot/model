use bitcode::{Decode, Encode};
use core::fmt;

/// Media Access Control address.
pub type MacAddress = [u8; 6];

/// Generic ping when there is no data to be sent. Because it sends no data
/// in the payload, this is effectively the common header that is used by all
/// messages in the protocol. This may occur during leak announcements.
#[derive(Debug, Decode, Encode, PartialEq, Eq)]
pub struct Ping {
    /// MAC address of the packet as a (sufficently) unique ID.
    pub mac: MacAddress,
}

impl fmt::Display for Ping {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d, e, f] = self.mac;
        write!(fmt, "{a:x}:{b:x}:{c:x}:{d:x}:{e:x}:{f:x}")
    }
}

/// This message is a ping from the unit on the detected flow rate.
#[derive(Debug, Decode, Encode, PartialEq, Eq)]
pub struct Flow {
    /// Common header is the ping message.
    pub head: Ping,
    /// Detected flow rate measured in revolutions per second
    /// (according to the water flow sensor).
    #[bitcode_hint(expected_range = "0..16384")]
    pub flow: u16,
}
