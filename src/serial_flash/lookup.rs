//! Lookup table

use std::ops::{Index, IndexMut};

pub use crate::flexspi_lut::*;

/// The default sequence definition lookup indices
///
/// `CommandSequence`s are looked up by the processor when it needs to
/// interact with the flash chip. The enumeration lets us index back into
/// the `Lookup` struct, and associate a sequence command for that action.
#[repr(usize)]
pub enum CommandSequence {
    Read = 0,
    ReadStatus = 1,
    WriteEnable = 3,
    EraseSector = 5,
    PageProgram = 9,
    ChipErase = 11,
    Dummy = 15,
}

impl CommandSequence {
    fn command_index_name(idx: usize) -> Option<&'static str> {
        use CommandSequence::*;
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
/// methods for inserting command sequences into the table. The contents
/// of sequences are based on the FLASH chip that we're talking to. Refer
/// to your FLASH chip's refence manual.
///
/// ```
/// use imxrt_boot_gen::serial_flash::{
///     LookupTable,
///     CommandSequence,
///     Sequence, Instr,
///     opcodes::sdr::*,
///     Pads,
///     STOP,
/// };
///
/// let mut lookup_table = LookupTable::new();
/// lookup_table[CommandSequence::Read] = Sequence([
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
            .map(|(idx, instr)| (instr, CommandSequence::command_index_name(idx)))
    }
}

impl Index<CommandSequence> for LookupTable {
    type Output = Sequence;

    fn index(&self, cmd: CommandSequence) -> &Sequence {
        &self.0[cmd as usize]
    }
}

impl IndexMut<CommandSequence> for LookupTable {
    fn index_mut(&mut self, cmd: CommandSequence) -> &mut Sequence {
        &mut self.0[cmd as usize]
    }
}
