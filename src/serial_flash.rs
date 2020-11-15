//! Serial NOR flash boot
//!
//! `serial_flash` provides the types necessary to boot an i.MX RT processor
//! from serial NOR flash. *Note: NAND Flash boot not yet implemented.*
//!
//! The API includes
//!
//! - Flexible Serial Peripheral Interface (FlexSPI) sequence and lookup table (LUT)
//! - FlexSPI configuration block ([`FlexSPIConfigurationBlock`])
//! - serial NOR configuration block ([`nor::ConfigurationBlock`]), which should be
//!   properly placed in memory
//!
//! # Sequences and LUTs
//!
//! A [`Sequence`] is a collection of up to eight FlexSPI instructions ([`Instr`]).
//! The FlexSPI controller sequentially executes instructions to perform reads, writes
//! and I/O with a connected FLASH device. The FlexSPI controller finds each sequence
//! in a [`LookupTable`].
//!
//! Use a [`SequenceBuilder`] to create `Sequence`s:
//!
//! ```
//! use imxrt_boot_gen::serial_flash::{Instr, Sequence, SequenceBuilder, Pads, opcodes::sdr::*};
//!
//! # const FAST_READ_QUAD_IO: u8 = 0;
//! # const READ_STATUS_REGISTER_1: u8 = 0;
//! const SEQ_READ: Sequence = SequenceBuilder::new()
//!     .instr(Instr::new(CMD, Pads::One, FAST_READ_QUAD_IO))
//!     .instr(Instr::new(RADDR, Pads::Four, 0x18))
//!     .instr(Instr::new(DUMMY, Pads::Four, 0x06))
//!     .instr(Instr::new(READ, Pads::Four, 0x04))
//!     .build();
//!
//! const SEQ_READ_STATUS: Sequence = SequenceBuilder::new()
//!     .instr(Instr::new(CMD, Pads::One, READ_STATUS_REGISTER_1))
//!     .instr(Instr::new(READ, Pads::One, 0x04))
//!     .build();
//! ```
//!
//! Then, assign each sequence to a [`Command`] in a `LookupTable`:
//!
//! ```
//! use imxrt_boot_gen::serial_flash::{Command, LookupTable};
//! # use imxrt_boot_gen::serial_flash::{Sequence, SequenceBuilder};
//!
//! # const SEQ_READ: Sequence = SequenceBuilder::new().build();
//! # const SEQ_READ_STATUS: Sequence = SequenceBuilder::new().build();
//! const LUT: LookupTable = LookupTable::new()
//!     .command(Command::Read, SEQ_READ)
//!     .command(Command::ReadStatus, SEQ_READ_STATUS);
//! ```
//!
//! # FlexSPI Configuration Block
//!
//! Once you've created your sequences and lookup table, use the lookup table to create
//! a [`FlexSPIConfigurationBlock`]. See the `FlexSPIConfigurationBlock` documentation
//! for more information.
//!
//! # Serial NOR Configuration Block
//!
//! Finally, use the FlexSPI configuration block to create a Serial NOR configuration
//! block. You are responsible for placing the serial NOR configuration block at the correct
//! location in memory. See [`nor::ConfigurationBlock`] for an example.

mod fields;
mod lookup;
pub mod nor;

pub use fields::*;
pub use lookup::*;

/// ASCII 'FCFB'
const TAG: u32 = 0x4246_4346;
/// [07:00] bugfix = 0
/// [15:08] minor
/// [23:16] major = 1
/// [31:24] ascii ‘V’
const VERSION: u32 = 0x5601_0000;

/// The recommended `csHoldTime`, `0x03`.
///
/// This is the default value if not set with [`FlexSPIConfigurationBlock::cs_hold_time`].
pub const RECOMMENDED_CS_HOLD_TIME: u8 = 0x03;
/// The recommended `csSetupTime`, `0x03`.
///
/// This is the default value if not set with [`FlexSPIConfigurationBlock::cs_setup_time`].
pub const RECOMMENDED_CS_SETUP_TIME: u8 = 0x03;

