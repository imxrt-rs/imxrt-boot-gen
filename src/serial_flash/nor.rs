//! Serial NOR configuration blocks and fields

use core::num::NonZeroU8;

use crate::{Imxrt, flexspi};

/// The serial clock frequency for IP access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct IpSerialClockFrequency(pub(crate) NonZeroU8);

impl IpSerialClockFrequency {
    /// Returns the raw enum value for this frequency selection.
    pub const fn get(self) -> u8 {
        self.0.get()
    }
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
/// ## 1160, 1170, and 1180 notes
///
/// By default, `isUniformBlockSize` is set to 1, indicating that the block size and
/// sector sizes are equal. Using `block_size` clears this field and allows you to
/// differentiate the block size from the sector size.
///
/// ```no_run
/// use imxrt_boot_gen::serial_flash::nor;
/// use imxrt_boot_gen::{Imxrt, flexspi::SerialClockOption};
/// # use imxrt_boot_gen::flexspi::{self, LookupTable};
///
/// const CHIP: Imxrt = // ...
/// # Imxrt::Imxrt1170;
///
/// # const FLEXSPI_CONFIGURATION_BLOCK: flexspi::ConfigurationBlock = flexspi::ConfigurationBlock::new(LookupTable::new());
/// #[unsafe(no_mangle)]
/// #[unsafe(link_section = ".serial_nor_cb")]
/// static SERIAL_NOR_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
///     nor::ConfigurationBlock::new(CHIP, FLEXSPI_CONFIGURATION_BLOCK)
///         .page_size(256)
///         .sector_size(4096)
///         .ip_cmd_serial_clk_freq(Some(CHIP.ip_serial_clock_frequency(SerialClockOption::MHz30)));
/// ```
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ConfigurationBlock {
    mem_cfg: flexspi::ConfigurationBlock,
    page_size: u32,
    sector_size: u32,
    ip_cmd_serial_clk_freq: Option<IpSerialClockFrequency>,
    extras: Extras,
}

/// Extra configurations available on some MCUs.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Extras {
    is_uniform_block_size: u8,
    is_data_order_swapped: u8,
    _reserved0: [u8; 5],
    block_size: u32,
    flash_state_ctx: u32,
    _reserved1: [u8; 40],
}

const _: () = assert!(55 == core::mem::size_of::<Extras>());

const fn extras(imxrt: Imxrt) -> Extras {
    use Imxrt::*;
    Extras {
        // By default, signal that block size equals sector size.
        // Only do this on supported chips.
        is_uniform_block_size: matches!(imxrt, Imxrt1160 | Imxrt1170 | Imxrt1180) as u8,
        is_data_order_swapped: 0u8,
        _reserved0: [0u8; 5],
        block_size: 0u32,
        flash_state_ctx: 0u32,
        _reserved1: [0u8; 40],
    }
}

impl ConfigurationBlock {
    /// Create a new serial NOR configuration block based on the FlexSPI configuration
    /// block
    pub const fn new(imxrt: Imxrt, mut mem_cfg: flexspi::ConfigurationBlock) -> Self {
        mem_cfg.device_type = 1;
        ConfigurationBlock {
            mem_cfg,
            page_size: 0,
            sector_size: 0,
            ip_cmd_serial_clk_freq: None,
            extras: extras(imxrt),
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
    /// Set the serial clock frequency for IP access.
    ///
    /// Use `None` to indicate no change from the default frequency.
    pub const fn ip_cmd_serial_clk_freq(
        mut self,
        serial_clock_frequency: Option<IpSerialClockFrequency>,
    ) -> Self {
        self.ip_cmd_serial_clk_freq = serial_clock_frequency;
        self
    }

    /// Return extra configurations that are available on the "bigger" parts.
    pub const fn extras(&mut self, imxrt: Imxrt) -> Option<&mut Extras> {
        match imxrt {
            Imxrt::Imxrt1160 | Imxrt::Imxrt1170 | Imxrt::Imxrt1180 => Some(&mut self.extras),
            _ => None,
        }
    }
}

impl Extras {
    /// Set the serial NOR block size if it differs from the sector size.
    ///
    /// By default, the configuration block signals to the hardware that the
    /// sector size is the same as the block size. Calling this will override
    /// that setting, allowing you to configure a different block size.
    ///
    /// The behavior is unspecified if you call this with a block size that's
    /// equal to the sector size.
    pub const fn block_size(&mut self, block_size: u32) -> &mut Self {
        self.is_uniform_block_size = 0u8;
        self.block_size = block_size;
        self
    }
}

const _STATIC_ASSERT_SIZE: [u32; 1] =
    [0; (core::mem::size_of::<ConfigurationBlock>() == 512) as usize];
