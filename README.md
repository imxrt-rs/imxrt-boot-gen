# imxrt-boot-gen

Generate iMXRT data structures that are required for booting.

## Terms

- Flex Serial Peripheral Interface (FlexSPI) Configuration Block (FCB), a data structure that describes how the processor interfaces flash devices via FlexSPI. It's an array of magic numbers placed in flash at a known location. Suitable for serial NOR and NAND flash booting.

## Rationale

This crate lets you define iMXRT data structures that are required to boot, like FCBs. FCBs are typically written by hand. If you're defining a FCB in C, you can use structs and macros to define the FCB layout and values. The approach, however, isn't great for catching invalid values at compile time. This crate takes a different, more radical approach, by using Rust to generate the data structures at compile time.

`imxrt-boot-gen` provides an API for FCB generation. Use it in another crate's `build.rs` to define the FCB, and write it to a file:

```rust
let fcb = FCBBuilder::new(DeviceType::SerialNOR(nor_cb), lookup_table)
        .read_sample_clk_src(ReadSampleClockSource::LoopbackFromDQSPad)
        .cs_hold_time(0x01)
        .cs_setup_time(0x02)
        .column_address_width(ColumnAddressWidth::OtherDevices)
        .device_mode_configuration(DeviceModeConfiguration::Disabled)
        // Other fields...
        .build()
        .unwrap();
let out_dir = env::var("OUT_DIR").unwrap();
let dest_path = Path::new(&out_dir).join("fcb.rs");
let mut f = File::create(&dest_path).unwrap();
writeln!(f, "{}", fcb).unwrap();
```

Then, inside your crate's `lib.rs`, include the generated file:

```rust
include!(concat!(env!("OUT_DIR"), "/fcb.rs"));
```

Your crate now exports a FCB that resembles

```rust
#[link_section = ".fcb"]
#[no_mangle]
pub static FLEXSPI_CONFIGURATION_BLOCK: [u8; 512] = [
    0x46, // 0x000 Tag 'FCFB'
    0x43, // 0x001 
    0x46, // 0x002 
    0x42, // 0x003 
    0x00, // 0x004 Version 'bugfix'
    0x00, // 0x005 Version 'minor'
    0x01, // 0x006 Version 'major
    0x56, // 0x007 Version 'V'
    // ...
];
```

You may now link that crate into another executable. Make sure that you place the FCB at the correct location in flash! The correct location varies by processor and boot configuration; consult your iMXRT reference manual for more information.

## ABI

The generated FCB has the symbol `FLEXSPI_CONFIGURATION_BLOCK`. The symbol is not mangled. The memory is an array of 512 `u8`s. It has a link section of `".fcb"`. The ABI ensures compatibility with both Rust and C. By building a C static library from your Rust crate, you can link the FCB into other C applications that target the iMXRT processor family.

## Supported Processors

The list below note the crate's compatibility with iMXRT chips. The crate selectively enables and disables features based on your processor. Processors that are not selected have not been evaluated or implemented.

- [x] imxrt1011
- [ ] imxrt1015
- [ ] imxrt1021
- [ ] imxrt1051
- [ ] imxrt1052
- [x] imxrt1061
- [x] imxrt1062
- [x] imxrt1064

Select your processor by enabling the corresponding feature:

```toml
[build-dependencies]
imxrt-boot-gen = { features = ["imxrt1062"] }
```

## Examples

See the [`teensy4-fcb` crate](https://crates.io/crates/teensy4-fcb) for an example of how to use the `imxrt-boot-gen` crate. The `teensy4-rs` project uses this crate to generate the FlexSPI firmware configuration block.
