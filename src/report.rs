use bitcode::{Decode, Encode};

/// Media Access Control address.
pub type MacAddress = [u8; 6];

/// Generic ping when there is no data to be sent. Because it sends no data
/// in the payload, this is effectively the common header that is used by all
/// messages in the protocol. This may occur during leak announcements.
#[derive(Debug, Decode, Encode)]
pub struct Ping {
    /// MAC address of the packet as a (sufficently) unique ID.
    mac: MacAddress,
}

/// This message is a ping from the unit on the detected flow rate.
#[derive(Debug, Decode, Encode)]
pub struct Flow {
    /// Common header is the ping message.
    head: Ping,
    /// Detected flow rate measured in revolutions per second
    /// (according to the water flow sensor).
    flow: u64,
}
