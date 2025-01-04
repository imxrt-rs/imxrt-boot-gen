//! Serial NOR configuration blocks and fields

use crate::flexspi;

/// `ipCmdSerialClkFreq` field for serial NOR-specific FCB
///
/// Chip specific value, not used by ROM.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SerialClockFrequency {
    /// No change, keep current serial clock unchanged
    NoChange = 0,
    MHz30,
    MHz50,
    MHz60,
    #[cfg(not(any(feature = "imxrt1170", feature = "imxrt1180")))]
    MHz75,
    MHz80,
    MHz100,
    #[cfg(any(
        feature = "imxrt1040",
        feature = "imxrt1060",
        feature = "imxrt1064",
        feature = "imxrt1170",
        feature = "imxrt1180"
    ))]
    MHz120,
    MHz133,
    #[cfg(any(
        feature = "imxrt1040",
        feature = "imxrt1050",
        feature = "imxrt1060",
        feature = "imxrt1064"
    ))]
    MHz166,
}

/// A serial NOR configuration block
///
/// This is the memory that you'll need to properly place in memory in order to
/// boot your i.MX RT system. Consider keeping the symbol name, and specifying
/// a link section, so that you can more easily place the memory in your linker
/// script.
///
/// Unless otherwise specified, all unset fields are set to a bitpattern of zero.
///
/// ## 1170 notes
///
/// By default, `isUniformBlockSize` is set to 1, indicating that the block size and
/// sector sizes are equal. Using `block_size` clears this field and allows you to
/// differentiate the block size from the sector size.
///
/// ```no_run
/// use imxrt_boot_gen::serial_flash::nor;
/// # use imxrt_boot_gen::flexspi::{self, LookupTable};
///
/// # const FLEXSPI_CONFIGURATION_BLOCK: flexspi::ConfigurationBlock = flexspi::ConfigurationBlock::new(LookupTable::new());
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
    mem_cfg: flexspi::ConfigurationBlock,
    page_size: u32,
    sector_size: u32,
    ip_cmd_serial_clk_freq: SerialClockFrequency,
    extras: Extras,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct Imxrt11xxExtras {
    is_uniform_block_size: u8,
    is_data_order_swapped: u8,
    _reserved0: [u8; 5],
    block_size: u32,
    flash_state_ctx: u32,
    _reserved1: [u8; 40],
}

const _: () = assert!(55 == core::mem::size_of::<Imxrt11xxExtras>());

#[cfg(any(feature = "imxrt1170", feature = "imxrt1180"))]
type Extras = Imxrt11xxExtras;

#[cfg(not(any(feature = "imxrt1170", feature = "imxrt1180")))]
type Extras = [u8; core::mem::size_of::<Imxrt11xxExtras>()];

const fn extras() -> Extras {
    #[cfg(any(feature = "imxrt1170", feature = "imxrt1180"))]
    {
        Extras {
            // By default, signal that block size equals sector size.
            is_uniform_block_size: 1u8,
            is_data_order_swapped: 0u8,
            _reserved0: [0u8; 5],
            block_size: 0u32,
            flash_state_ctx: 0u32,
            _reserved1: [0u8; 40],
        }
    }
    #[cfg(not(any(feature = "imxrt1170", feature = "imxrt1180")))]
    {
        [0u8; core::mem::size_of::<Imxrt11xxExtras>()]
    }
}

impl ConfigurationBlock {
    /// Create a new serial NOR configuration block based on the FlexSPI configuration
    /// block
    pub const fn new(mut mem_cfg: flexspi::ConfigurationBlock) -> Self {
        mem_cfg.device_type = 1;
        ConfigurationBlock {
            mem_cfg,
            page_size: 0,
            sector_size: 0,
            ip_cmd_serial_clk_freq: SerialClockFrequency::NoChange,
            extras: extras(),
        }
    }
    /// Set the serial NOR page size
    pub const fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }
    /// Set the serial NOR sector size
    pub const fn sector_size(mut self, sector_size: u32) -> Self {
        self.sector_size = sector_size;
        self
    }
    /// Set the serial clock frequency
    pub const fn ip_cmd_serial_clk_freq(
        mut self,
        serial_clock_frequency: SerialClockFrequency,
    ) -> Self {
        self.ip_cmd_serial_clk_freq = serial_clock_frequency;
        self
    }
}

#[cfg(any(feature = "imxrt1170", feature = "imxrt1180"))]
impl ConfigurationBlock {
    /// Set the serial NOR block size if it differs from the sector size.
    ///
    /// By default, the configuration block signals to the hardware that the
    /// sector size is the same as the block size. Calling this will override
    /// that setting, allowing you to configure a different block size.
    ///
    /// The behavior is unspecified if you call this with a block size that's
    /// equal to the sector size.
    pub const fn block_size(mut self, block_size: u32) -> Self {
        self.extras.is_uniform_block_size = 0u8;
        self.extras.block_size = block_size;
        self
    }
}

const _STATIC_ASSERT_SIZE: [u32; 1] =
    [0; (core::mem::size_of::<ConfigurationBlock>() == 512) as usize];

#[cfg(test)]
mod test {
    use super::{flexspi, ConfigurationBlock, SerialClockFrequency};
    use crate::flexspi::LookupTable;

    #[test]
    fn smoke() {
        const _CFG: ConfigurationBlock =
            ConfigurationBlock::new(flexspi::ConfigurationBlock::new(LookupTable::new()))
                .page_size(256)
                .sector_size(4095)
                .ip_cmd_serial_clk_freq(SerialClockFrequency::MHz30);
    }
}
