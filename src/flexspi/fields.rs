//! FlexSPI configuration block fields

use core::num::NonZeroU8;

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
    // TODO serial NAND flash values 12 and 13 apply, at a minimum,
    // to the following chips:
    //
    // - imxrt1020
    // - imxrt1170
}

/// Sequence parameter for device mode configuration
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C, packed)]
pub struct DeviceModeSequence {
    /// How many sequences are needed
    /// to execute the command?
    sequence_count: u8,
    /// Where do we start in the LUT?
    sequence_index: u8,
    _reserved: u16,
}

impl DeviceModeSequence {
    /// Create a new sequence parameter for device configuration
    ///
    /// `sequence_index`: starting LUT index of Device mode configuration command
    /// `sequence_count`: number of LUT sequences for Device mode configuration command
    pub const fn new(sequence_count: u8, sequence_index: u8) -> Self {
        Self {
            sequence_count,
            sequence_index,
            _reserved: 0,
        }
    }

    pub(crate) const fn zeroed() -> Self {
        Self::new(0, 0)
    }
}

/// Configuration commands to augment LUT sequences.
pub type ConfigurationCommand = DeviceModeSequence;

/// Describes both the `deviceModeCfgEnable` field, and
/// the `deviceModeArg` field, which is only valid if
/// the configuration is enabled.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DeviceModeConfiguration {
    /// Device configuration mode is disabled
    #[default]
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

/// Wait time for all configuration commands
///
/// From the docs...
///
/// > Available for device that support v1.1.0 FlexSPI configuration block.
/// > If it is greater than 0, ROM will wait waitTimeCfgCommands * 100us
/// > for all device memory configuration commands instead of using read
/// > status to wait until these commands complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct WaitTimeConfigurationCommands(u16);
impl WaitTimeConfigurationCommands {
    pub const fn disable() -> Self {
        WaitTimeConfigurationCommands(0)
    }

    /// Computes the wait time from the specified `wait_time_us` (microseconds)
    ///
    /// The duration should be divisible by `100us`, since the
    /// value is a factor scaled by `100us`
    pub const fn new(wait_time_us: u16) -> Self {
        WaitTimeConfigurationCommands(wait_time_us / 100)
    }
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

/// Options for the serial clock frequency.
///
/// Use this with an [`Imxrt`](crate::Imxrt) to produce
/// a [`SerialClockFrequency`]. Note that not all options
/// are valid for all parts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SerialClockOption {
    MHz30,
    MHz50,
    MHz60,
    MHz75,
    MHz80,
    MHz100,
    MHz120,
    MHz133,
    MHz166,
}

/// Serial clock frequency for flash read / write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SerialClockFrequency(pub(crate) NonZeroU8);

impl SerialClockFrequency {
    /// Returns the raw value for this clock frequency enum.
    pub const fn get(self) -> u8 {
        self.0.get()
    }
}

/// A FlexSPI serial flash region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum SerialFlashRegion {
    A1,
    A2,
    B1,
    B2,
}
