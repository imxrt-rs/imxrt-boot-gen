//! FlexSPI configuration block (FCB) for the iMXRT1170EVK.
//!
//! This FCB is compatible with the IS25WB QuadSPI flash storage found on the
//! iMXRT1170EVK.
#![no_std]

pub use nor::ConfigurationBlock;

use imxrt_boot_gen::flexspi::{self, opcodes::sdr::*, *};
use imxrt_boot_gen::flexspi::{
    FlashPadType, ReadSampleClockSource, SerialClockFrequency, SerialFlashRegion,
};
use imxrt_boot_gen::serial_flash::*;

const SEQ_READ: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, 0xEB))
    .instr(Instr::new(RADDR, Pads::Four, 0x18))
    .instr(Instr::new(DUMMY, Pads::Four, 0x06))
    .instr(Instr::new(READ, Pads::Four, 0x04))
    .build();
const SEQ_READ_STATUS: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, 0x05))
    .instr(Instr::new(READ, Pads::One, 0x04))
    .build();
const SEQ_WRITE_ENABLE: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, 0x06))
    .build();
const SEQ_ERASE_SECTOR: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, 0x20))
    .instr(Instr::new(RADDR, Pads::One, 0x18))
    .build();
const SEQ_PAGE_PROGRAM: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, 0x02))
    .instr(Instr::new(RADDR, Pads::One, 0x18))
    .instr(Instr::new(WRITE, Pads::One, 0x04))
    .build();
const SEQ_CHIP_ERASE: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, 0x60))
    .build();

const LUT: LookupTable = LookupTable::new()
    .command(Command::Read, SEQ_READ)
    .command(Command::ReadStatus, SEQ_READ_STATUS)
    .command(Command::WriteEnable, SEQ_WRITE_ENABLE)
    .command(Command::EraseSector, SEQ_ERASE_SECTOR)
    .command(Command::PageProgram, SEQ_PAGE_PROGRAM)
    .command(Command::ChipErase, SEQ_CHIP_ERASE);

const COMMON_CONFIGURATION_BLOCK: flexspi::ConfigurationBlock =
    flexspi::ConfigurationBlock::new(LUT)
        .version(Version::new(1, 4, 0))
        .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
        .cs_hold_time(3)
        .cs_setup_time(3)
        .controller_misc_options(0x10)
        .serial_flash_pad_type(FlashPadType::Quad)
        .serial_clk_freq(SerialClockFrequency::MHz133)
        .flash_size(SerialFlashRegion::A1, 16 * 1024 * 1024);

pub const SERIAL_NOR_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
    nor::ConfigurationBlock::new(COMMON_CONFIGURATION_BLOCK)
        .page_size(256)
        .sector_size(4 * 1024)
        .ip_cmd_serial_clk_freq(nor::SerialClockFrequency::MHz30)
        .block_size(64 * 1024);

#[no_mangle]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), link_section = ".fcb")]
pub static FLEXSPI_CONFIGURATION_BLOCK: nor::ConfigurationBlock = SERIAL_NOR_CONFIGURATION_BLOCK;

#[cfg(test)]
mod tests {
    use super::SERIAL_NOR_CONFIGURATION_BLOCK;

    /// Magic numbers extracted from a build of the 1170 EVK's SDK.
    ///
    /// The actual configuration has an instruction sequence, 'erase block,'
    /// at offset 0x100. I I dropped it when coping over the raw values, since
    /// we don't have support for custom instruction sequences. Erase sector and
    /// erase chip are both implemented.
    const EXPECTED: [u32; 128] = [
        0x46434642, 0x00040156, 0x00000000, 0x01030300, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x10000000, 0x01040700, 0x00000000, 0x00000000, 0x00000001,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xeb04180a, 0x06320426, 0x00000000,
        0x00000000, 0x05040424, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x06040000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x20041808, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x02041808, 0x04200000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x60040000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00010000, 0x00100000, 0x01000000, 0x00000000, 0x00000100, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000,
    ];

    #[test]
    fn imxrt1170evk() {
        let actual: [u32; 128] = unsafe { core::mem::transmute(SERIAL_NOR_CONFIGURATION_BLOCK) };
        for (i, (a, e)) in actual.iter().zip(EXPECTED).enumerate() {
            let offset = i * 4;
            assert_eq!(
                a.to_be_bytes(),
                e.to_le_bytes(),
                "Offset {offset:#X}\nACTUAL: {actual:?}\nEXPECTED: {EXPECTED:?}"
            );
        }
    }
}
