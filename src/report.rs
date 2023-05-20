use bitcode::{Decode, Encode};

/// This message is a ping from the unit on the detected flow rate.
#[derive(Debug, Decode, Encode, PartialEq, Eq)]
pub struct Flow {
    /// Common header is the ping message.
    pub addr: crate::MacAddress,
    /// Detected flow rate measured in revolutions per second
    /// (according to the water flow sensor).
    #[bitcode_hint(expected_range = "0..16384")]
    pub flow: u16,
}
