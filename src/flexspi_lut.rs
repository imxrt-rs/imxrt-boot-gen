//! FlexSPI Lookup Table (LUT) instructions, opcodes, and sequences
//!
//! Derived from the iMXRT1060 Reference Manual (Rev 2),
//! section 27.5.8.

use std::fmt;

pub(crate) const INSTRUCTION_SIZE: usize = 2;

/// A FlexSPI instruction
///
/// An `Instr` has an opcode, a pad count, and an opcode-dependent operand.
/// Opcodes are available in the [`opcode` module](opcodes/index.html).
///
/// `Instr`s are used to create FlexSPI lookup table command [`Sequence`s](struct.Sequence.html).
#[derive(Clone, Copy)]
pub struct Instr {
    /// Raw instructions
    raw: [u8; INSTRUCTION_SIZE],
    opcode: Opcode,
    pads: Pads,
}

impl Instr {
    /// Create a new FlexSPI LUT instruction
    ///
    /// Note that the `JUMP_ON_CS` and `STOP` opcodes are not available. However,
    /// there are pre-defined [`JUMP_ON_CS`](constant.JUMP_ON_CS.html) and [`STOP`](constant.STOP.html)
    /// instructions which you should use.
    pub const fn new(opcode: Opcode, pads: Pads, operand: u8) -> Self {
        let x: 
            i32;
        Instr {
            // Little endian
            raw: [operand, (opcode.0 << 2) | (pads as u8)],
            opcode,
            pads,
        }
    }

    const fn stop() -> Self {
        Instr {
            raw: [0; INSTRUCTION_SIZE],
            opcode: opcodes::STOP,
            pads: Pads::One, // unused
        }
    }

    const fn jump_on_cs() -> Self {
        Instr {
            raw: [0; INSTRUCTION_SIZE],
            opcode: opcodes::JUMP_ON_CS,
            pads: Pads::One, // unused
        }
    }

    /// Returns the raw bytes representing this instruction
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.opcode {
            opcodes::STOP => write!(f, "STOP"),
            opcodes::JUMP_ON_CS => write!(f, "JUMP_ON_CS"),
            opcode => write!(
                f,
                "OPCODE={}, PADS={}, OPERAND={:#02X}",
                opcode, self.pads, self.raw[0]
            ),
        }
    }
}

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let raw = u16::from_le_bytes(self.raw);
        write!(f, "{:#02X}", raw)
    }
}

/// STOP FlexSPI instruction
pub const STOP: Instr = Instr::stop();
/// JUMP_ON_CS FlexSPI instruction
pub const JUMP_ON_CS: Instr = Instr::jump_on_cs();

pub(crate) const INSTRUCTIONS_PER_SEQUENCE: usize = 8;

/// A collection of FlexSPI LUT instructions
///
/// Each `Sequence` may have up to eight instructions. Any unused instructions must
/// be inlined to [`STOP`](constant.STOP.html). The sequences you'll require are dependent
/// on the specific flash memory that you're interacting with.
///
/// `Sequence`s are used to create a [`LookupTable`](../serial_flash/lookup/struct.LookupTable.html).
///
/// # Example
///
/// ```
/// use imxrt_boot_gen::serial_flash::{
///     Sequence,
///     Instr,
///     STOP,
///     Pads,
///     opcodes::sdr::*,
/// };
///
/// const SEQ_READ: Sequence = Sequence([
///     Instr::new(CMD, Pads::One, 0xEB),
///     Instr::new(READ, Pads::Four, 0x04),
///     STOP,
///     STOP,
///     STOP,
///     STOP,
///     STOP,
///     STOP,
/// ]);
/// ```
#[derive(Clone, Copy)]
pub struct Sequence(pub [Instr; INSTRUCTIONS_PER_SEQUENCE]);
pub(crate) const SEQUENCE_SIZE: usize = INSTRUCTIONS_PER_SEQUENCE * INSTRUCTION_SIZE;

impl Sequence {
    pub(crate) const fn stopped() -> Self {
        Sequence([STOP; INSTRUCTIONS_PER_SEQUENCE])
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let instr_strs: Vec<String> = self.0.iter().map(ToString::to_string).collect();
        write!(f, "{}", instr_strs.join(";"))
    }
}

/// A FlexSPI opcode
///
/// Available `Opcode`s are defined in the `opcodes` module.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Opcode(u8);

/// Number of pads to use to execute the instruction
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Pads {
    /// Single mode
    One = 0x00,
    /// Dual mode
    Two = 0x01,
    /// Quad mode
    Four = 0x02,
    /// Octal mode
    Eight = 0x03,
}

impl fmt::Display for Pads {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pads = match *self {
            Pads::One => "SINGLE",
            Pads::Two => "DUAL",
            Pads::Four => "QUAD",
            Pads::Eight => "OCTAL",
        };
        write!(f, "{}", pads)
    }
}

