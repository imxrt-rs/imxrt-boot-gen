//! FCB fields
//!
//! The module implements and documents the common FCB fields.

use std::convert::TryFrom;
use std::ops::{Index, IndexMut};
use std::time::Duration;

/// `readSampleClkSrc` of the general FCB   
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ReadSampleClockSource {
    InternalLoopback = 0x00,
    LoopbackFromDQSPad = 0x01,
    FlashProvidedDQS = 0x03,
}

/// `columnAdressWidth`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ColumnAddressWidth {
    OtherDevices = 0,
    Hyperflash = 3,
}

/// Sequence parameter for device mode configuration
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DeviceModeSequence(pub(crate) [u8; 4]);
impl DeviceModeSequence {
    /// Create a new sequence parameter for device configuration
    ///
    /// `starting_lut_index`: starting LUT index of Device mode configuration command
    /// `number_of_luts`: number of LUT sequences for Device mode configuration command
    pub fn new(number_of_luts: u8, starting_lut_index: u8) -> Self {
        DeviceModeSequence(
            ((u32::from(starting_lut_index) << 8) | u32::from(number_of_luts)).to_le_bytes(),
        )
    }
}

/// Describes both the `deviceModeCfgEnable` field, and
/// the `deviceModeArg` field, which is only valid if
/// the configuration is enabled.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DeviceModeConfiguration {
    /// Device configuration mode is disabled
    Disabled,
    /// Device configuration mode is enabled
    ///
    /// Tells the processor to use the associated device mode argument and sequence
    Enabled {
        /// `deviceModeArg`
        device_mode_arg: u32,
        /// `deviceModeSeq`
        device_mode_seq: DeviceModeSequence,
    },
}

impl Default for DeviceModeConfiguration {
    fn default() -> Self {
        DeviceModeConfiguration::Disabled
    }
}

/// Wait time for all configuration commands
///
/// From the docs...
///
/// > Available for device that support v1.1.0 FlexSPI configuration block.
/// > If it is greater than 0, ROM will wait waitTimeCfgCommands * 100us
/// > for all device memory configuration commands instead of using read
/// > status to wait until these commands complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitTimeConfigurationCommands(pub(crate) u16);
impl WaitTimeConfigurationCommands {
    pub fn disable() -> Self {
        WaitTimeConfigurationCommands(0)
    }

    /// Computes the wait time from the specified `wait_time`. The
    /// provided duration should be divisible by `100us`, since the
    /// value is a factor scaled by `100us`. Returns `None` if representing
    /// this as a factor of `100us` returns `0`, or if the factor cannot be
    /// expressed in a `u16`.
    pub fn from_duration(wait_time: Duration) -> Option<Self> {
        let us = wait_time.as_micros();
        if us < 100 {
            None
        } else {
            let factor = u16::try_from(us / 100).ok()?;
            Some(WaitTimeConfigurationCommands(factor))
        }
    }
}

/// Describes the `deviceType` field.
///
/// Only the SerialNOR is implemented; `DeviceType`
/// may also have `SerialNAND` in the future.
#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    SerialNOR(super::nor::ConfigurationBlock),
}

/// `sFlashPad` field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FlashPadType {
    Single = 1,
    Dual = 2,
    Quad = 4,
    Octal = 8,
}

/// `serialClkFreq`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SerialClockFrequency {
    MHz30 = 1,
    MHz50 = 2,
    MHz60 = 3,
    MHz75 = 4,
    MHz80 = 5,
    MHz100 = 6,
    MHz120 = 7,
    MHz133 = 8,
    #[cfg(any(feature = "imxrt1061", feature = "imxrt1062", feature = "imxrt1064"))]
    MHz166 = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum SerialFlashRegion {
    A1,
    A2,
    B1,
    B2,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SerialFlashSize(pub(crate) [u32; 4]);

impl Index<SerialFlashRegion> for SerialFlashSize {
    type Output = u32;
    fn index(&self, region: SerialFlashRegion) -> &u32 {
        &self.0[region as usize]
    }
}

impl IndexMut<SerialFlashRegion> for SerialFlashSize {
    fn index_mut(&mut self, region: SerialFlashRegion) -> &mut u32 {
        &mut self.0[region as usize]
    }
}

impl SerialFlashSize {
    pub(crate) fn new() -> Self {
        SerialFlashSize::default()
    }
}
