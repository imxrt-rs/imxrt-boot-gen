//! Generate i.MX RT boot-time data structures.
//!
//! # Rationale
//!
//! i.MX RT processors require certain data structures in order to configure
//! FlexSPI and SEMC peripherals. The data structures must be placed
//! in a certain region of memory with values that describe how a peripheral should
//! interact with external storage. The data structures only support certain values,
//! and need a particular layout in order to boot the system.
//!
//! The `imxrt-boot-gen` crate helps you generate data structures to boot i.MX RT processors.
//! As of this writing, the API supports
//!
//! - serial NOR flash via FlexSPI
//!
//! Other configurations, like NAND flash and parallel SEMC, may be added in the future.
//!
//! `imxrt-boot-gen` does not perscribe a way to properly place these data structures in a
//! firmware image. Consider using [`imxrt-rt`](https://docs.rs/imxrt-rt) if you need
//! a runtime that can place these data structures in your firmware image.
//!
//! # Usage
//!
//! Add `imxrt-boot-gen` to your dependencies:
//!
//! ```toml
//! [dependencies]
//! imxrt-boot-gen = # ...
//! ```
//!
//! The entire API is `const`. You may define your data structures at compile
//! time, and assign the values to `static` memory in your embedded program.
//!
//! See the module-level documentation for more information about the API.
//!
//! ## License
//!
//! Licensed under either of
//!
//! - [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](./LICENSE-APACHE))
//! - [MIT License](http://opensource.org/licenses/MIT) ([LICENSE-MIT](./LICENSE-MIT))
//!
//! at your option.
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.

#![cfg_attr(not(test), no_std)]

use core::num::NonZeroU8;

use flexspi::{SerialClockFrequency, SerialClockOption};
use serial_flash::nor::IpSerialClockFrequency;

pub mod flexspi;
pub mod serial_flash;

/// The MCU family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Imxrt {
    /// A 1010 MCU.
    Imxrt1010,
    /// A 1020 MCU.
    ///
    /// This should also work for the 1024 MCUs.
    Imxrt1020,
    /// A 1040 MCU.
    Imxrt1040,
    /// A 1050 MCU.
    Imxrt1050,
    /// A 1060 MCU.
    ///
    /// This is expected to work with 1064 MCUs.
    Imxrt1060,
    /// A 1160 MCU, booting core.
    Imxrt1160,
    /// A 1170 MCU, booting core.
    Imxrt1170,
    /// A 1180 MCU, booting core.
    Imxrt1180,
}

/// An error during generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// This isn't supported for your selected chip.
    NotSupportedForChip,
}

impl Imxrt {
    /// Produce a serial clock frequency for most flash access.
    ///
    /// If your chip doesn't support the given option, this returns
    /// [`Error::NotSupportedForChip`].
    pub const fn try_serial_clock_frequency(
        self,
        opt: SerialClockOption,
    ) -> Result<SerialClockFrequency, Error> {
        let [scf, _] = match self {
            Self::Imxrt1010 => Self::imxrt1010(opt),
            Self::Imxrt1020 => Self::imxrt1020(opt),
            Self::Imxrt1040 => Self::imxrt1040(opt),
            Self::Imxrt1050 => Self::imxrt1050(opt),
            Self::Imxrt1060 => Self::imxrt1060(opt),
            Self::Imxrt1160 => Self::imxrt1160(opt),
            Self::Imxrt1170 => Self::imxrt1170(opt),
            Self::Imxrt1180 => Self::imxrt1180(opt),
        };

        let Some(scf) = NonZeroU8::new(scf) else {
            return Err(Error::NotSupportedForChip);
        };

        Ok(SerialClockFrequency(scf))
    }

    /// Produce a serial clock frequency for most flash accesses.
    ///
    /// # Panics
    ///
    /// Panics if the chip doesn't support the serial clock option.
    pub const fn serial_clock_frequency(self, opt: SerialClockOption) -> SerialClockFrequency {
        match self.try_serial_clock_frequency(opt) {
            Ok(freq) => freq,
            // Maybe someday, we can be more specific in a const fn.
            Err(_) => panic!("This chip doesn't support that frequency"),
        }
    }

