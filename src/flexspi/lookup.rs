//! FlexSPI Lookup table

use super::sequence::{SEQUENCE_SIZE, Sequence};

/// The default sequence definition lookup indices
///
/// `Command`s are looked up by the processor when it needs to
/// interact with the flash chip. The enumeration lets us index back into
/// the `Lookup` struct, and associate a sequence command for that action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Command {
    Read = 0,
    ReadStatus = 1,
    WriteEnable = 3,
    EraseSector = 5,
    PageProgram = 9,
    ChipErase = 11,
    Dummy = 15,
}

/// Size of the lookup table in bytes
const LOOKUP_TABLE_SIZE_BYTES: usize = 256;
const NUMBER_OF_SEQUENCES: usize = LOOKUP_TABLE_SIZE_BYTES / SEQUENCE_SIZE;

/// A sequence lookup table, part of the general FlexSPI configuration block
///
/// The contents of the sequences depend on what kind of FLASH device we're
/// interfacing. Refer to your FLASH device manual for more information.
///
/// Any unspecified command is set to a sequence of STOPs.
///
/// ```
/// use imxrt_boot_gen::flexspi::{
///     LookupTable,
///     Command,
///     SequenceBuilder,
///     Sequence, Instr,
///     opcodes::sdr::*,
///     Pads,
/// };
///
/// const LUT: LookupTable = LookupTable::new()
///     .command(Command::Read, SequenceBuilder::new()
///         .instr(Instr::new(CMD, Pads::One, 0xEB))
///         .instr(Instr::new(RADDR, Pads::Four, 0x02))
///         .build());
/// ```
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct LookupTable([Sequence; NUMBER_OF_SEQUENCES]);

impl Default for LookupTable {
    fn default() -> Self {
        Self::new()
    }
}

impl LookupTable {
    /// Create a new lookup table. All memory is set to zero.
    pub const fn new() -> Self {
        LookupTable([Sequence::stopped(); NUMBER_OF_SEQUENCES])
    }
    /// Assign the `sequence` to the command that is found at the `Command` index
    pub const fn command(self, cmd: Command, sequence: Sequence) -> Self {
        self.custom_command(cmd as _, sequence)
    }
    /// Assign a sequence to one of the freely-available LUT indexes.
    ///
    /// You're responsible for making sure this doesn't accidentally
    /// overwrite one of the standard [`Command`] sequences.
    pub const fn custom_command(mut self, index: usize, sequence: Sequence) -> Self {
        self.0[index] = sequence;
        self
    }
}

#[cfg(test)]
mod test {
    use super::{Command, LookupTable};
    use crate::flexspi::sequence::SequenceBuilder;

    #[test]
    fn smoke() {
        const _LUT: LookupTable = LookupTable::new()
            .command(Command::Read, SequenceBuilder::new().build())
            .command(Command::ReadStatus, SequenceBuilder::new().build())
            .command(Command::WriteEnable, SequenceBuilder::new().build())
            .command(Command::EraseSector, SequenceBuilder::new().build())
            .command(Command::PageProgram, SequenceBuilder::new().build())
            .command(Command::ChipErase, SequenceBuilder::new().build())
            .command(Command::Dummy, SequenceBuilder::new().build());
    }
}