/// FlexSPI configuration block
///
/// The FlexSPI configuration block consists of parameters that are for specific flash
/// devices. The configuration block includes the FlexSPI [`LookupTable`]. The configuration
/// block is shared between serial NOR and NAND configuration blocks.
///
/// # Default Values
///
/// - `cs_hold_time` is [`RECOMMENDED_CS_HOLD_TIME`]
/// - `cs_setup_time` is [`RECOMMENDED_CS_SETUP_TIME`]
///
/// All other configurable values are set to a bit pattern of 0.
///
/// # Examples
///
/// ```
/// use imxrt_boot_gen::serial_flash::*;
///
/// # const LUT: LookupTable = LookupTable::new();
/// const FLEXSPI_CONFIGURATION_BLOCK: FlexSPIConfigurationBlock =
///     FlexSPIConfigurationBlock::new(LUT)
///         .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
///         .cs_hold_time(0x01)
///         .cs_setup_time(0x02)
///         .column_address_width(ColumnAddressWidth::OtherDevices)
///         .device_mode_configuration(DeviceModeConfiguration::Disabled)
///         .wait_time_cfg_commands(WaitTimeConfigurationCommands::new(40_000))
///         .flash_size(SerialFlashRegion::A1, 0x0020_0000)
///         .serial_clk_freq(SerialClockFrequency::MHz60)
///         .serial_flash_pad_type(FlashPadType::Quad);
///
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct FlexSPIConfigurationBlock {
    tag: u32,
    version: u32,
    _reserved0: [u8; 4], // 0x008
    read_sample_clk_src: u8,
    cs_hold_time: u8,
    cs_setup_time: u8,
    column_address_width: u8,
    device_mode_configuration: u8,
    _reserved1: [u8; 1], // 0x011
    wait_time_cfg_commands: u16,
    device_mode_sequence: [u8; 4],
    device_mode_arg: u32,
    config_cmd_enable: u8,
    _reserved2: [u8; 3], // 0x01D
    config_cmd_seqs: [u8; 12],
    _reserved3: [u8; 4], // 0x02C
    cfg_cmd_args: [u8; 12],
    _reserved4: [u8; 4], // 0x03C
    controller_misc_options: u32,
    device_type: u8,
    serial_flash_pad_type: u8,
    serial_clk_freq: u8,
    lut_custom_seq_enable: u8,
    _reserved5: [u8; 8], // 0x048
    /// A1, A2, B1, B2
    serial_flash_sizes: [u32; 4],
    cs_pad_setting_override: u32,
    sclk_pad_setting_override: u32,
    data_pad_setting_override: u32,
    dqs_pad_setting_override: u32,
    timeout_ms: u32,
    command_interval: u32,
    data_valid_time: u32,
    busy_offset: u16,
    busy_bit_polarity: u16,
    lookup_table: LookupTable,
    lut_custom_seq: [u8; 48],
    _reserved6: [u8; 16],
}

impl FlexSPIConfigurationBlock {
    /// Create a new configuration block that uses `lookup_table` as the
    /// FlexSPI LUT
    pub const fn new(lookup_table: LookupTable) -> Self {
        FlexSPIConfigurationBlock {
            tag: TAG,
            version: VERSION,
            read_sample_clk_src: ReadSampleClockSource::InternalLoopback as u8,
            cs_hold_time: RECOMMENDED_CS_HOLD_TIME,
            cs_setup_time: RECOMMENDED_CS_SETUP_TIME,
            column_address_width: ColumnAddressWidth::OtherDevices as u8,
            device_mode_configuration: 0, // Disabled
            wait_time_cfg_commands: 0,
            device_mode_sequence: [0; 4],
            device_mode_arg: 0,
            config_cmd_enable: 0,
            config_cmd_seqs: [0; 12],
            cfg_cmd_args: [0; 12],
            controller_misc_options: 0,
            device_type: 0, // Invalid value; must be updated in NOR / NAND configuration block
            serial_flash_pad_type: 1, // Single pad
            serial_clk_freq: 0, // 30MHz
            lut_custom_seq_enable: 0,
            serial_flash_sizes: [0; 4],
            cs_pad_setting_override: 0,
            sclk_pad_setting_override: 0,
            data_pad_setting_override: 0,
            dqs_pad_setting_override: 0,
            timeout_ms: 0,
            command_interval: 0,
            data_valid_time: 0,
            busy_offset: 0,
            busy_bit_polarity: 0,
            lookup_table: lookup_table,
            lut_custom_seq: [0; 48],

            _reserved0: [0; 4],
            _reserved1: [0; 1],
            _reserved2: [0; 3],
            _reserved3: [0; 4],
            _reserved4: [0; 4],
            _reserved5: [0; 8],
            _reserved6: [0; 16],
        }
    }

