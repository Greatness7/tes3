mod bytes_io;
pub use crate::bytes_io::*;
pub use bytes_io_macros::*;
/// The code pages a `Reader` reads strings in and a `Writer` writes them in (`encoding`,
/// Windows-1252 by default): `encoding_rs::WINDOWS_1251` for a Russian install, and so on.
pub use encoding_rs::{self, Encoding};
