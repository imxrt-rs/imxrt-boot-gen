# imxrt-boot-gen

Generate i.MX RT boot-time data structures.

[![crates-io-shield][]][crates-io]
[![docs-rs-shield][]][docs-rs]

[crates-io-shield]: https://img.shields.io/crates/v/imxrt-boot-gen
[crates-io]: https://crates.io/crates/imxrt-boot-gen
[docs-rs-shield]: https://docs.rs/imxrt-boot-gen/badge.svg
[docs-rs]: https://docs.rs/imxrt-boot-gen/

### [API docs (main branch)](https://imxrt-rs.github.io/imxrt-boot-gen/)

## Rationale

i.MX RT processors require certain data structures in order to configure
FlexSPI and SEMC peripherals. The data structurs must be placed
in a certain region of memory, with values that describe how a peripheral should
interact with that memory. The data structures only support certain values,
and need a particular layout in order to boot the system.

The `imxrt-boot-gen` crate helps you generate data structures to boot i.MX RT processors.
As of this writing, the API supports

- serial NOR flash

Other configurations, like NAND flash and parallel SEMC, may be added in the future.

## Usage

Add `imxrt-boot-gen` to your dependencies, and select your processor with a feature flag:

```toml
[dependencies]
imxrt-boot-gen = { features = ["imxrt1060"] }
```

The entire API is `const`. You may define your data structures at compile
time, and assign the values to `static` memory in your embedded program.

See the module-level documentation for more information about the API.

## Features

The crate *requires* a feature selection. Features correlate to i.MX RT processor families.
The supported features are listed below.

- `"imxrt1010"`
- `"imxrt1060"`
- `"imxrt1064"`
- `"imxrt1170"`

### License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](./LICENSE-APACHE))
- [MIT License](http://opensource.org/licenses/MIT) ([LICENSE-MIT](./LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

License: MIT OR Apache-2.0
