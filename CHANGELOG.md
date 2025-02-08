# Changelog

This document tracks changes to the `imxrt-boot-gen` package and all other
packages maintained within this repository.

## [Unreleased]

Add support for new MCUS:

- 1040
- 1160

Expose configuration commands in the common FlexSPI configuration block.

Add support for custom sequences in the FCB LUT.

## [0.3.3] - 2024-10-26

Add support for the 1180 family

## [0.3.2] - 2024-02-01

Add support for the 1050 family.

## [0.3.1] - 2023-01-14

Add support for the 1020 family.

## [0.3.0] - 2022-11-21

**BREAKING** Update Rust edition to 2021.

Add support for the 1170 family.

Publish 0.1 FCBs for NXP's IMXRT1010EVK and IMXRT1170EVK.

## [0.2.1] - 2022-11-21

Fix the `ipCmdSerialClkFreq` field width in the serial NOR flash configuration
block.

## [0.2.0] - 2020-12-26

**BREAKING** The 0.2 release introduces a `const` API to replace the build-time
interface. We do not generate data structures in a build script, then write them
out as Rust code. Instead, we generate them at compile time, using a `const` API.
This simplifies the library while remaining type safe. The redesign lets you
specify your own symbol names and link sections for your configuration blocks.

See the updated documentation for more information on the API. The rest of this
section describes migration tips.

The 0.2 release is `no_std`. Take the `imxrt_boot_gen` symbols from your build
script, and move them into your embedded Rust code. You will need to update
your import paths to reference the re-organized modules.

### Sequences

A `SequenceBuilder` supports `Sequence` allocation. You should change your
`Sequence` definitions to use `SequenceBuilder`. The example below compares
the old `Sequence` API with the new `SequenceBuilder` API:

```rust
// Old API:
use imxrt_boot_gen::serial_flash::opcodes::sdr::*;
use imxrt_boot_gen::serial_flash::*;

const SEQ_READ: Sequence = Sequence([
    Instr::new(CMD, Pads::One, 0xEB),
    Instr::new(READ, Pads::Four, 0x04),
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
    STOP,
]);

// New API:
use imxrt_boot_gen::flexspi::{*, opcodes::sdr::*};
const SEQ_READ: Sequence = SequenceBuilder::new()
    .instr(Instr::new(CMD, Pads::One, 0xEB))
    .instr(Instr::new(READ, Pads::Four, 0x04))
    .build();
```

### LUT

Lookup tables have a `const` API that maps a command to a sequence. The example
below shows how you might update your lookup table definition.

```rust
// Old API:
let lookup_table = {
    use imxrt_boot_gen::serial_flash::CommandSequence::*;
    let mut lut = LookupTable::new();
    lut[Read] = SEQ_READ;
    lut[ReadStatus] = SEQ_READ_STATUS;
    lut[WriteEnable] = SEQ_WRITE_ENABLE;
    lut[EraseSector] = SEQ_ERASE_SECTOR;
    lut[PageProgram] = SEQ_PAGE_PROGRAM;
    lut[ChipErase] = SEQ_CHIP_ERASE;
    lut
};

// New API:
const LUT: LookupTable = LookupTable::new()
    .command(Command::Read, SEQ_READ)
    .command(Command::ReadStatus, SEQ_READ_STATUS)
    .command(Command::WriteEnable, SEQ_WRITE_ENABLE)
    .command(Command::EraseSector, SEQ_ERASE_SECTOR)
    .command(Command::PageProgram, SEQ_PAGE_PROGRAM)
    .command(Command::ChipErase, SEQ_CHIP_ERASE);
```

### FlexSPI and serial NOR configuration blocks

FlexSPI configuration blocks may be `const`. Provide the LUT as an argument
to your `flexspi::ConfigurationBlock` constructor. Then pass your FlexSPI
configuration block into the serial NOR configuration block.

Define your serial NOR configuration block in `static` memory. Place the serial
NOR configuration block in FLASH memory to boot your system.

```rust
// Old API:
let fcb = FCBBuilder::new(DeviceType::SerialNOR(nor_cb), lookup_table)
    .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
    .cs_hold_time(0x01)
    .cs_setup_time(0x02)
    .column_address_width(ColumnAddressWidth::OtherDevices)
    .device_mode_configuration(DeviceModeConfiguration::Disabled)
    .wait_time_cfg_commands(WaitTimeConfigurationCommands::disable())
    .flash_size(SerialFlashRegion::A1, 0x0020_0000)
    .serial_clk_freq(SerialClockFrequency::MHz60)
    .serial_flash_pad_type(FlashPadType::Quad)
    .build()
    .unwrap();

// New API:
use imxrt_boot_gen::flexspi::{self, *};
use imxrt_boot_gen::serial_flash::*;

const FLEXSPI_CONFIGURATION_BLOCK: flexspi::ConfigurationBlock =
    flexspi::ConfigurationBlock::new(LUT)
        .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
        .cs_hold_time(0x01)
        .cs_setup_time(0x02)
        .column_address_width(ColumnAddressWidth::OtherDevices)
        .device_mode_configuration(DeviceModeConfiguration::Disabled)
        .wait_time_cfg_commands(WaitTimeConfigurationCommands::disable())
        .flash_size(SerialFlashRegion::A1, 0x0020_0000)
        .serial_clk_freq(SerialClockFrequency::MHz60)
        .serial_flash_pad_type(FlashPadType::Quad);

#[no_mangle]
#[link_section = ".serial_nor_cb"]
static SERIAL_NOR_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
    nor::ConfigurationBlock::new(FLEXSPI_CONFIGURATION_BLOCK)
        .page_size(256)
        .sector_size(4096)
        .ip_cmd_serial_clk_freq(nor::SerialClockFrequency::MHz30);
```

## [0.1.0] - 2020-04-07

First release

[Unreleased]: https://github.com/imxrt-rs/imxrt-boot-gen/compare/v0.3.0...HEAD
[0.3.3]: https://github.com/imxrt-rs/imxrt-boot-gen/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/imxrt-rs/imxrt-boot-gen/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/imxrt-rs/imxrt-boot-gen/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/imxrt-rs/imxrt-boot-gen/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/imxrt-rs/imxrt-boot-gen/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/imxrt-rs/imxrt-boot-gen/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/imxrt-rs/imxrt-boot-gen/releases/tag/v0.1.0
