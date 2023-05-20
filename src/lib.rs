#![no_std]

#[cfg(feature = "report")]
pub mod report;

#[cfg(any(feature = "report"))]
pub use bitcode::{decode, encode};
