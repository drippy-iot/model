#![cfg_attr(not(feature = "report"), no_std)]

#[cfg(feature = "report")]
pub mod report;

#[cfg(any(feature = "report"))]
pub use bitcode::{decode, encode};

use bitcode::{Decode, Encode};
use core::fmt;

#[cfg(feature = "sql")]
use postgres_types::{private::BytesMut, FromSql, IsNull, ToSql, Type as PsqlType};

/// Media Access Control
#[derive(Debug, Decode, Encode, PartialEq, Eq)]
pub struct MacAddress([u8; 6]);

impl fmt::Display for MacAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d, e, f] = self.0;
        write!(fmt, "{a:x}:{b:x}:{c:x}:{d:x}:{e:x}:{f:x}")
    }
}

#[cfg(feature = "sql")]
impl<'a> FromSql<'a> for MacAddress {
    fn from_sql(
        ty: &PsqlType,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        assert_eq!(*ty, PsqlType::MACADDR);
        let mut mac = [0; 6];
        mac.copy_from_slice(raw);
        Ok(Self(mac))
    }

    postgres_types::accepts!(MACADDR);
}

#[cfg(feature = "sql")]
impl ToSql for MacAddress {
    fn to_sql(
        &self,
        ty: &PsqlType,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        out.extend_from_slice(self.0.as_ref());
        assert_eq!(*ty, PsqlType::MACADDR);
        Ok(IsNull::No)
    }

    postgres_types::accepts!(MACADDR);
    postgres_types::to_sql_checked!();
}
