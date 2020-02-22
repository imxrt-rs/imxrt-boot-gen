//! The `Builder`
//!
//! The builder turns fields into the FlexSPI configuration block.

use super::fcb;
use super::fields::*;
use super::lookup::LookupTable;

/// The recommended `csHoldTime`, `0x03`.
///
/// This is the default value if not set in [`cs_hold_time()`](struct.FCBBuilder.html#method.cs_hold_time).
pub const RECOMMENDED_CS_HOLD_TIME: u8 = 0x03;
/// The recommended `csSetupTime`, `0x03`.
///
/// This is the default value if not set in [`cs_setup_time()`](struct.FCBBuilder.html#method.cs_setup_time).
pub const RECOMMENDED_CS_SETUP_TIME: u8 = 0x03;

/// Builder for a FlexSPI configuration block
/// that configures reads from serial flash.
///
/// See the builder methods, or any associated types, for
/// more information. Most FCB fields will have a default value
/// unless you override the value.
pub struct FCBBuilder {
    read_sample_clk_src: ReadSampleClockSource,
    cs_hold_time: u8,
    cs_setup_time: u8,
    column_address_width: ColumnAddressWidth,
    device_mode_configuration: DeviceModeConfiguration,
    wait_time_cfg_commands: WaitTimeConfigurationCommands,
    device_type: DeviceType,
    serial_flash_pad_type: FlashPadType,
    serial_clk_freq: SerialClockFrequency,
    flash_size: SerialFlashSize,
    lookup_table: LookupTable,
}

impl FCBBuilder {
    /// Create a new FCB builder for the specified `device_type` using the provided `lookup_table`.
    pub fn new(device_type: DeviceType, lookup_table: LookupTable) -> Self {
        FCBBuilder {
            read_sample_clk_src: ReadSampleClockSource::InternalLoopback,
            cs_hold_time: RECOMMENDED_CS_HOLD_TIME,
            cs_setup_time: RECOMMENDED_CS_SETUP_TIME,
            column_address_width: ColumnAddressWidth::OtherDevices,
            device_mode_configuration: DeviceModeConfiguration::Disabled,
            wait_time_cfg_commands: WaitTimeConfigurationCommands::disable(),
            device_type,
            serial_flash_pad_type: FlashPadType::Single,
            serial_clk_freq: SerialClockFrequency::MHz30,
            flash_size: SerialFlashSize::new(),
            lookup_table,
        }
    }

    /// `readSampleClkSrc`, the clock source for FlexSPI
    ///
    /// If not set, this defaults to `ReadSampleClockSource::InternalLoopback`.
    pub fn read_sample_clk_src(&mut self, read_sample_clk_src: ReadSampleClockSource) -> &mut Self {
        self.read_sample_clk_src = read_sample_clk_src;
        self
    }

    /// Set the chip select hold time (`csHoldTime`)
    ///
    /// If not set, this will be `RECOMMENDED_CS_HOLD_TIME`, which is `0x03`.
    pub fn cs_hold_time(&mut self, cs_hold_time: u8) -> &mut Self {
        self.cs_hold_time = cs_hold_time;
        self
    }

    /// Set the chip select setup time (`csSetupTime`)
    ///
    /// If not set, this will be `RECOMMENDED_CS_SETUP_TIME`, which is `0x03`.
    pub fn cs_setup_time(&mut self, cs_setup_time: u8) -> &mut Self {
        self.cs_setup_time = cs_setup_time;
        self
    }

    /// `columnAddressWidth`, the properties of the flash memory
    ///
    /// If not set, this defaults to `ColumnAddressWidth::OtherDevices`
    pub fn column_address_width(&mut self, column_address_width: ColumnAddressWidth) -> &mut Self {
        self.column_address_width = column_address_width;
        self
    }

    /// Sets device configuration mode. The `DeviceModeConfiguration::Disabled` variant
    /// will set `deviceModeCfgEnable` to "disabled". Otherwise, we will set
    /// `deviceModeCfgEnable` to "enabled," and we use the sequence and argument
    /// parameters in the FCB.
    ///
    /// If not set, this defaults to `DeviceModeConfiguration::Disabled`.
    pub fn device_mode_configuration(
        &mut self,
        device_mode_configuration: DeviceModeConfiguration,
    ) -> &mut Self {
        self.device_mode_configuration = device_mode_configuration;
        self
    }

    /// Sets `waitTimeCfgCommands`
    ///
    /// If not set, this defaults to `WaitTimeConfigurationCommands::disable()`.
    pub fn wait_time_cfg_commands(
        &mut self,
        wait_time_cfg_commands: WaitTimeConfigurationCommands,
    ) -> &mut Self {
        self.wait_time_cfg_commands = wait_time_cfg_commands;
        self
    }

    /// Sets the serial flash pad type, `sFlashPad`.
    ///
    /// If not set, this defaults to `FlashPadType::Single`.
    pub fn serial_flash_pad_type(&mut self, serial_flash_pad_type: FlashPadType) -> &mut Self {
        self.serial_flash_pad_type = serial_flash_pad_type;
        self
    }

