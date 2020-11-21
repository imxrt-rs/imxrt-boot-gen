//! Generate data structures for booting i.MX RT processors
//!
//! # Rationale
//!
//! i.MX RT processors require certain data structures in order to configure
//! FlexSPI and / or SEMC peripherals. The data structurs must be placed
//! in a certain region of FLASH, with values that describe how a peripheral should
//! interact with persistent memory. The data structures have a lot of magic
//! numbers, and need a very particular layout in order to boot the system.
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
//! imxrt-boot-gen = { features = ["imxrt1060"] }
//! ```
//!
//! The entire API is `const`. You may define your data structures at compile
//! time, and assign them to `static` memory in your embedded program.
//!
//! See the module-level documentation for more information about the API.

#![cfg_attr(not(test), no_std)]

pub mod flexspi;
pub mod serial_flash;
