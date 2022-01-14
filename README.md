# TinyQOI

[![CI](https://github.com/embedded-graphics/tinyqoi/actions/workflows/ci.yml/badge.svg)](https://github.com/embedded-graphics/tinyqoi/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/tinyqoi.svg)](https://crates.io/crates/tinyqoi)
[![Docs.rs](https://docs.rs/tinyqoi/badge.svg)](https://docs.rs/tinyqoi)
[![embedded-graphics on Matrix](https://img.shields.io/matrix/rust-embedded-graphics:matrix.org)](https://matrix.to/#/#rust-embedded-graphics:matrix.org)

## [Documentation](https://docs.rs/tinyqoi)

QOI image decoder for embedded applications.

`tinyqoi` is a QOI image decoder mainly targeted at use with `embedded_graphics`.

## Examples

A `Qoi` image can be wrapped in an embedded-graphics `Image` to display
it on any `DrawTarget` which uses `Rgb888` colors:

```rust
use tinyqoi::Qoi;
use embedded_graphics::{prelude, image::Image};

// Parse QOI image.
let data = include_bytes!("tests/colors.qoi");
let qoi = Qoi::new(data).unwrap();

// Draw image to display.
Image::new(&qoi, Point::zero()).draw(&mut display).unwrap();
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