impl fmt::Debug for Pads {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#02X}", *self as u8)
    }
}

/// FlexSPI lookup table instruction opcodes
///
/// Opcodes are separated by their data transfer rates. General opcodes
/// are in the top-level `opcodes` module.
pub mod opcodes {
    use super::Opcode;

    /// Single data transfer rate (SDR) opcodes
    pub mod sdr {
        use super::Opcode;
        /// Transmit command code to flash
        pub const CMD: Opcode = Opcode(0x01);
        /// Transmit row address to flash
        pub const RADDR: Opcode = Opcode(0x02);
        /// Transmit column address to flash
        pub const CADDR: Opcode = Opcode(0x03);
        /// Transmit mode bits to flash
        ///
        /// Bit number 1
        pub const MODE1: Opcode = Opcode(0x04);
        /// Transmit mode bits to flash
        ///
        /// Bit number 2
        pub const MODE2: Opcode = Opcode(0x05);
        /// Transmit mode bits to flash
        ///
        /// Bit number 4
        pub const MODE4: Opcode = Opcode(0x06);
        /// Transmit mode bits to flash
        ///
        /// Bit number 8
        pub const MODE8: Opcode = Opcode(0x07);
        /// Transmit programming data to flash
        pub const WRITE: Opcode = Opcode(0x08);
        /// Receive data from flash
        ///
        /// Read Data is put into AHB_RX_BUF or IP_RX_FIFO.
        pub const READ: Opcode = Opcode(0x09);
        /// Receive Read Data or Preamble bit from Flash device
        ///
        /// FlexSPI Controller will compare the data line bits with DLPR
        /// register to determine a correct sampling clock phase.
        pub const LEARN: Opcode = Opcode(0x0A);
        /// Transmit Read/ Program Data size (byte number) to Flash
        pub const DATASZ: Opcode = Opcode(0x0B);
        /// Leave data lines undriven by FlexSPI controller.
        ///
        /// Provide turnaround cycles from host driving to device driving.
        /// `num_pads` will determine the number of pads in input mode.
        pub const DUMMY: Opcode = Opcode(0x0C);
        /// Similar to `DUMMY`, but the cycle number is different
        pub const DUMMY_RWDS: Opcode = Opcode(0x0D);
    }

    /// Stop execution, deassert CS. Next command sequence
    /// (to the same flash device) will started from instruction pointer 0.
    pub(super) const STOP: Opcode = Opcode(0x00);
    /// Stop execution, deassert CS and save operand[7:0]
    /// as the instruction start pointer for next sequence.
    ///
    /// Normally this instruction is used to support XIP enhance mode.
    pub(super) const JUMP_ON_CS: Opcode = Opcode(0x1F);

    /// Dual data transfer rate (DDR) opcodes
    ///
    /// See the documentation on the corresponding [`ssr` opcode](../sdr/index.html)
    /// for more information.
    pub mod ddr {
        use super::sdr;
        use super::Opcode;

        /// Adds `0x20` to the opcode to make it a DDR opcode
        const fn to_ddr(opcode: Opcode) -> Opcode {
            Opcode(opcode.0 + 0x20)
        }

