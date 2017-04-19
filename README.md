# Adafruit 2.7" Monochrome 128x64 OLED Driver

This is a platform-agnostic driver for the Solomon Systech SSD1325 OLED display
driver IC in monochrome mode. This chip is used in the 
[Adafruit 2.7" Monochrome 128x64 OLED Display Module](https://learn.adafruit.com/2-7-monochrome-128x64-oled-display-module).
This library is transport-agnostic, in that it has no intrinsic dependencies.
Supply an `io::Write` compatible object for transferring data and an
`ssd1325::ControlChannel` for controlling side-band pins such as
`D/NC` and `nRST`, and you're all set.

## Compatibility

Tested with the aforementioned module only. This should work with any SSD1325
display, however the initialization sequence may not be sufficient. Please
submit an issue if you run into issues and I'll attempt to make the
interface more generic to support your use case.

## License

Released under the MIT license. See `LICENSE` for full details.
