//! Serial NOR and NAND FLASH booting
//!
//! The types in `serial_flash` can help you define a FCB suitable for serial
//! NOR- / NAND-flash booting.
//!
//! Note: NAND Flash boot not yet implemented

mod builder;
mod fcb;
mod fields;
mod lookup;
pub mod nor;

pub use builder::*;
pub use fcb::*;
pub use fields::*;
pub use lookup::*;
