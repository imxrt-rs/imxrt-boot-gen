//! Lookup table
//!
//! The lookup table is a 256 byte array of commands that's part of the general
//! FCB. We provide accessors that let you interact with the lookup table as either
//! a byte slice or slice of `u32`s.

use std::ops::Deref;

use crate::flexspi_lut::seq_to_bytes;
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

/// A handle to a sequence inside of the lookup table
pub struct LutSeq<'a> {
    table: &'a mut [u8],
}

impl<'a> LutSeq<'a> {
    /// Sets the sequence in the lookup table to the supplied
    /// sequence
    pub fn set(&mut self, seq: Sequence) {
        seq_to_bytes(seq, self.table);
    }
}

/// Size of the lookup table in bytes
const LOOKUP_TABLE_SIZE_BYTES: usize = 256;

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
/// lookup_table.command(SequenceCommand::Read).set(Sequence([
///     Instr::new(CMD, Pads::One, 0xEB),
///     Instr::new(RADDR, Pads::Four, 0x02),
///     STOP,
///     STOP,
///     STOP,
///     STOP,
///     STOP,
///     STOP,
/// ]));
/// ```
pub struct LookupTable([u8; LOOKUP_TABLE_SIZE_BYTES]);

impl Default for LookupTable {
    fn default() -> LookupTable {
        LookupTable([0; LOOKUP_TABLE_SIZE_BYTES])
    }
}

impl LookupTable {
    /// Create a new lookup table. All memory is set to zero.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn command(&mut self, cmd: SequenceCommand) -> LutSeq {
        // Two bytes per instruction, and eight instructions
        // in a sequence.
        const INDEX_TO_BYTE_OFFSET: usize = 8 * 2;
        let start = (cmd as usize) * INDEX_TO_BYTE_OFFSET;
        let end = start + INDEX_TO_BYTE_OFFSET;
        LutSeq {
            table: &mut self.0[start..end],
        }
    }
}

impl Deref for LookupTable {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for LookupTable {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Show that the sequence commands index into the correct
    /// place in the u8 array
    #[test]
    fn sequence_command_offset() {
        let mut lut = LookupTable::new();
        lut.command(SequenceCommand::ChipErase).set(Sequence([
            Instr::new(opcodes::sdr::CMD, Pads::Two, 0xDE),
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
        ]));
        assert_eq!(lut[11 * 16], 0xDE);
    }
}
