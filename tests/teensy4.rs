//! Serial NOR configuration block for the Teensy 4

use imxrt_boot_gen::flexspi::{opcodes::sdr::*, *};
use imxrt_boot_gen::serial_flash::*;

/// Instructions for the Winbond W25Q16JV
/// SPI flash memory controller
mod winbond {
    pub const FAST_READ_QUAD_IO: u8 = 0xEB;
    pub const READ_STATUS_REGISTER_1: u8 = 0x05;
    pub const WRITE_ENABLE: u8 = 0x06;
    pub const SECTOR_ERASE: u8 = 0x20;
    pub const PAGE_PROGRAM: u8 = 0x02;
    pub const CHIP_ERASE: u8 = 0x60;
}

use winbond::*;

//
// Sequences for lookup table
//

const SEQ_READ: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, FAST_READ_QUAD_IO))
    .instr(Instr::new(RADDR, Pads::Four, 0x18))
    .instr(Instr::new(DUMMY, Pads::Four, 0x06))
    .instr(Instr::new(READ, Pads::Four, 0x04))
    .build();

const SEQ_READ_STATUS: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, READ_STATUS_REGISTER_1))
    .instr(Instr::new(READ, Pads::One, 0x04))
    .build();

const SEQ_WRITE_ENABLE: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, WRITE_ENABLE))
    .build();

const SEQ_ERASE_SECTOR: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, SECTOR_ERASE))
    .instr(Instr::new(RADDR, Pads::One, 0x18))
    .build();

const SEQ_PAGE_PROGRAM: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, PAGE_PROGRAM))
    .instr(Instr::new(RADDR, Pads::One, 0x18))
    .instr(Instr::new(WRITE, Pads::One, 0x04))
    .build();

const SEQ_CHIP_ERASE: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, CHIP_ERASE))
    .build();

//
// Lookup table
//

const LUT: LookupTable = LookupTable::new()
    .command(Command::Read, SEQ_READ)
    .command(Command::ReadStatus, SEQ_READ_STATUS)
    .command(Command::WriteEnable, SEQ_WRITE_ENABLE)
    .command(Command::EraseSector, SEQ_ERASE_SECTOR)
    .command(Command::PageProgram, SEQ_PAGE_PROGRAM)
    .command(Command::ChipErase, SEQ_CHIP_ERASE);

//
// Common FlexSPI configuration block
//

const FLEXSPI_CONFIGURATION_BLOCK: FlexSPIConfigurationBlock = FlexSPIConfigurationBlock::new(LUT)
    .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
    .cs_hold_time(0x01)
    .cs_setup_time(0x02)
    .column_address_width(ColumnAddressWidth::OtherDevices)
    .device_mode_configuration(DeviceModeConfiguration::Disabled)
    .wait_time_cfg_commands(WaitTimeConfigurationCommands::disable())
    .flash_size(SerialFlashRegion::A1, 0x0020_0000)
    .serial_clk_freq(SerialClockFrequency::MHz60)
    .serial_flash_pad_type(FlashPadType::Quad);

//
// Final serial NOR configuration block
//
// This is what you want to place in the i.MX RT boot section
//

const SERIAL_NOR_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
    nor::ConfigurationBlock::new(FLEXSPI_CONFIGURATION_BLOCK)
        .page_size(256)
        .sector_size(4096)
        .ip_cmd_serial_clk_freq(nor::SerialClockFrequency::MHz30);

#[test]
fn teensy4() {
    let actual: &[u32; 128] = unsafe { core::mem::transmute(&SERIAL_NOR_CONFIGURATION_BLOCK) };
    const CHUNK_TEST_SIZE: usize = 16;
    let mut count = 0;
    for (idx, (actual_chunk, expected_chunk)) in actual
        .chunks(CHUNK_TEST_SIZE)
        .zip(EXPECTED.chunks(CHUNK_TEST_SIZE))
        .enumerate()
    {
        assert_eq!(
            actual_chunk,
            expected_chunk,
            "Start index {}",
            idx * CHUNK_TEST_SIZE
        );
        count += 1;
    }
    assert_eq!(count, 128 / 16);
}

