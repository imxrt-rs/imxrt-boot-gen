//! Lookup table
//!
//! The lookup table is a 256 byte array of commands that's part of the general
//! FCB. We provide accessors that let you interact with the lookup table as either
//! a byte slice or slice of `u32`s.

use std::ops::{Index, IndexMut};

pub use crate::flexspi_lut::*;

/// The default sequence definition lookup indices
#[repr(usize)]
pub enum SequenceCommand {
    Read = 0,
    ReadStatus = 1,
    WriteEnable = 3,
    EraseSector = 5,
    PageProgram = 9,
    ChipErase = 11,
    Dummy = 15,
}

impl SequenceCommand {
    fn command_index_name(idx: usize) -> Option<&'static str> {
        use SequenceCommand::*;
        match idx {
            idx if idx == Read as usize => Some("READ"),
            idx if idx == ReadStatus as usize => Some("READ_STATUS"),
            idx if idx == WriteEnable as usize => Some("WRITE_ENABLE"),
            idx if idx == EraseSector as usize => Some("ERASE_SECTOR"),
            idx if idx == PageProgram as usize => Some("PAGE_PROGRAM"),
            idx if idx == ChipErase as usize => Some("CHIP_ERASE"),
            idx if idx == Dummy as usize => Some("DUMMY"),
            _ => None,
        }
    }
}

/// Size of the lookup table in bytes
const LOOKUP_TABLE_SIZE_BYTES: usize = 256;
const NUMBER_OF_SEQUENCES: usize = LOOKUP_TABLE_SIZE_BYTES / SEQUENCE_SIZE;

/// The lookup table, part of the general FCB memory region.
///
/// `LookupTable` is a fixed-sized byte array. We provide convenience
/// methods for inserting command sequences into the table.
///
/// ```
/// use imxrt_fcb_gen::serial_flash::{
///     LookupTable,
///     SequenceCommand,
///     Sequence, Instr,
///     opcodes::sdr::*,
///     Pads,
///     STOP,
/// };
///
/// let mut lookup_table = LookupTable::new();
/// lookup_table[SequenceCommand::Read] = Sequence([
///     Instr::new(CMD, Pads::One, 0xEB),
///     Instr::new(RADDR, Pads::Four, 0x02),
///     STOP,
///     STOP,
///     STOP,
///     STOP,
///     STOP,
///     STOP,
/// ]);
/// ```
pub struct LookupTable([Sequence; NUMBER_OF_SEQUENCES]);

impl Default for LookupTable {
    fn default() -> LookupTable {
        LookupTable([Sequence::stopped(); NUMBER_OF_SEQUENCES])
    }
}

impl LookupTable {
    /// Create a new lookup table. All memory is set to zero.
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&Sequence, Option<&'static str>)> {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, instr)| (instr, SequenceCommand::command_index_name(idx)))
    }
}

impl Index<SequenceCommand> for LookupTable {
    type Output = Sequence;

    fn index(&self, cmd: SequenceCommand) -> &Sequence {
        &self.0[cmd as usize]
    }
}

impl IndexMut<SequenceCommand> for LookupTable {
    fn index_mut(&mut self, cmd: SequenceCommand) -> &mut Sequence {
        &mut self.0[cmd as usize]
    }
}