        pub const CMD: Opcode = to_ddr(sdr::CMD);
        pub const RADDR: Opcode = to_ddr(sdr::RADDR);
        pub const CADDR: Opcode = to_ddr(sdr::CADDR);
        pub const MODE1: Opcode = to_ddr(sdr::MODE1);
        pub const MODE2: Opcode = to_ddr(sdr::MODE2);
        pub const MODE4: Opcode = to_ddr(sdr::MODE4);
        pub const MODE8: Opcode = to_ddr(sdr::MODE8);
        pub const WRITE: Opcode = to_ddr(sdr::WRITE);
        pub const READ: Opcode = to_ddr(sdr::READ);
        pub const LEARN: Opcode = to_ddr(sdr::LEARN);
        pub const DATASZ: Opcode = to_ddr(sdr::DATASZ);
        pub const DUMMY: Opcode = to_ddr(sdr::DUMMY);
        pub const DUMMY_RWDS: Opcode = to_ddr(sdr::DUMMY_RWDS);
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use opcodes::ddr;
        use opcodes::sdr;
        match *self {
            // SDR
            sdr::CMD => write!(f, "CMD_SDR"),
            sdr::RADDR => write!(f, "RADDR_SDR"),
            sdr::CADDR => write!(f, "CADDR_SDR"),
            sdr::MODE1 => write!(f, "MODE1_SDR"),
            sdr::MODE2 => write!(f, "MODE2_SDR"),
            sdr::MODE4 => write!(f, "MODE4_SDR"),
            sdr::MODE8 => write!(f, "MODE8_SDR"),
            sdr::WRITE => write!(f, "WRITE_SDR"),
            sdr::READ => write!(f, "READ_SDR"),
            sdr::LEARN => write!(f, "LEARN_SDR"),
            sdr::DATASZ => write!(f, "DATASZ_SDR"),
            sdr::DUMMY => write!(f, "DUMMY_SDR"),
            sdr::DUMMY_RWDS => write!(f, "DUMMY_RWDS_SDR"),
            // DDR
            ddr::CMD => write!(f, "CMD_DDR"),
            ddr::RADDR => write!(f, "RADDR_DDR"),
            ddr::CADDR => write!(f, "CADDR_DDR"),
            ddr::MODE1 => write!(f, "MODE1_DDR"),
            ddr::MODE2 => write!(f, "MODE2_DDR"),
            ddr::MODE4 => write!(f, "MODE4_DDR"),
            ddr::MODE8 => write!(f, "MODE8_DDR"),
            ddr::WRITE => write!(f, "WRITE_DDR"),
            ddr::READ => write!(f, "READ_DDR"),
            ddr::LEARN => write!(f, "LEARN_DDR"),
            ddr::DATASZ => write!(f, "DATASZ_DDR"),
            ddr::DUMMY => write!(f, "DUMMY_DDR"),
            ddr::DUMMY_RWDS => write!(f, "DUMMY_RWDS_DDR"),
            // Others
            opcodes::STOP => write!(f, "STOP"),
            opcodes::JUMP_ON_CS => write!(f, "JUMP_ON_CS"),
            // Should be unreachable
            unknown => write!(f, "UNKNOWN({:#02X})", unknown.0),
        }
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#02X}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::opcodes::sdr::*;
    use super::Instr;
    use super::Pads;
    use super::Sequence;
    use super::STOP;

    fn seq_to_bytes(seq: Sequence) -> Vec<u8> {
        let mut buffer = vec![0; super::SEQUENCE_SIZE];
        buffer
            .chunks_exact_mut(2)
            .zip(seq.0.iter())
            .for_each(|(dst, src)| dst.copy_from_slice(&src.raw));
        buffer
    }

    // Tests were implemented by a study of the
    // known-good Teensy 4 FCB lookup table.
    //
    // See table Table 9-16. LUT sequence definition for Serial NOR,
    // to better understand the meaning behind the sequences.

    #[test]
    fn teensy4_read() {
        const EXPECTED: [u8; super::SEQUENCE_SIZE] = [
            0xEB, 0x04, 0x18, 0x0A, 0x06, 0x32, 0x04, 0x26, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let seq = Sequence([
            Instr::new(CMD, Pads::One, 0xEB),
            Instr::new(RADDR, Pads::Four, 0x18),
            Instr::new(DUMMY, Pads::Four, 0x06),
            Instr::new(READ, Pads::Four, 0x04),
            STOP,
            STOP,
            STOP,
            STOP,
        ]);

        assert_eq!(&seq_to_bytes(seq), &EXPECTED);
    }

    #[test]
    fn teensy4_read_status() {
        const EXPECTED: [u8; 4] = [0x05, 0x04, 0x04, 0x24];
        let seq = Sequence([
            Instr::new(CMD, Pads::One, 0x05),
            Instr::new(READ, Pads::One, 0x04),
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
        ]);
        assert_eq!(&seq_to_bytes(seq)[0..4], &EXPECTED);
    }

    #[test]
    fn teensy4_write_enable() {
        const EXPECTED: u128 = 0x0000_0406;
        let seq = Sequence([
            Instr::new(CMD, Pads::One, 0x06),
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
        ]);
        assert_eq!(&EXPECTED.to_le_bytes(), &seq_to_bytes(seq)[..]);
    }

    #[test]
    fn teensy4_erase_sector() {
        const EXPECTED: u128 = 0x0818_0420;
        let seq = Sequence([
            Instr::new(CMD, Pads::One, 0x20),
            Instr::new(RADDR, Pads::One, 0x18),
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
        ]);
        assert_eq!(&EXPECTED.to_le_bytes(), &seq_to_bytes(seq)[..]);
    }

    #[test]
    fn teensy4_page_program() {
        const EXPECTED: u128 = 0x0000_2004_0818_0402;
        let seq = Sequence([
            Instr::new(CMD, Pads::One, 0x02),
            Instr::new(RADDR, Pads::One, 0x18),
            Instr::new(WRITE, Pads::One, 0x04),
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
        ]);
        assert_eq!(&EXPECTED.to_le_bytes(), &seq_to_bytes(seq)[..]);
    }

    #[test]
    fn teensy4_chip_erase() {
        const EXPECTED: u128 = 0x0000_0460;
        let seq = Sequence([
            Instr::new(CMD, Pads::One, 0x60),
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
        ]);
        assert_eq!(&EXPECTED.to_le_bytes(), &seq_to_bytes(seq)[..]);
    }
}