// A known, working FCB for the Teensy 4.
const EXPECTED: [u32; 128] = [
    // 448 byte common FlexSPI configuration block, 8.6.3.1 page 223 (RT1062 rev 0)
    // MCU_Flashloader_Reference_Manual.pdf, 8.2.1, Table 8-2, page 72-75
    0x4246_4346, // Tag				0x00
    0x5601_0000, // Version
    0,           // reserved
    0x0002_0101, // columnAdressWidth,dataSetupTime,dataHoldTime,readSampleClkSrc
    0x0000_0000, // waitTimeCfgCommands,-,deviceModeCfgEnable
    0,           // deviceModeSeq
    0,           // deviceModeArg
    0x0000_0000, // -,-,-,configCmdEnable
    0,           // configCmdSeqs		0x20
    0,
    0,
    0,
    0, // cfgCmdArgs			0x30
    0,
    0,
    0,
    0x0000_0000, // controllerMiscOption		0x40
    0x0003_0401, // lutCustomSeqEnable,serialClkFreq,sflashPadType,deviceType
    0,           // reserved
    0,           // reserved
    0x0020_0000, // sflashA1Size			0x50
    0,           // sflashA2Size
    0,           // sflashB1Size
    0,           // sflashB2Size
    0,           // csPadSettingOverride		0x60
    0,           // sclkPadSettingOverride
    0,           // dataPadSettingOverride
    0,           // dqsPadSettingOverride
    0,           // timeoutInMs			0x70
    0,           // commandInterval
    0,           // dataValidTime
    0x0000_0000, // busyBitPolarity,busyOffset
    0x0A18_04EB, // lookupTable[0]		0x80
    0x2604_3206, // lookupTable[1]
    0,           // lookupTable[2]
    0,           // lookupTable[3]
    0x2404_0405, // lookupTable[4]		0x90
    0,           // lookupTable[5]
    0,           // lookupTable[6]
    0,           // lookupTable[7]
    0,           // lookupTable[8]		0xA0
    0,           // lookupTable[9]
    0,           // lookupTable[10]
    0,           // lookupTable[11]
    0x0000_0406, // lookupTable[12]		0xB0
    0,           // lookupTable[13]
    0,           // lookupTable[14]
    0,           // lookupTable[15]
    0,           // lookupTable[16]		0xC0
    0,           // lookupTable[17]
    0,           // lookupTable[18]
    0,           // lookupTable[19]
    0x0818_0420, // lookupTable[20]		0xD0
    0,           // lookupTable[21]
    0,           // lookupTable[22]
    0,           // lookupTable[23]
    0,           // lookupTable[24]		0xE0
    0,           // lookupTable[25]
    0,           // lookupTable[26]
    0,           // lookupTable[27]
    0,           // lookupTable[28]		0xF0
    0,           // lookupTable[29]
    0,           // lookupTable[30]
    0,           // lookupTable[31]
    0, // 0x0818_04D8, // lookupTable[32]		0x100 // This is a reserved index in the lookup table
    0, // lookupTable[33]
    0, // lookupTable[34]
    0, // lookupTable[35]
    0x0818_0402, // lookupTable[36]		0x110
    0x0000_2004, // lookupTable[37]
    0, // lookupTable[38]
    0, // lookupTable[39]
    0, // lookupTable[40]		0x120
    0, // lookupTable[41]
    0, // lookupTable[42]
    0, // lookupTable[43]
    0x0000_0460, // lookupTable[44]		0x130
    0, // lookupTable[45]
    0, // lookupTable[46]
    0, // lookupTable[47]
    0, // lookupTable[48]		0x140
    0, // lookupTable[49]
    0, // lookupTable[50]
    0, // lookupTable[51]
    0, // lookupTable[52]		0x150
    0, // lookupTable[53]
    0, // lookupTable[54]
    0, // lookupTable[55]
    0, // lookupTable[56]		0x160
    0, // lookupTable[57]
    0, // lookupTable[58]
    0, // lookupTable[59]
    0, // lookupTable[60]		0x170
    0, // lookupTable[61]
    0, // lookupTable[62]
    0, // lookupTable[63]
    0, // LUT 0: Read			0x180
    0, // LUT 1: ReadStatus
    0, // LUT 3: WriteEnable
    0, // LUT 5: EraseSector
    0, // LUT 9: PageProgram		0x190
    0, // LUT 11: ChipErase
    0, // LUT 15: Dummy
    0, // LUT unused?
    0, // LUT unused?			0x1A0
    0, // LUT unused?
    0, // LUT unused?
    0, // LUT unused?
    0, // reserved			0x1B0
    0, // reserved
    0, // reserved
    0, // reserved
    // 64 byte Serial NOR configuration block, 8.6.3.2, page 346
    256,  // pageSize			0x1C0
    4096, // sectorSize
    1,    // ipCmdSerialClkFreq
    0,    // reserved
    0,    // reserved			0x1D0
    0,    // reserved
    0,    // reserved
    0,    // reserved
    0,    // reserved			0x1E0
    0,    // reserved
    0,    // reserved
    0,    // reserved
    0,    // reserved			0x1F0
    0,    // reserved
    0,    // reserved
    0,    // reserved
];
