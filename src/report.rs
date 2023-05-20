#[cfg(feature = "sql")]
use postgres_types::{private::BytesMut, FromSql, IsNull, ToSql, Type as PsqlType};

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

#[cfg(feature = "sql")]
impl<'a> FromSql<'a> for Ping {
    fn from_sql(
        ty: &PsqlType,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        assert_eq!(*ty, PsqlType::MACADDR);
        let mut mac = [0; 6];
        mac.copy_from_slice(raw);
        Ok(Self { mac })
    }

    postgres_types::accepts!(MACADDR);
}

#[cfg(feature = "sql")]
impl ToSql for Ping {
    fn to_sql(
        &self,
        ty: &PsqlType,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        out.extend_from_slice(self.mac.as_ref());
        assert_eq!(*ty, PsqlType::MACADDR);
        Ok(IsNull::No)
    }

    postgres_types::accepts!(MACADDR);
    postgres_types::to_sql_checked!();
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
