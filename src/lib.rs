//!
//! A library for working with content from [The Elder Scrolls 3: Morrowind](https://en.wikipedia.org/wiki/The_Elder_Scrolls_III:_Morrowind).
//!

/// Module for working with `.esp` files.
#[cfg(feature = "esp")]
pub use esp;

/// Module for working with `.nif` files.
#[cfg(feature = "nif")]
pub use nif;