    /// `readSampleClkSrc`, the clock source for FlexSPI
    ///
    /// If not set, this defaults to `ReadSampleClockSource::InternalLoopback`.
    pub const fn read_sample_clk_src(mut self, read_sample_clk_src: ReadSampleClockSource) -> Self {
        self.read_sample_clk_src = read_sample_clk_src as u8;
        self
    }

    /// Set the chip select hold time (`csHoldTime`)
    ///
    /// If not set, this will be `RECOMMENDED_CS_HOLD_TIME`, which is `0x03`.
    pub const fn cs_hold_time(mut self, cs_hold_time: u8) -> Self {
        self.cs_hold_time = cs_hold_time;
        self
    }

    /// Set the chip select setup time (`csSetupTime`)
    ///
    /// If not set, this will be `RECOMMENDED_CS_SETUP_TIME`, which is `0x03`.
    pub const fn cs_setup_time(mut self, cs_setup_time: u8) -> Self {
        self.cs_setup_time = cs_setup_time;
        self
    }

    /// `columnAddressWidth`, the properties of the flash memory
    ///
    /// If not set, this defaults to `ColumnAddressWidth::OtherDevices`
    pub const fn column_address_width(mut self, column_address_width: ColumnAddressWidth) -> Self {
        self.column_address_width = column_address_width as u8;
        self
    }

    /// Sets device configuration mode. The `DeviceModeConfiguration::Disabled` variant
    /// will set `deviceModeCfgEnable` to "disabled". Otherwise, we will set
    /// `deviceModeCfgEnable` to "enabled," and we use the sequence and argument
    /// parameters in the FCB.
    ///
    /// If not set, this defaults to `DeviceModeConfiguration::Disabled`.
    pub const fn device_mode_configuration(
        mut self,
        device_mode_configuration: DeviceModeConfiguration,
    ) -> Self {
        match device_mode_configuration {
            DeviceModeConfiguration::Disabled => {
                self.device_mode_configuration = 0;
            }
            DeviceModeConfiguration::Enabled {
                device_mode_seq,
                device_mode_arg,
            } => {
                self.device_mode_configuration = 1;
                self.device_mode_sequence = device_mode_seq.0;
                self.device_mode_arg = device_mode_arg;
            }
        }
        self
    }

    /// Sets `waitTimeCfgCommands`
    ///
    /// If not set, this defaults to `WaitTimeConfigurationCommands::disable()`.
    pub const fn wait_time_cfg_commands(
        mut self,
        wait_time_cfg_commands: WaitTimeConfigurationCommands,
    ) -> Self {
        self.wait_time_cfg_commands = wait_time_cfg_commands.0;
        self
    }

    /// Sets the serial flash pad type, `sFlashPad`.
    ///
    /// If not set, this defaults to `FlashPadType::Single`.
    pub const fn serial_flash_pad_type(mut self, serial_flash_pad_type: FlashPadType) -> Self {
        self.serial_flash_pad_type = serial_flash_pad_type as u8;
        self
    }

    /// Sets the serial clock frequencey, `serialClkFreq`
    ///
    /// If not set, this defaults to `SerialClockFrequency::MHz30`.
    pub const fn serial_clk_freq(mut self, serial_clk_freq: SerialClockFrequency) -> Self {
        self.serial_clk_freq = serial_clk_freq as u8;
        self
    }

    /// Set a flash size for the provided flash region
    ///
    /// Any region that's not set will default to `0`.
    pub const fn flash_size(mut self, flash_region: SerialFlashRegion, flash_size: u32) -> Self {
        self.serial_flash_sizes[flash_region as usize] = flash_size;
        self
    }
}

const _STATIC_ASSERT_SIZE: [u32; 1] =
    [0; (core::mem::size_of::<FlexSPIConfigurationBlock>() == 448) as usize];