    /// Produce a serial clock frequency for IP access.
    pub const fn try_ip_serial_clock_frequency(
        self,
        opt: SerialClockOption,
    ) -> Result<IpSerialClockFrequency, Error> {
        let [_, ipscf] = match self {
            Self::Imxrt1010 => Self::imxrt1010(opt),
            Self::Imxrt1020 => Self::imxrt1020(opt),
            Self::Imxrt1040 => Self::imxrt1040(opt),
            Self::Imxrt1050 => Self::imxrt1050(opt),
            Self::Imxrt1060 => Self::imxrt1060(opt),
            Self::Imxrt1160 => Self::imxrt1160(opt),
            Self::Imxrt1170 => Self::imxrt1170(opt),
            Self::Imxrt1180 => Self::imxrt1180(opt),
        };

        let Some(ipscf) = NonZeroU8::new(ipscf) else {
            return Err(Error::NotSupportedForChip);
        };

        Ok(IpSerialClockFrequency(ipscf))
    }

    /// Produce a serial clock frequency for IP access.
    ///
    /// # Panics
    ///
    /// Panics if the chip doesn't support the serial clock option.
    pub const fn ip_serial_clock_frequency(self, opt: SerialClockOption) -> IpSerialClockFrequency {
        match self.try_ip_serial_clock_frequency(opt) {
            Ok(freq) => freq,
            Err(_) => panic!("This chip doesn't support that frequency"),
        }
    }

    //
    // Element 0 => serial clock enum
    // Element 1 => IP serial clock enum
    //
    // If the value isn't supported, return zero.
    //

    const fn imxrt1010(opt: SerialClockOption) -> [u8; 2] {
        match opt {
            SerialClockOption::MHz30 => [1, 1],
            SerialClockOption::MHz50 => [2, 2],
            SerialClockOption::MHz60 => [3, 3],
            SerialClockOption::MHz75 => [4, 4],
            SerialClockOption::MHz80 => [5, 5],
            SerialClockOption::MHz100 => [6, 6],
            SerialClockOption::MHz120 => [7, 0],
            SerialClockOption::MHz133 => [8, 7],
            SerialClockOption::MHz166 => [0, 0],
        }
    }

    const fn imxrt1020(opt: SerialClockOption) -> [u8; 2] {
        match opt {
            SerialClockOption::MHz30 => [1, 1],
            SerialClockOption::MHz50 => [2, 2],
            SerialClockOption::MHz60 => [3, 3],
            SerialClockOption::MHz75 => [4, 4],
            SerialClockOption::MHz80 => [5, 5],
            SerialClockOption::MHz100 => [6, 6],
            SerialClockOption::MHz120 => [0, 0],
            SerialClockOption::MHz133 => [7, 7],
            SerialClockOption::MHz166 => [8, 8],
        }
    }

    const fn imxrt1040(opt: SerialClockOption) -> [u8; 2] {
        match opt {
            SerialClockOption::MHz30 => [1, 1],
            SerialClockOption::MHz50 => [2, 2],
            SerialClockOption::MHz60 => [3, 3],
            SerialClockOption::MHz75 => [4, 4],
            SerialClockOption::MHz80 => [5, 5],
            SerialClockOption::MHz100 => [6, 6],
            SerialClockOption::MHz120 => [7, 7],
            SerialClockOption::MHz133 => [8, 8],
            SerialClockOption::MHz166 => [9, 9],
        }
    }

    const fn imxrt1050(opt: SerialClockOption) -> [u8; 2] {
        Self::imxrt1020(opt)
    }

    const fn imxrt1060(opt: SerialClockOption) -> [u8; 2] {
        Self::imxrt1040(opt)
    }

    const fn imxrt1160(opt: SerialClockOption) -> [u8; 2] {
        match opt {
            SerialClockOption::MHz30 => [1, 1],
            SerialClockOption::MHz50 => [2, 2],
            SerialClockOption::MHz60 => [3, 3],
            SerialClockOption::MHz75 => [0, 0],
            SerialClockOption::MHz80 => [4, 4],
            SerialClockOption::MHz100 => [5, 5],
            SerialClockOption::MHz120 => [6, 6],
            SerialClockOption::MHz133 => [7, 7],
            SerialClockOption::MHz166 => [8, 8],
        }
    }

    const fn imxrt1170(opt: SerialClockOption) -> [u8; 2] {
        Self::imxrt1160(opt)
    }

    const fn imxrt1180(opt: SerialClockOption) -> [u8; 2] {
        Self::imxrt1160(opt)
    }
}
