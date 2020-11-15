# Changelog

## [Unreleased] - YYYY-MM-DD

### Added

- Added `SequenceBuilder` to support `Sequence` allocation. `SequenceBuilder`
  requires fewer lines of code to define the same FlexSPI LUT sequence, and it
  catches errors at compile time.

### Removed

- Removed the `Sequence` public interface. Users should change their `Sequence`
  definitions to use `SequenceBuilder`. The example below compares the old
  `Sequence` API with the new `SequenceBuilder` API:

    ```rust
    // Old API:
    const SEQ_READ: Sequence = Sequence([
        Instr::new(CMD, Pads::One, 0xEB),
        Instr::new(READ, Pads::Four, 0x04),
        STOP,
        STOP,
        STOP,
        STOP,
        STOP,
        STOP,
    ]);
    
    // New API:
    const SEQ_READ: Sequence = SequenceBuilder::new()
        .instr(Instr::new(CMD, Pads::One, 0xEB))
        .instr(Instr::new(READ, Pads::Four, 0x04))
        .build();
    ```

## [0.1.0] - 2020-04-07

First release

[Unreleased]: https://github.com/imxrt-rs/imxrt-boot-gen/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/imxrt-rs/imxrt-boot-gen/releases/tag/v0.1.0