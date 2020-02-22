//! Fields specific for NOR flash

/// `ipCmdSerialClkFreq` field for serial NOR-specific FCB
///
/// Chip specific value, not used by ROM
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SerialClockFrequency {
    /// No change, keep current serial clock unchanged
    NoChange = 0,
    MHz30 = 1,
    MHz50 = 2,
    MHz60 = 3,
    MHz75 = 4,
    MHz80 = 5,
    MHz100 = 6,
    #[cfg(any(feature = "imxrt1061", feature = "imxrt1062", feature = "imxrt1064"))]
    MHz120 = 7,
    #[cfg(feature = "imxrt1011")]
    MHz133 = 7,
    #[cfg(any(feature = "imxrt1061", feature = "imxrt1062", feature = "imxrt1064"))]
    MHz133 = 8,
    #[cfg(any(feature = "imxrt1061", feature = "imxrt1062", feature = "imxrt1064"))]
    MHz166 = 9,
}

/// The fields specific for defining a serial NOR FCB
#[derive(Debug, Clone, Copy)]
pub struct ConfigurationBlock {
    pub page_size: u32,
    pub sector_size: u32,
    pub ip_cmd_serial_clk_freq: SerialClockFrequency,
}
