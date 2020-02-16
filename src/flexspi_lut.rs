//! FlexSPI Lookup Table (LUT) instruction set
//!
//! Derived from the iMXRT1060 Reference Manual (Rev 2),
//! section 27.5.8

use std::fmt;

/// A FlexSPI instruction
#[derive(Clone, Copy)]
pub struct Instr {
    raw: [u8; 2],
    opcode: Opcode,
    pads: NumPads,
}

impl Instr {
    /// Create a new FlexSPI LUT instruction
    ///
    /// If defining a `STOP` or `JUMP_ON_CS` instruction, ensure that you set `pads` to `One`.
    pub const fn new(opcode: Opcode, pads: NumPads, operand: u8) -> Self {
        Instr {
            // Little endian
            raw: [operand, (opcode.0 << 2) | (pads as u8)],
            opcode,
            pads,
        }
    }

    const fn stop() -> Self {
        Instr {
            raw: [0; 2],
            opcode: opcodes::STOP,
            pads: NumPads::One, // unused
        }
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.opcode {
            opcodes::STOP => write!(f, "STOP"),
            opcodes::JUMP_ON_CS => write!(f, "JUMP_ON_CS"),
            opcode => write!(f, "{},{},{:#02X}", opcode, self.pads, self.raw[0]),
        }
    }
}

/// STOP instruction, for convenience
pub const STOP: Instr = Instr::stop();

/// A collection of FlexSPI LUT instructions
pub struct Sequence([u8; 16]);

/// Create a sequence of FlexSPI instructions from individual instructions
///
/// There can be up to 8 instructions per sequence. If you don't have eight instructions,
/// use `STOP` when you're at the end.
pub fn sequence(instrs: [Instr; 8]) -> Sequence {
    let mut seq: [u8; 16] = [0; 16];
    seq.chunks_exact_mut(2)
        .zip(instrs.iter().map(|instr| instr.raw))
        .for_each(|(dst, src)| {
            dst.copy_from_slice(&src);
        });
    Sequence(seq)
}

/// A FlexSPI opcode
///
/// `Opcode`s are defined in the `opcodes` module.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Opcode(u8);

/// Number of pads to use to execute the instruction
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum NumPads {
    /// Single mode
    One = 0x00,
    /// Dual mode
    Two = 0x01,
    /// Quad mode
    Four = 0x02,
    /// Octal mode
    Eight = 0x03,
}

impl fmt::Display for NumPads {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#02X}", *self as u8)
    }
}

/// LUT instruction opcodes
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
    pub const STOP: Opcode = Opcode(0x00);
    /// Stop execution, deassert CS and save operand[7:0]
    /// as the instruction start pointer for next sequence.
    ///
    /// Normally this instruction is used to support XIP enhance mode.
    pub const JUMP_ON_CS: Opcode = Opcode(0x1F);

    /// Dual data transfer rate (DDR) opcodes
    ///
    /// See the documentation on the corresponding `ssr` opcode
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

#[cfg(test)]
mod test {
    use super::opcodes::sdr::*;
    use super::sequence;
    use super::Instr;
    use super::NumPads;
    use super::STOP;

    #[test]
    fn quad_io_fast_read_command() {
        const EXPECTED: [u8; 16] = [
            0xEB, 0x04, 0x18, 0x0A, 0x06, 0x32, 0x04, 0x26, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let instrs = sequence([
            Instr::new(CMD, NumPads::One, 0xEB),
            Instr::new(RADDR, NumPads::Four, 0x18),
            Instr::new(DUMMY, NumPads::Four, 0x06),
            Instr::new(READ, NumPads::Four, 0x04),
            STOP,
            STOP,
            STOP,
            STOP,
        ]);

        assert_eq!(instrs.0, EXPECTED);
    }

    #[test]
    fn more_teensy4_magic_numbers() {
        const EXPECTED: [u8; 4] = [0x05, 0x04, 0x04, 0x24];
        let instrs = sequence([
            Instr::new(CMD, NumPads::One, 0x05),
            Instr::new(READ, NumPads::One, 0x04),
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
            STOP,
        ]);
        assert_eq!(&instrs.0[0..4], &EXPECTED[..]);
        assert_eq!(
            instrs.0[4..].iter().cloned().collect::<Vec<u8>>(),
            std::iter::repeat(0).take(12).collect::<Vec<u8>>()
        );
    }
}
