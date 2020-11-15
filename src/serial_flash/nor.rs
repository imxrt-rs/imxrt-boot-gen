//! Serial NOR configuration blocks and fields

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

/// A serial NOR configuration block
///
/// This is the memory that you'll need to properly place in memory in order to
/// boot your i.MX RT system. Consider keeping the symbol name, and specifying
/// a link section, so that you can more easily place the memory in your linker
/// script.
///
/// ```no_run
/// use imxrt_boot_gen::serial_flash::nor;
/// # use imxrt_boot_gen::serial_flash::{FlexSPIConfigurationBlock, LookupTable};
///
/// # const FLEXSPI_CONFIGURATION_BLOCK: FlexSPIConfigurationBlock = FlexSPIConfigurationBlock::new(LookupTable::new());
/// #[no_mangle]
/// #[link_section = ".serial_nor_cb"]
/// static SERIAL_NOR_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
///     nor::ConfigurationBlock::new(FLEXSPI_CONFIGURATION_BLOCK)
///         .page_size(256)
///         .sector_size(4096)
///         .ip_cmd_serial_clk_freq(nor::SerialClockFrequency::MHz30);
/// ```
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
