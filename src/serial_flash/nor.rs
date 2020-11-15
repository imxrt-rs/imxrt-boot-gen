//! Fields specific for NOR flash

use super::FlexSPIConfigurationBlock;

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
#[repr(C, packed)]
pub struct ConfigurationBlock {
    mem_cfg: FlexSPIConfigurationBlock,
    page_size: u32,
    sector_size: u32,
    ip_cmd_serial_clk_freq: u32,
    _reserved: [u8; 52],
}

impl ConfigurationBlock {
    pub const fn new(mut mem_cfg: FlexSPIConfigurationBlock) -> Self {
        mem_cfg.device_type = 1;
        ConfigurationBlock {
            mem_cfg,
            page_size: 0,
            sector_size: 0,
            ip_cmd_serial_clk_freq: 0,
            _reserved: [0; 52],
        }
    }
    pub const fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }
    pub const fn sector_size(mut self, sector_size: u32) -> Self {
        self.sector_size = sector_size;
        self
    }
    pub const fn ip_cmd_serial_clk_freq(
        mut self,
        serial_clock_frequency: SerialClockFrequency,
    ) -> Self {
        self.ip_cmd_serial_clk_freq = serial_clock_frequency as u32;
        self
    }
}

const _STATIC_ASSERT_SIZE: [u32; 1] =
    [0; (core::mem::size_of::<ConfigurationBlock>() == 512) as usize];

#[cfg(test)]
mod test {
    use super::{
        super::LookupTable, ConfigurationBlock, FlexSPIConfigurationBlock, SerialClockFrequency,
    };

    #[test]
    fn smoke() {
        const _CFG: ConfigurationBlock =
            ConfigurationBlock::new(FlexSPIConfigurationBlock::new(LookupTable::new()))
                .page_size(256)
                .sector_size(4095)
                .ip_cmd_serial_clk_freq(SerialClockFrequency::MHz30);
    }
}
