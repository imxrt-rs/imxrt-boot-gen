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
#[repr(transparent)]
pub struct DeviceModeSequence([u8; 4]);
impl DeviceModeSequence {
    /// Create a new sequence parameter for device configuration
    ///
    /// `starting_lut_index`: starting LUT index of Device mode configuration command
    /// `number_of_luts`: number of LUT sequences for Device mode configuration command
    pub const fn new(number_of_luts: u8, starting_lut_index: u8) -> Self {
        DeviceModeSequence(
            (((starting_lut_index as u32) << 8) | (number_of_luts as u32)).to_le_bytes(),
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
    #[cfg(not(feature = "imxrt1170"))]
    MHz75,
    MHz80,
    MHz100,
    #[cfg(not(feature = "imxrt1020"))]
    MHz120,
    MHz133,
    #[cfg(any(
        feature = "imxrt1020",
        feature = "imxrt1060",
        feature = "imxrt1064",
        feature = "imxrt1170"
    ))]
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
