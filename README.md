# imxrt-fcb-gen

Define an iMXRT firmware configuration block (FBC) in Rust.

## Rationale

A firmware configuration block (FCB) is an array of memory that defines how the processor should read from flash. It is placed in a known location in flash. FCBs are typically written by hand. If you're defining a FCB in C, you can use structs and macros to define the FCB layout and values. The approach, however, isn't great for catching invalid values at compile time. This crate takes a different, more radical approach, by using Rust to generate the FCB at compile time.

`imxrt-fcb-gen` provides an API for FCB generation. Use it in another crate's `build.rs` to define the FCB, and write it to a file:

```rust
let builder = Builder {
    read_sample_clock_source: ReadSampleClockSource::LoopbackFromDQSPad,
    cs_hold_time: CSHoldTime::new(0x01),
    cs_setup_time: CSSetupTime::new(0x02),
    column_address_width: ColumnAddressWidth::other_devices(),
    device_mode_configuration: DeviceModeConfiguration::Disabled,
    // Other members...
};
let fcb = builder.build().unwrap();
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
pub static FIRMWARE_CONFIGURATION_BLOCK: [u8; 512] = [
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

The generated FCB has the symbol `FIRMWARE_CONFIGURATION_BLOCK`. The symbol is not mangled. The memory is an array of 512 `u8`s. It has a link section of `".fcb"`. The ABI ensures compatibility with both Rust and C. Indeed, by building a C static library from your Rust crate, you can link the FCB into other C applications that target the iMXRT processor family.

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
imxrt-fcb-gen = { features = ["imxrt1062"] }
```

## Examples

See the [`teensy4-fcb` crate](https://crates.io/crates/teensy4-fcb) for an example of how to use the `imxrt-fcb-gen` crate.
