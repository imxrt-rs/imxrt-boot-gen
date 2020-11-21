//! Generate data structures for booting i.MX RT processors
//!
//! # Rationale
//!
//! iMXRT processors require certain data structures in flash in order to configure
//! FlexSPI and / or SEMC peripherals. The data structurs must be placed
//! in a certain region of FLASH, with values that describe how a peripheral should
//! interact with NAND- / NOR-based FLASH memory. The data structures have a lot of magic
//! numbers, and it would be nice to have an API to correctly generate the values.
//!
//! The `imxrt-boot-gen` crate helps you make data structures to boot i.MX RT processors.
//! As of this writing, the API supports
//!
//! - serial NOR flash
//!
//! Other configurations, like NAND flash and parallel SEMC, may be added in the future.
//!
//! # Usage
//!
//! Add `imxrt-boot-gen` to your dependencies, and select your processor with a feature flag:
//!
//! ```toml
//! [dependencies]
//! imxrt-boot-gen = { features = ["imxrt1062"] }
//! ```
//!
//! See the module-level documentation for more information about the API.

#![cfg_attr(not(test), no_std)]

pub mod flexspi;
pub mod serial_flash;
