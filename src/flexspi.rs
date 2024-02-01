//! FlexSPI configuration block definitions
//!
//! The FlexSPI module includes
//!
//! - instruction sequences
//! - instruction lookup table (LUT)
//! - the FlexSPI configuration block
//!
//! The `flexspi` types are used throughout the [`serial_flash`](crate::serial_flash) API, since the FlexSPI
//! configuration block is at the start of every serial NOR / NAND configuration block.
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
//! use imxrt_boot_gen::flexspi::{Instr, Sequence, SequenceBuilder, Pads, opcodes::sdr::*};
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
//! use imxrt_boot_gen::flexspi::{Command, LookupTable};
//! # use imxrt_boot_gen::flexspi::{Sequence, SequenceBuilder};
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
//! a [`ConfigurationBlock`]. See the `ConfigurationBlock` documentation
//! for more information.

mod fields;
mod lookup;
mod sequence;

pub use fields::*;
pub use lookup::{Command, LookupTable};
pub use sequence::{opcodes, Instr, Pads, Sequence, SequenceBuilder, JUMP_ON_CS, STOP};

/// A version identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Version(u32);

impl Version {
    /// Construct a version number for your FCB.
    ///
    /// Once constructed, pass the version to the configuration block with
    /// [`ConfigurationBlock::version`](ConfigurationBlock::version).
    pub const fn new(major: u8, minor: u8, bugfix: u8) -> Version {
        Version(
            ((b'V' as u32) << 24) | ((major as u32) << 16) | ((minor as u32) << 8) | bugfix as u32,
        )
    }
}

/// ASCII 'FCFB'
const TAG: u32 = 0x4246_4346;
/// The default FCB version used by this library.
///
/// Use [`Version::new`](Version::new) to compute your own version identifier.
pub const VERSION_DEFAULT: Version = Version::new(1, 0, 0);
#[allow(clippy::assertions_on_constants)] // Sanity check.
const _: () = assert!(VERSION_DEFAULT.0 == 0x5601_0000);

/// The recommended `csHoldTime`, `0x03`.
///
/// This is the default value if not set with [`ConfigurationBlock::cs_hold_time`].
pub const RECOMMENDED_CS_HOLD_TIME: u8 = 0x03;
/// The recommended `csSetupTime`, `0x03`.
///
/// This is the default value if not set with [`ConfigurationBlock::cs_setup_time`].
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
/// use imxrt_boot_gen::flexspi::*;
///
/// # const LUT: LookupTable = LookupTable::new();
/// const FLEXSPI_CONFIGURATION_BLOCK: ConfigurationBlock =
///     ConfigurationBlock::new(LUT)
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
pub struct ConfigurationBlock {
    tag: u32,
    version: Version,
    _reserved0: [u8; 4], // 0x008
    read_sample_clk_src: ReadSampleClockSource,
    cs_hold_time: u8,
    cs_setup_time: u8,
    column_address_width: ColumnAddressWidth,
    device_mode_configuration: u8,
    /// TODO: this isn't reserved on 1170.
    /// It's "device mode type", with a default value
    /// of "generic."
    _reserved1: [u8; 1], // 0x011
    wait_time_cfg_commands: WaitTimeConfigurationCommands,
    device_mode_sequence: DeviceModeSequence,
    device_mode_arg: u32,
    config_cmd_enable: u8,
    _reserved2: [u8; 3], // 0x01D
    config_cmd_seqs: [u8; 12],
    _reserved3: [u8; 4], // 0x02C
    cfg_cmd_args: [u8; 12],
    _reserved4: [u8; 4], // 0x03C
    controller_misc_options: u32,
    pub(crate) device_type: u8,
    serial_flash_pad_type: FlashPadType,
    serial_clk_freq: SerialClockFrequency,
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

impl ConfigurationBlock {
    /// Create a new configuration block that uses `lookup_table` as the
    /// FlexSPI LUT
    pub const fn new(lookup_table: LookupTable) -> Self {
        ConfigurationBlock {
            tag: TAG,
            version: VERSION_DEFAULT,
            read_sample_clk_src: ReadSampleClockSource::InternalLoopback,
            cs_hold_time: RECOMMENDED_CS_HOLD_TIME,
            cs_setup_time: RECOMMENDED_CS_SETUP_TIME,
            column_address_width: ColumnAddressWidth::OtherDevices,
            device_mode_configuration: 0, // Disabled
            wait_time_cfg_commands: WaitTimeConfigurationCommands::disable(),
            device_mode_sequence: DeviceModeSequence::new(0, 0),
            device_mode_arg: 0,
            config_cmd_enable: 0,
            config_cmd_seqs: [0; 12],
            cfg_cmd_args: [0; 12],
            controller_misc_options: 0,
            device_type: 0, // Invalid value; must be updated in NOR / NAND configuration block
            serial_flash_pad_type: FlashPadType::Single,
            serial_clk_freq: SerialClockFrequency::MHz30,
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
            lookup_table,
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

    /// Override the version.
    ///
    /// The default value is [`VERSION_DEFAULT`].
    pub const fn version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    /// `readSampleClkSrc`, the clock source for FlexSPI
    ///
    /// If not set, this defaults to `ReadSampleClockSource::InternalLoopback`.
    pub const fn read_sample_clk_src(mut self, read_sample_clk_src: ReadSampleClockSource) -> Self {
        self.read_sample_clk_src = read_sample_clk_src;
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
        self.column_address_width = column_address_width;
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
                self.device_mode_sequence = device_mode_seq;
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
        self.wait_time_cfg_commands = wait_time_cfg_commands;
        self
    }

    /// Sets the serial flash pad type, `sFlashPad`.
    ///
    /// If not set, this defaults to `FlashPadType::Single`.
    pub const fn serial_flash_pad_type(mut self, serial_flash_pad_type: FlashPadType) -> Self {
        self.serial_flash_pad_type = serial_flash_pad_type;
        self
    }

    /// Sets the serial clock frequencey, `serialClkFreq`
    ///
    /// If not set, this defaults to `SerialClockFrequency::MHz30`.
    pub const fn serial_clk_freq(mut self, serial_clk_freq: SerialClockFrequency) -> Self {
        self.serial_clk_freq = serial_clk_freq;
        self
    }

    /// Set a flash size for the provided flash region
    ///
    /// Any region that's not set will default to `0`.
    pub const fn flash_size(mut self, flash_region: SerialFlashRegion, flash_size: u32) -> Self {
        self.serial_flash_sizes[flash_region as usize] = flash_size;
        self
    }

    /// Set miscellaneous controller options.
    ///
    /// See your chip's reference manual for more information on valid values. This method performs
    /// no checking on the input.
    pub const fn controller_misc_options(mut self, options: u32) -> Self {
        self.controller_misc_options = options;
        self
    }
}

const _STATIC_ASSERT_SIZE: [u32; 1] =
    [0; (core::mem::size_of::<ConfigurationBlock>() == 448) as usize];
