//! The `Builder` API
//!
//! The builder turns the fields into the FCB.

use super::fields::*;
use super::lookup::LookupTable;
use crate::fcb;

/// Builder for a firmware configuration block
/// that configures reads from serial flash.
///
/// See the documentation on the types for more information.
pub struct Builder {
    pub read_sample_clock_source: ReadSampleClockSource,
    pub cs_hold_time: CSHoldTime,
    pub cs_setup_time: CSSetupTime,
    pub column_address_width: ColumnAddressWidth,
    pub device_mode_configuration: DeviceModeConfiguration,
    pub wait_time_cfg_commands: WaitTimeConfigurationCommands,
    pub device_mode_seq: DeviceModeSequence,
    pub device_type: DeviceType,
    pub serial_flash_pad_type: FlashPadType,
    pub serial_clk_freq: SerialClockFrequency,
    pub flash_a1_size: SerialFlashSize<A1>,
    pub flash_a2_size: SerialFlashSize<A2>,
    pub flash_b1_size: SerialFlashSize<B1>,
    pub flash_b2_size: SerialFlashSize<B2>,
    pub lookup_table: LookupTable,
}

impl Builder {
    /// Turns the `Builder` into an `FCB`, or returns an error if there
    /// is something incorrect.
    ///
    /// # Panics
    ///
    /// `build` may panic if there is an error in the implementation that
    /// ends up writing a field to a reserved offset in the FCB.
    pub fn build(self) -> Result<fcb::FCB, Box<dyn std::error::Error>> {
        let mut fcb = fcb::FCB::new();

        self.serialze_lookup(&mut fcb);

        fcb.field_comment(
            0x00C,
            &(self.read_sample_clock_source as u8).to_le_bytes(),
            "readSampleClkSrc",
        );
        fcb.field_comment(0x00D, &self.cs_hold_time, "csHoldTime");
        fcb.field_comment(0x00E, &self.cs_setup_time, "csSetupTime");
        fcb.field_comment(0x00F, &self.column_address_width, "columnAddressWidth");
        fcb.field_comment(
            0x010,
            match self.device_mode_configuration {
                DeviceModeConfiguration::Disabled => &[0],
                DeviceModeConfiguration::Enabled(_) => &[1],
            },
            "deviceModeCfgEnable",
        );
        fcb.field_comment(0x013, &self.wait_time_cfg_commands, "waitTimeCfgCommands");
        fcb.field_comment(0x014, &self.device_mode_seq, "deviceModeSeq");

        if let DeviceModeConfiguration::Enabled(arg) = self.device_mode_configuration {
            fcb.field_comment(0x018, &arg, "deviceModeArg");
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
            &self.flash_a1_size.size.to_le_bytes(),
            "sflashA1Size",
        );
        fcb.field_comment(
            0x054,
            &self.flash_a2_size.size.to_le_bytes(),
            "sflashA2Size",
        );
        fcb.field_comment(
            0x058,
            &self.flash_b1_size.size.to_le_bytes(),
            "sflashB1Size",
        );
        fcb.field_comment(
            0x05C,
            &self.flash_b2_size.size.to_le_bytes(),
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
                fcb.field_comment(0x1C0, &norcb.page_size, "pageSize");
                fcb.field_comment(0x1C4, &norcb.sector_size, "sectorSize");
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
