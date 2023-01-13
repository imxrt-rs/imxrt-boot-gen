#![cfg(feature = "imxrt1020")]

use imxrt_boot_gen::flexspi::{self, opcodes::sdr::*, *};
use imxrt_boot_gen::serial_flash::*;

/// Instructions for the ISSI IS25LP064A SPI flash memory controller
mod issi {
    pub const FAST_READ_QUAD_IO: u8 = 0xEB;
}

use issi::*;

//
// Sequences for lookup table
//

const SEQ_READ: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, FAST_READ_QUAD_IO))
    .instr(Instr::new(RADDR, Pads::Four, 0x18))
    .instr(Instr::new(DUMMY, Pads::Four, 0x06))
    .instr(Instr::new(READ, Pads::Four, 0x04))
    .build();

//
// Lookup table
//

const LUT: LookupTable = LookupTable::new().command(Command::Read, SEQ_READ);

//
// Common FlexSPI configuration block
//

const FLEXSPI_CONFIGURATION_BLOCK: flexspi::ConfigurationBlock =
    flexspi::ConfigurationBlock::new(LUT)
        .version(Version::new(1, 4, 0))
        .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
        .cs_hold_time(0x03)
        .cs_setup_time(0x03)
        .column_address_width(ColumnAddressWidth::OtherDevices)
        .device_mode_configuration(DeviceModeConfiguration::Disabled)
        .wait_time_cfg_commands(WaitTimeConfigurationCommands::disable())
        .flash_size(SerialFlashRegion::A1, 0x0080_0000)
        .serial_clk_freq(SerialClockFrequency::MHz100)
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
        .ip_cmd_serial_clk_freq(nor::SerialClockFrequency::NoChange);

#[test]
fn imxrt1020_evk() {
    let actual: [u32; 128] = unsafe { core::mem::transmute(SERIAL_NOR_CONFIGURATION_BLOCK) };
    for (i, (a, e)) in actual.into_iter().zip(EXPECTED).enumerate() {
        let offset = i * 4;
        assert_eq!(
            a, e,
            "Offset {offset:#X}\nACTUAL: {actual:?}\nEXPECTED: {EXPECTED:?}"
        );
    }
}