    /// Sets the serial clock frequencey, `serialClkFreq`
    ///
    /// If not set, this defaults to `SerialClockFrequency::MHz30`.
    pub fn serial_clk_freq(&mut self, serial_clk_freq: SerialClockFrequency) -> &mut Self {
        self.serial_clk_freq = serial_clk_freq;
        self
    }

    /// Set a flash size for the provided flash region
    ///
    /// Any region that's not set will default to `0`.
    pub fn flash_size(&mut self, flash_region: SerialFlashRegion, flash_size: u32) -> &mut Self {
        self.flash_size[flash_region] = flash_size;
        self
    }

    /// Turns the `Builder` into an `FCB`, or returns an error if there
    /// is something incorrect.
    ///
    /// # Panics
    ///
    /// `build` may panic if there is an error in the implementation that
    /// ends up writing a field to a reserved offset in the FCB.
    pub fn build(&self) -> Result<fcb::FCB, Box<dyn std::error::Error>> {
        let mut fcb = fcb::FCB::new();

        self.serialze_lookup(&mut fcb);

        fcb.field_comment(
            0x00C,
            &(self.read_sample_clk_src as u8).to_le_bytes(),
            "readSampleClkSrc",
        );
        fcb.field_comment(0x00D, &[self.cs_hold_time], "csHoldTime");
        fcb.field_comment(0x00E, &[self.cs_setup_time], "csSetupTime");
        fcb.field_comment(
            0x00F,
            &[self.column_address_width as u8],
            "columnAddressWidth",
        );
        fcb.field_comment(
            0x010,
            match self.device_mode_configuration {
                DeviceModeConfiguration::Disabled => &[0],
                // TODO other fields
                DeviceModeConfiguration::Enabled { .. } => &[1],
            },
            "deviceModeCfgEnable",
        );
        fcb.field_comment(
            0x013,
            &self.wait_time_cfg_commands.0.to_le_bytes(),
            "waitTimeCfgCommands",
        );

        if let DeviceModeConfiguration::Enabled {
            device_mode_seq,
            device_mode_arg,
        } = self.device_mode_configuration
        {
            fcb.field_comment(0x014, &device_mode_seq.0, "deviceModeSeq");
            fcb.field_comment(0x018, &device_mode_arg.to_le_bytes(), "deviceModeArg");
        }

        // TODO configCmdEnable
        // TODO configCmdSeqs
        // TODO cfgCmdArgs
        // TODO controllerMiscOption

        fcb.field_comment(
            0x044,
            match self.device_type {
                DeviceType::SerialNOR(_) => &[1],
            },
            "deviceType",
        );
        fcb.field_comment(
            0x045,
            &(self.serial_flash_pad_type as u8).to_le_bytes(),
            "sflashPadType",
        );
        fcb.field_comment(
            0x046,
            &(self.serial_clk_freq as u8).to_le_bytes(),
            "serialClkFreq",
        );

        // TODO lutCustomSeqEnable

        // TODO after adding SerialNAND, we have to multiply all
        // the flash sizes by 2
        fcb.field_comment(
            0x050,
            &self.flash_size[SerialFlashRegion::A1].to_le_bytes(),
            "sflashA1Size",
        );
        fcb.field_comment(
            0x054,
            &self.flash_size[SerialFlashRegion::A2].to_le_bytes(),
            "sflashA2Size",
        );
        fcb.field_comment(
            0x058,
            &self.flash_size[SerialFlashRegion::B1].to_le_bytes(),
            "sflashB1Size",
        );
        fcb.field_comment(
            0x05C,
            &self.flash_size[SerialFlashRegion::B2].to_le_bytes(),
            "sflashB2Size",
        );

        // TODO csPadSettingOverride
        // TODO sclkPadSettingOverride
        // TODO dataPadSettingOverride
        // TODO dqsPadSettingOverride
        // TODO timeoutInMs
        // TODO commandInverval
        // TODO dataValidTime
        // TODO busyOffset
        // TODO busyBitPolarity

        match self.device_type {
            DeviceType::SerialNOR(norcb) => {
                fcb.field_comment(0x1C0, &norcb.page_size.to_le_bytes(), "pageSize");
                fcb.field_comment(0x1C4, &norcb.sector_size.to_le_bytes(), "sectorSize");
                fcb.field_comment(
                    0x1C8,
                    &(norcb.ip_cmd_serial_clk_freq as u8).to_le_bytes(),
                    "ipCmdSerialClkFreq",
                );
            }
        }
        Ok(fcb)
    }

    fn serialze_lookup(&self, fcb: &mut fcb::FCB) {
        const LOOKUP_TABLE_OFFSET: usize = 0x080;
        let mut offset = 0;
        for (seq_idx, (seq, cmd)) in self.lookup_table.iter().enumerate() {
            for instr in seq.0.iter() {
                let raw = instr.raw();
                if let Some(cmd) = cmd {
                    fcb.field_comment(
                        LOOKUP_TABLE_OFFSET + offset,
                        &raw,
                        format!("(LUT[{}]) {}: {}", seq_idx, cmd, instr),
                    );
                } else {
                    fcb.field_comment(
                        LOOKUP_TABLE_OFFSET + offset,
                        &raw,
                        format!("(LUT[{}])", seq_idx),
                    );
                }
                offset += raw.len();
            }
        }
    }
}
