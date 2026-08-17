mod archive;
mod builder;
mod hash;

pub use archive::{Archive, ArchiveAnomalies, Entry};
pub use builder::Builder;
pub use hash::{FileHash, hash_path};

use std::io;

pub(crate) const VERSION: u32 = 0x0000_0100;
pub(crate) const HEADER_SIZE: u64 = 12;

pub(crate) fn invalid_data_error(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

pub(crate) fn invalid_data<T>(message: &'static str) -> io::Result<T> {
    Err(invalid_data_error(message))
}
