# imxrt-fcb-gen

Define your iMXRT firmware configuration block (FBC) in Rust.

## Supported Processors

The list below indicates the crate's compatibility with iMXRT chips. The crate selectively enables and disables features
based on your processor. Processors that are not selected have not been evaluated or implemented.

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