//! A library for generating data structures required to boot iMXRT systems. Intended
//! for generating Rust code in build scripts.
//!
//! # Rationale
//!
//! iMXRT processors require certain data structures in flash in order to configure
//! FlexSPI and / or SEMC peripherals. A FlexSPI Configuration Block (FCB) is an array that
//! describes how the processor should initiate a boot. It's expected to be placed
//! in a certain region of FLASH, with values that describe how a peripheral should
//! interact with NAND- / NOR-based FLASH memory. The raw FCB has a lot of magic
//! numbers, and it would be nice to have an API to generate the FCB.
//!
//! The `imxrt-boot-gen` crate provides an API for generating the FCB. As of this
//! writing, it supports only the generation of an FCB for reading NOR Flash via
//! FlexSPI. Other configurations, such as NAND Flash and / or the SEMC interface,
//! may be added later.
//!
//! # Usage
//!
//! Add `imxrt-boot-gen` to your build dependencies, and select your processor with a feature flag:
//!
//! ```toml
//! [build-dependencies]
//! imxrt-boot-gen = { features = ["imxrt1062"] }
//! ```
//!
//! The rest of this documentation will describe the API for defining a FlexSPI configuration block
//! (FCB).
//!
//! Prepare a `build.rs` script. Import all types from the kind of FCB that you're generating, and
//! create a FCB from an `FCBBuilder`.
//!
//! ```
//! use imxrt_boot_gen::serial_flash::*; // Booting from serial flash
//! # let lookup_table = LookupTable::new();
//! let nor_cb = nor::ConfigurationBlock {
//!     page_size: 256,
//!     sector_size: 4096,
//!     ip_cmd_serial_clk_freq: nor::SerialClockFrequency::MHz30,
//! };
//! let fcb = FCBBuilder::new(DeviceType::SerialNOR(nor_cb), lookup_table)
//!         .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
//!         .cs_hold_time(0x01)
//!         .cs_setup_time(0x02)
//!         .column_address_width(ColumnAddressWidth::OtherDevices)
//!         .device_mode_configuration(DeviceModeConfiguration::Disabled)
//!         .wait_time_cfg_commands(WaitTimeConfigurationCommands::disable())
//!         .flash_size(SerialFlashRegion::A1, 0x0020_0000)
//!         .serial_clk_freq(SerialClockFrequency::MHz60)
//!         .serial_flash_pad_type(FlashPadType::Quad)
//!         .build()
//!         .unwrap();
//! ```
//!
//! The values in the `FCBBuilder`'s will be serialized into the FCB. See the documentation on each
//! field to learn more about the field meaning and possible values. The fields mirror those in your
//! iMXRT's reference manual, so you may consult those docs for more information.
//!
//! The `FCBBuilder` requires a lookup table of type `LookupTable`. The lookup table (LUT) is an array
//! of FlexSPI command sequences that describe how to interact with the external flash controller. We can index
//! a LUT by a command sequence, `CommandSequence`, to associate a `Sequence` with that command. A `Sequence`
//! is a collection of up to eight FlexSPI instructions, `Instr`. Use the `STOP` instructions if you do not need
//! to utilize all eight instructions.
//!
//! ```
//! use imxrt_boot_gen::{
//!     serial_flash::*, // All contents from serial flash
//!     serial_flash::opcodes::sdr::*, // All SDR instruction opcodes
//! };
//!
//! // READ sequence
//! const SEQ_READ: Sequence = Sequence([
//!     Instr::new(CMD, Pads::One, 0xEB),
//!     Instr::new(RADDR, Pads::Four, 0x18),
//!     Instr::new(DUMMY, Pads::Four, 0x06),
//!     Instr::new(READ, Pads::Four, 0x04),
//!     STOP,
//!     STOP,
//!     STOP,
//!     STOP,
//! ]);
//!
//! // ERASE SECTOR sequence
//! const SEQ_ERASE_SECTOR: Sequence = Sequence([
//!     Instr::new(CMD, Pads::One, 0x20),
//!     Instr::new(RADDR, Pads::One, 0x18),
//!     STOP,
//!     STOP,
//!     STOP,
//!     STOP,
//!     STOP,
//!     STOP,
//! ]);
//! // Other sequences...
//!
//! // Add the sequences in the lookup table
//! let mut lookup_table = LookupTable::new();
//! lookup_table[CommandSequence::Read] = SEQ_READ;
//! lookup_table[CommandSequence::EraseSector] = SEQ_ERASE_SECTOR;
//!
//! # let nor_cb = nor::ConfigurationBlock {
//! #     page_size: 256,
//! #     sector_size: 4096,
//! #     ip_cmd_serial_clk_freq: nor::SerialClockFrequency::MHz30,
//! # };
//! let fcb = FCBBuilder::new(DeviceType::SerialNOR(nor_cb), lookup_table)
//!         // Other FCB fields...
//!         .build()
//!         .unwrap();
//! ```
//!
//! The contents of the FlexSPI sequences and instructions will be specific to your flash memory. Consult your chip's
//! documentation for more information. Consult the iMXRT reference manual for more information on the lookup table.
//!
//! Once you've initialized the builder, build the FCB. The FCB implements `Display`, and it will display itself
//! as a Rust array with the ABI guarantees described below.
//!
//! ```no_run
//! # use imxrt_boot_gen::serial_flash::*; // Booting from serial flash
//! use std::fs::File;
//! use std::io::Write;
//!
//! # let nor_cb = nor::ConfigurationBlock {
//! #     page_size: 256,
//! #     sector_size: 4096,
//! #     ip_cmd_serial_clk_freq: nor::SerialClockFrequency::MHz30,
//! # };
//! # let lookup_table = LookupTable::new();
//! # let fcb = FCBBuilder::new(DeviceType::SerialNOR(nor_cb), lookup_table)
//! #         // Other FCB fields...
//! #         .build()
//! #         .unwrap();
//! let mut fcb_rs = File::create("fcb.rs").unwrap();
//! writeln!(fcb_rs, "{}", fcb);
//! ```
//!
//! # ABI
//!
//! The output is a single, 512-byte `u8` array, called `FLEXSPI_CONFIGURATION_BLOCK`.
//! The name is not mangled. It may be referenced in a linker script by its section,
//! `".fcb"`. Given the ABI guarantees, the FCB should be usable from both Rust and C.

mod flexspi_lut;
pub mod serial_flash;