// A known, working FCB for the MIMXRT1020-EVK.
#[rustfmt::skip]
const EXPECTED: [u32; 128] = [
    /* 0x000 */ 0x4246_4346, // Tag
    /*       */ 0x5601_0400, // Version
    /*       */ 0,          // reserved
    /*       */ 0x0003_0301, // columnAdressWidth,dataSetupTime,dataHoldTime,readSampleClkSrc
    /* 0x010 */ 0,           // waitTimeCfgCommands,-,deviceModeCfgEnable
    /*       */ 0,           // deviceModeSeq
    /*       */ 0,           // deviceModeArg
    /*       */ 0,           // -,-,-,configCmdEnable
    /* 0x020 */ 0,           // configCmdSeqs
    /*       */ 0,
    /*       */ 0,
    /*       */ 0,           // reserved
    /* 0x030 */ 0,           // cfgCmdArgs
    /*       */ 0,
    /*       */ 0,
    /*       */ 0,           // reserved
    /* 0x040 */ 0,           // controllerMiscOption
    /*       */ 0x0006_0401, // lutCustomSeqEnable,serialClkFreq,sflashPadType,deviceType
    /*       */              // (XXX orig. 0x0006_0400, i.e. deviceType is 0, which is undefined)
    /*       */ 0,           // reserved
    /*       */ 0,           // reserved
    /* 0x050 */ 0x0080_0000, // sflashA1Size
    /*       */ 0,           // sflashA2Size
    /*       */ 0,           // sflashB1Size
    /*       */ 0,           // sflashB2Size
    /* 0x060 */ 0,           // csPadSettingOverride
    /*       */ 0,           // sclkPadSettingOverride
    /*       */ 0,           // dataPadSettingOverride
    /*       */ 0,           // dqsPadSettingOverride
    /* 0x070 */ 0,           // timeoutInMs
    /*       */ 0,           // commandInterval
    /*       */ 0,           // dataValidTime
    /*       */ 0,           // busyBitPolarity,busyOffset
    /* 0x080 */ 0x0a18_04eb, // lookupTable[0]
    /*       */ 0x2604_3206, // lookupTable[1]
    /*       */ 0,           // lookupTable[2]
    /*       */ 0,           // lookupTable[3]
    /* 0x090 */ 0,           // lookupTable[4]
    /*       */ 0,           // lookupTable[5]
    /*       */ 0,           // lookupTable[6]
    /*       */ 0,           // lookupTable[7]
    /* 0x0a0 */ 0,           // lookupTable[8]
    /*       */ 0,           // lookupTable[9]
    /*       */ 0,           // lookupTable[10]
    /*       */ 0,           // lookupTable[11]
    /* 0x0b0 */ 0,           // lookupTable[12]
    /*       */ 0,           // lookupTable[13]
    /*       */ 0,           // lookupTable[14]
    /*       */ 0,           // lookupTable[15]
    /* 0x0c0 */ 0,           // lookupTable[16]
    /*       */ 0,           // lookupTable[17]
    /*       */ 0,           // lookupTable[18]
    /*       */ 0,           // lookupTable[19]
    /* 0x0d0 */ 0,           // lookupTable[20]
    /*       */ 0,           // lookupTable[21]
    /*       */ 0,           // lookupTable[22]
    /*       */ 0,           // lookupTable[23]
    /* 0x0e0 */ 0,           // lookupTable[24]
    /*       */ 0,           // lookupTable[25]
    /*       */ 0,           // lookupTable[26]
    /*       */ 0,           // lookupTable[27]
    /* 0x0f0 */ 0,           // lookupTable[28]
    /*       */ 0,           // lookupTable[29]
    /*       */ 0,           // lookupTable[30]
    /*       */ 0,           // lookupTable[31]
    /* 0x100 */ 0,           // lookupTable[32]
    /*       */ 0,           // lookupTable[33]
    /*       */ 0,           // lookupTable[34]
    /*       */ 0,           // lookupTable[35]
    /* 0x110 */ 0,           // lookupTable[36]
    /*       */ 0,           // lookupTable[37]
    /*       */ 0,           // lookupTable[38]
    /*       */ 0,           // lookupTable[39]
    /* 0x120 */ 0,           // lookupTable[40]
    /*       */ 0,           // lookupTable[41]
    /*       */ 0,           // lookupTable[42]
    /*       */ 0,           // lookupTable[43]
    /* 0x130 */ 0,           // lookupTable[44]
    /*       */ 0,           // lookupTable[45]
    /*       */ 0,           // lookupTable[46]
    /*       */ 0,           // lookupTable[47]
    /* 0x140 */ 0,           // lookupTable[48]
    /*       */ 0,           // lookupTable[49]
    /*       */ 0,           // lookupTable[50]
    /*       */ 0,           // lookupTable[51]
    /* 0x150 */ 0,           // lookupTable[52]
    /*       */ 0,           // lookupTable[53]
    /*       */ 0,           // lookupTable[54]
    /*       */ 0,           // lookupTable[55]
    /* 0x160 */ 0,           // lookupTable[56]
    /*       */ 0,           // lookupTable[57]
    /*       */ 0,           // lookupTable[58]
    /*       */ 0,           // lookupTable[59]
    /* 0x170 */ 0,           // lookupTable[60]
    /*       */ 0,           // lookupTable[61]
    /*       */ 0,           // lookupTable[62]
    /*       */ 0,           // lookupTable[63]
    /* 0x180 */ 0,           // lutCustomSeq
    /*       */ 0,
    /*       */ 0,
    /*       */ 0,
    /* 0x190 */ 0,
    /*       */ 0,
    /*       */ 0,
    /*       */ 0,
    /* 0x1a0 */ 0,
    /*       */ 0,
    /*       */ 0,
    /*       */ 0,
    /* 0x1b0 */ 0,           // reserved
    /*       */ 0,           // reserved
    /*       */ 0,           // reserved
    /*       */ 0,           // reserved

    // 64 byte Serial NOR configuration block, 9.6.3.2, page 211
    /* 0x1c0 */ 256,         // pageSize
    /*       */ 4096,        // sectorSize
    /*       */ 0,           // ipCmdSerialClkFreq
    /*       */ 0,           // reserved
    /* 0x1d0 */ 0,           // reserved (XXX orig. 0x00040000, blockSize according to the SDK)
    /*       */ 0,           // reserved
    /*       */ 0,           // reserved
    /*       */ 0,           // reserved
    /* 0x1e0 */ 0,           // reserved
    /*       */ 0,           // reserved
    /*       */ 0,           // reserved
    /*       */ 0,           // reserved
    /* 0x1f0 */ 0,           // reserved
    /*       */ 0,           // reserved
    /*       */ 0,           // reserved
    /*       */ 0,           // reserved
];
