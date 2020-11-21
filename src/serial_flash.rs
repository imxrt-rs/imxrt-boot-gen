//! Serial NOR flash boot
//!
//! `serial_flash` provides the types necessary to boot an i.MX RT processor
//! from serial NOR flash. *Note: NAND Flash boot not yet implemented.*
//!
//! # Serial NOR Configuration Block
//!
//! To create a serial NOR configuration block, first create a FlexSPI
//! configuration block. See the [`flexspi`](crate::flexspi) module for more details.
//!
//! Use the FlexSPI configuration block to create a Serial NOR configuration
//! block. You are responsible for placing the serial NOR configuration block at the correct
//! location in memory. See [`nor::ConfigurationBlock`] for an example.

pub mod nor;
