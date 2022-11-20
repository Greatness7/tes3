#![cfg_attr(feature = "nightly", feature(min_specialization))]

mod bytes_io;
pub use crate::bytes_io::*;
pub use bytes_io_macros::*;
