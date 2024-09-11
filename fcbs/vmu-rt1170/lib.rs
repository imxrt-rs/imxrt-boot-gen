//! FlexSPI configuration block (FCB) for the iMXRT1170EVK.
//!
//! This FCB is compatible with thei Macronix MX25UM51345GXDI00 Hyperflash (Octal SPI) storage found on the
//! VMU RT1170. However, it only supports reads.
#![no_std]

pub use nor::ConfigurationBlock;

use imxrt_boot_gen::flexspi::{self, opcodes::sdr::*, *};
use imxrt_boot_gen::serial_flash::*;

const BOOT_SEQ_READ: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, 0x03))
    .instr(Instr::new(RADDR, Pads::One, 0x18))
    .instr(Instr::new(READ, Pads::One, 0x04))
    .instr(STOP)
    .build();

const BOOT_LUT: LookupTable = LookupTable::new().command(Command::Read, BOOT_SEQ_READ);

const BOOT_CONFIGURATION_BLOCK: flexspi::ConfigurationBlock =
    flexspi::ConfigurationBlock::new(BOOT_LUT)
        .version(Version::new(1, 4, 0))
        .read_sample_clk_src(ReadSampleClockSource::InternalLoopback)
        .cs_hold_time(1)
        .cs_setup_time(1)
        .controller_misc_options(0x10)
        .serial_flash_pad_type(FlashPadType::Single)
        .serial_clk_freq(SerialClockFrequency::MHz80)
        .flash_size(SerialFlashRegion::A1, 64 * 1024 * 1024);

/// Boot FlexSPI NOR Configuration for the VMU only needs read commands
/// over single wire SPI at a slower clock.
///
/// After booting the VMU can reconfigure FlexSPI to be clocked faster
/// using OctalSPI as needed with the full LUT of command sequences.
pub const BOOT_SERIAL_NOR_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
    nor::ConfigurationBlock::new(BOOT_CONFIGURATION_BLOCK)
        .page_size(256)
        .sector_size(4 * 1024)
        .ip_cmd_serial_clk_freq(nor::SerialClockFrequency::MHz30)
        .block_size(64 * 1024);

#[no_mangle]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), link_section = ".fcb")]
pub static FLEXSPI_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
    BOOT_SERIAL_NOR_CONFIGURATION_BLOCK;
