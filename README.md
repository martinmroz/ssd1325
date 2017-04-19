Adafruit 2.7" Monochrome 128x64 OLED Driver
===========================================

[![](http://meritbadge.herokuapp.com/ssd1325)](https://crates.io/crates/ssd1325)
[![Build Status](https://travis-ci.org/martinmroz/ssd1325.svg?branch=master)](https://travis-ci.org/martinmroz/ssd1325)
[![Coverage Status](https://coveralls.io/repos/github/martinmroz/ssd1325/badge.svg?branch=master)](https://coveralls.io/github/martinmroz/ssd1325?branch=master)

This is a platform-agnostic driver for the Solomon Systech SSD1325 OLED display
driver IC in monochrome mode. This chip is used in the 
[Adafruit 2.7" Monochrome 128x64 OLED Display Module](https://learn.adafruit.com/2-7-monochrome-128x64-oled-display-module).
This library is transport-agnostic, in that it has no intrinsic dependencies.
Supply an `io::Write` compatible object for transferring data, such as
from [rust-spidev](https://github.com/rust-embedded/rust-spidev).
Then, implement a `ssd1325::ControlChannel` using, for instance,
[rust-sysfs-gpio](https://github.com/rust-embedded/rust-sysfs-gpio)
for controlling side-band pins (`D/NC` and `nRST`). Finally, wire up your
display and you should be all set.

## Compatibility

Tested with the aforementioned module only. This should work with any SSD1325
display, however the initialization sequence may not be sufficient. Please
submit an issue if you run into issues and I'll attempt to make the
interface more generic to support your use case.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ssd1325 = "0.1"
```

In addition, and this to your crate root:

```rust
extern crate ssd1325;
```

## License

Released under the MIT license. See `LICENSE` for full details.
