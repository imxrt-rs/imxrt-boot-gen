//! A library for generating an iMXRT Firmware Configuration Block (FCB)
//! from Rust. Intended to be used in `build.rs` build scripts to generate
//! FCBs for iMXRT-based systems.
//!
//! # Rationale
//!
//! The iMXRT Firmware Configuration Block (FCB) is an array that
//! describes how the processor should initiate a boot. It's expected to be placed
//! in a certain region of FLASH, with values that describe how a peripheral should
//! interact with NAND- / NOR-based FLASH memory. The FCB has a lot of magic
//! numbers, and it would be nice to have an API to generate the FCB.
//!
//! The `imxrt-fcb-gen` crate provides an API for generating the FCB. As of this
//! writing, it supports only the generation of an FCB for reading NOR Flash via
//! FlexSPI. Other configurations, such as NAND Flash and / or the SEMC interface,
//! may be added later.
//!
//! # Usage
//!
//! Add `imxrt-fcb-gen` to your build dependencies:
//!
//! ```toml
//! [build-dependencies]
//! imxrt-fcb-gen = { path = "../imxrt-fcb-gen" }
//! ```
//!
//! Then, instantiate a `Builder` in your `build.rs` build script. Unless a
//! member of the `Builder` implements `Default`, it's a required value.
//! See the `iMXRT` reference manual for details that may be missing from this library.
//!
//! Once the builder is finished, write the FCB instance from your build script, and
//! include it using the `include!` macro in your crate. See the `teensy4-fcb`
//! crate for an example.
//!
//! # ABI
//!
//! The output is a single `u8` array, called `FIRMWARE_CONFIGURATION_BLOCK`.
//! The name is not mangled. It may be referenced in a linker script by its section,
//! `".fcb"`. These characteristics may be modified by the methods on the builder.

#[macro_use]
mod macros;

mod fcb;
mod flexspi_lut;
pub mod serial_flash;
