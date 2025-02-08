//! FlexSPI configuration block fields

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

/// `serialClkFreq`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SerialClockFrequency {
    MHz30 = 1,
    MHz50,
    MHz60,
    #[cfg(not(any(feature = "imxrt1160", feature = "imxrt1170", feature = "imxrt1180")))]
    MHz75,
    MHz80,
    MHz100,
    #[cfg(any(
        feature = "imxrt1010",
        feature = "imxrt1040",
        feature = "imxrt1060",
        feature = "imxrt1064",
        feature = "imxrt1160",
        feature = "imxrt1170",
        feature = "imxrt1180"
    ))]
    MHz120,
    MHz133,
    #[cfg(not(feature = "imxrt1010"))]
    MHz166,
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
