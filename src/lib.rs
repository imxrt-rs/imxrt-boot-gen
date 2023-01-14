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
//! Add `imxrt-boot-gen` to your dependencies, and select your processor with a feature flag:
//!
//! ```toml
//! [dependencies]
//! imxrt-boot-gen = { features = ["imxrt1060"] }
//! ```
//!
//! The entire API is `const`. You may define your data structures at compile
//! time, and assign the values to `static` memory in your embedded program.
//!
//! See the module-level documentation for more information about the API.
//!
//! # Features
//!
//! The crate *requires* a feature selection. Features correlate to i.MX RT processor families.
//! The supported features are listed below.
//!
//! - `"imxrt1010"`
//! - `"imxrt1020"`
//! - `"imxrt1060"`
//! - `"imxrt1064"`
//! - `"imxrt1170"`
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

pub mod flexspi;
pub mod serial_flash;
