
use std::{error, fmt, io, thread, time};

mod commands {
  /// 10.1.1 Set Column Address
  /// Note: Each pixel takes 4 bits in display memory so `(end-start)` should be `h_pixels/2`.
  ///
  /// # Arguments
  /// * start: `u8` Column start address of display data RAM.
  /// * end: `u8` Column end address of display data RAM.
  pub const SETCOLADDR: u8 = 0x15;
  /// 10.1.2 Set Row Address
  ///
  /// # Arguments
  /// * start: `u8` Row start address of display data in RAM.
  /// * end: `u8` Row end address of display data in RAM.
  pub const SETROWADDR: u8 = 0x75;
  /// 10.1.3 Set Contrast Current
  ///
  /// # Arguments
  /// * contrast: `u8` Contrast current from `0` to `0x7F`.
  pub const SETCONTRAST: u8 = 0x81;
  /// 10.1.4 Set Current Range to Full.
  pub const SETCURRENT_FULL: u8 = 0x84 + 0x03;
  /// 10.1.5 Set Re-Map.
  ///
  /// # Arguments
  /// * One byte bitmask as follows.
  ///   * `7-------` Unused. Set to `0`.
  ///   * `-6------` COM Split Odd/Even. When `0`, no COM splitting is performed.
  ///   * `--*-----` Reserved. Set to `0`.
  ///   * `---4----` COM Remapping up to down when `0`, down to up when `1`.
  ///   * `----*---` Reserved. Set to `0`.
  ///   * `-----2--` Address increment mode. When `0`, moves left. when `1` moves down.
  ///   * `------1-` Nibble remapping, when `0` bytes are read `0b_7654_3210`, when `1`, `0b_3210_7654`.
  ///   * `-------0` Column Address Remap: Segment left-to-right when `0` and right-to-left when `1`.
  pub const SETREMAP: u8 = 0xA0;
  /// 10.1.6 Set Display Start Line
  ///
  /// # Arguments
  /// * start_line: `u8` Start line from 0 to 80, used for vertical scrolling.
  pub const SETSTARTLINE: u8 = 0xA1;
  /// 10.1.7 Set Display Offset
  ///
  /// # Arguments
  /// * offset: `u8` Display offset from 0 to 80, used for vertical scrolling.
  pub const SETOFFSET: u8 = 0xA2;
  /// 10.1.8.1 Enter Normal Display Mode
  pub const NORMALDISPLAY: u8 = 0xA4;
  /// 10.1.8.4 Enter Inverse Display Mode
  pub const INVERTDISPLAY: u8 = 0xA7;
  /// 10.1.9 Set Multiplex Ratio
  ///
  /// # Arguments
  /// * ratio: `u8` Set screen multiplex ratio from 16MUX to 80MUX.
  pub const SETMULTIPLEX: u8 = 0xA8;
  /// 10.1.10 Set Master Configuration
  ///
  /// Selects the external Vcc power supply.
  /// This command will be activated after issuing Set Display On (`0xAF`).
  pub const MASTERCONFIG: u8 = 0xAD;
  /// 10.1.11.1 Set Display Off
  pub const DISPLAYOFF: u8 = 0xAE;
  /// 10.1.11.1 Set Display On
  pub const DISPLAYON: u8 = 0xAF;
  /// Table 18: Set Pre-charge Compensation Enable
  ///
  /// # Arguments
  /// * One byte bitmask as follows.
  ///   * `--543210` `0x08` on reset, `0x28` to enable compensation.
  pub const SETPRECHARGECOMPENABLE: u8 = 0xB0;
  /// 10.1.14 Set Phase Length
  ///
  /// # Arguments
  /// * One byte bitmask as follows.
  ///   * `7654----` Phase length of precharge (Phase 2).
  ///   * `----3210` Phase length of reset (Phase 1).
  pub const SETPHASELEN: u8 = 0xB1;
  /// 10.1.15 Set Row Period
  ///
  /// # Arguments
  /// * period: `u8` Value between `0x14` and `0x7F`. Defines the frame rate,
  ///   where lower values yield higher frame rates and less defined grays.
  pub const SETROWPERIOD: u8 = 0xB2;
  /// 10.1.16 Set Display Clock Divide Ratio
  ///
  /// # Arguments
  /// * One byte bitmask as follows.
  ///   * `7654----` Oscillator frequency. Increases with value.
  ///   * `----3210` Divide ratio = (Value + 1). 
  pub const SETCLOCK: u8 = 0xB3;
  /// Table 18: Set Pre-charge
  ///
  /// # Arguments
  /// * One byte bitmask as follows.
  ///   * `-----210` `0x00` on reset, `0x03` recommended level.
  pub const SETPRECHARGECOMP: u8 = 0xB4;
  /// Table 18: Set Gray Scale Table.
  ///
  /// # Arguments
  /// * Table: [u8; 8] as defined in the Table 18.
  pub const SETGRAYTABLE: u8 = 0xB8;
  /// 10.1.12 Set Vcomh Voltage
  ///
  /// # Arguments
  /// * One byte value as defined in Table 18.
  pub const SETVCOMLEVEL: u8 = 0xBE;
  /// Table 18: Set Segment Low Voltage
  ///
  /// # Arguments
  /// * One byte value as defined in Table 18.
  ///   * `0x02` = Keep VSL pin NC.
  ///   * `0x0E` = Default. Connect a capacitor between Vsl and Vss.
  pub const SETVSL: u8 = 0xBF;
  /// Table 18: Graphic Acceleration Command Options
  ///
  /// # Arguments
  /// * One byte bitmask as follows.
  ///   * `***-----` Unused. Set to `0`.`.
  ///   * `---4----` To enable reverse during copying, set to `1`.
  ///   * `----**--` Unused. Set to `0`.
  ///   * `------1-` To enable wrap-around in X-direction on copy, set to `1`.
  ///   * `-------0` To enable fill rectangle on draw, set to `1`.
  pub const GFXACCEL: u8 = 0x23;
  /// 10.2.2 Draw Rectangle
  ///
  /// # Arguments
  ///   * start_col: `u8` Starting column coordinates.
  ///   * start_row: `u8` Starting row coordinates.
  ///   * end_col: `u8` Ending column coordinates.
  ///   * end_row: `u8` Ending row coordinates.
  ///   * pattern: `u8` Grayscale pattern to fill with.
  pub const DRAWRECT: u8 = 0x24;
}

/// Errors which may occur interacting with the display.
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum DisplayError {
  /// It was not possible to send all the necessary data to the display.
  WriteFailed,
}

impl fmt::Display for DisplayError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", (self as &error::Error).description())
  }
}

impl error::Error for DisplayError {
  fn description(&self) -> &str {
    match self {
      &DisplayError::WriteFailed =>
        "write failed: unable to send complete sequence to display",
    }
  }
}

/// Mode of the primary communication channel.
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum DisplayMode {
  /// Interface idle. Not requested directly, but should be used when idle.
  Idle,
  /// Display reset mode.
  Reset,
  /// Display idle mode. Cannot be in Reset, but can be either Data or Command.
  Data,
  /// Display command transport mode.
  Command,
}

/// Responsible for placing the display in a given mode prior to executing a command.
pub trait ControlChannel {
  /// Put the display communication channel in the specified `mode`.
  /// Once the command is executed the display must be left in a state other than `Reset`.
  fn run_in_mode(&mut self, mode: DisplayMode, f: &mut FnMut() -> Result<(),Box<error::Error>>) -> Result<(),Box<error::Error>>;
}

/// An SSD1325 display interface command adapter.
pub struct Ssd1325<'a> {
  /// Transport for sending data to the display.
  transport: &'a mut io::Write,
  /// Transport for side-band control data.
  control_channel: &'a mut ControlChannel,
}

impl<'a> Ssd1325<'a> {

  /// Returns a new instance of the receiver.
  /// The `transport` instance is used to send data to the display, typically over SPI although
  /// the MCU interface can be used if a suitable adapter is provided.
  /// The `control_channel` is used to put the display into a given mode before writing data.
  /// Typically, this is done using sysfs gpio.
  /// The display must be initialized prior to use, and is left Off.
  pub fn new(transport: &'a mut io::Write, control_channel: &'a mut ControlChannel) -> Self {
    Ssd1325 {
      transport: transport,
      control_channel: control_channel,
    }
  }

  /// Resets and initializes the display. Blocks for approximately 600ms.
  pub fn init(&mut self) -> Result<(),Box<error::Error>> {
    use commands::*;

    // The initialization sequence.
    const INIT_SEQUENCE: &'static [u8] = &[
      // Turn the display off.
      DISPLAYOFF,
      // Set the oscillator division.
      SETCLOCK, 0xF1,
      // Set the multiplex ratio to 1/64 duty cycle.
      SETMULTIPLEX, 0x3F,
      // Set the display offset to 76.
      SETOFFSET, 0x4C,
      // Set the start line to 0.
      SETSTARTLINE, 0x00,
      // Set Master Config to DC/DC Converter.
      MASTERCONFIG, 0x02,
      // Set segment remap to 0x50: *1010000 (COM split, remap bottom-up, horiz. increment, no nibble remap).
      SETREMAP, 0x50,
      // Set full current range.
      SETCURRENT_FULL,
      // Set the gray color palette.
      SETGRAYTABLE, 0x01, 0x11, 0x22, 0x32, 0x43, 0x54, 0x65, 0x76,
      // Set the contrast to maximum.
      SETCONTRAST, 0x7F,
      // Set the row period.
      SETROWPERIOD, 0x51,
      // Set the phase length.
      SETPHASELEN, 0x55,
      // Set the precharge comparator to 2.
      SETPRECHARGECOMP, 0x02,
      // Enable the precharge comparator.
      SETPRECHARGECOMPENABLE, 0x28,
      // Set the high voltage level of the COM pin (`0x1C = 0.80 * Vref`).
      SETVCOMLEVEL, 0x1C,
      // Set the low voltage level of the SEG pin. Value may be wrong.
      SETVSL, (0x0D | 0x02),
      // Set the display to non-inverted configuration.
      NORMALDISPLAY,
      // Turn on the Draw Rect command only, used to clear the screen.
      GFXACCEL, 0x01,
    ];

    // Reset the display.
    self.reset()?;

    // Send the initialization sequence in command mode to the display.
    self.write_sequence(DisplayMode::Command, INIT_SEQUENCE)
  }

  /// Clears the display.
  pub fn clear(&mut self) -> Result<(),Box<error::Error>> {
    use commands::*;

    // Clear sequence utilizing graphics acceleration.
    const CLEAR_SEQUENCE: &'static [u8] = &[
      // Clear the display.
      DRAWRECT, 0x00, 0x00, 0x3F, 0x3F, 0x00,
    ];

    // Send the clear sequence in command mode to the display.
    self.write_sequence(DisplayMode::Command, CLEAR_SEQUENCE)
  }

  /// Turn the display on or off. Configured to Off after initialization.
  pub fn set_on(&mut self, on: bool) -> Result<(),Box<error::Error>> {
    match on {
      true =>
        self.write_sequence(DisplayMode::Command, &[commands::DISPLAYON]),
      false =>
        self.write_sequence(DisplayMode::Command, &[commands::DISPLAYOFF]),
    }
  }

  /// Make the display inverted or normal. Configured to Normal after initialization.
  pub fn set_inverted(&mut self, inverted: bool) -> Result<(),Box<error::Error>> {
    match inverted {
      true =>
        self.write_sequence(DisplayMode::Command, &[commands::INVERTDISPLAY]),
      false =>
        self.write_sequence(DisplayMode::Command, &[commands::NORMALDISPLAY]),
    }
  }

  /// Send an entire bitmap frame to the display.
  /// The input image must be a 1-bit bitmap image arranged as 64 rows of 128 pixels.
  /// Pixels must be packed 8 per byte, with the most significant bit corresponding to
  /// the first pixel in the group (i.e. `0b1234567`).
  pub fn blit_l1(&mut self, frame: &[[u8; 16]; 64]) -> Result<(),Box<error::Error>> {
    use commands::*;

    // Clear sequence utilizing graphics acceleration.
    const BLIT_PREAMBLE_SEQUENCE: &'static [u8] = &[
      // Set the column address range to 0x00...0x3F. Each pixel takes 4 bits.
      SETCOLADDR, 0x00, 0x3F,
      // Set the row address range to 0x00...0x3F. There are 64 rows.
      SETROWADDR, 0x00, 0x3F,
    ];

    // Write the blit preamble sequence to the display.
    self.write_sequence(DisplayMode::Command, BLIT_PREAMBLE_SEQUENCE)?;

    // Unpack each line of display data and send it over the transport in Data mode.
    let mut sequence = [0u8; 64];
    for line in frame.iter() {
      unpack_line_for_display(line, &mut sequence);
      self.write_sequence(DisplayMode::Data, &sequence)?;
    }

    Ok(())
  }

  /// Resets the display and waits for it to restart. Takes approximately ~550ms.
  ///
  /// # Returns
  /// An error from the control channel if the display could not enter Reset mode.
  fn reset(&mut self) -> Result<(),Box<error::Error>> {
    self.control_channel.run_in_mode(DisplayMode::Reset, &mut move || {
      thread::sleep(time::Duration::from_millis(10));
      Ok(())
    })?;

    // Allow the display to restart for 500ms while holding the interface implicitly idle.
    thread::sleep(time::Duration::from_millis(500));
    Ok(())
  }

  /// Send a sequence of `bytes` to the display in `mode`.
  ///
  /// # Returns
  /// A local error if not all data could be sent.
  fn write_sequence(&mut self, mode: DisplayMode, bytes: &[u8]) -> Result<(),Box<error::Error>> {
    let mut transport = &mut self.transport;

    // Send the sequence to the display over the transport once the control channel is configured.
    self.control_channel.run_in_mode(mode, &mut move || {
      let sent = transport.write(bytes).map_err(|e| Box::new(e))?;
      if sent < bytes.len() {
        Err(Box::new(DisplayError::WriteFailed))
      } else {
        Ok(())
      }
    })
  }

}

/// Converts a `0bABCDEFGH` packed monochrome binary bitmap byte sequence into
/// a group of 4 display-pixels of the form `[0bAAAABBBB, 0bCCCCDDDD, 0bEEEEFFFF, 0bGGGGHHHH]`.
/// The `unpacked` slice must be at least 4 bytes long.
fn unpack_pixels_for_display(packed: u8, unpacked: &mut [u8]) {
  let mut pixel_group = packed;
  for i in 0..4 {
    let l = if (pixel_group & 0x80) != 0 { 0xF0 } else { 0x00 };
    let r = if (pixel_group & 0x40) != 0 { 0x0F } else { 0x00 };
    unpacked[i] = l | r;
    pixel_group <<= 2;
  }
}

/// Unpacks an entire line of pixels for display.
fn unpack_line_for_display(line: &[u8; 16], unpacked: &mut [u8; 64]) {
  for (index, pixel) in line.iter().enumerate() {
    let range_start = index * 4;
    let range_end = range_start + 4;
    unpack_pixels_for_display(*pixel, &mut unpacked[range_start .. range_end]);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]  
  fn test_unpack_pixels() {
    let mut buffer = [0u8; 4];
    unpack_pixels_for_display(0b00000000, &mut buffer);
    assert_eq!(buffer, [0x00, 0x00, 0x00, 0x00]);
    unpack_pixels_for_display(0b00001111, &mut buffer);
    assert_eq!(buffer, [0x00, 0x00, 0xFF, 0xFF]);
    unpack_pixels_for_display(0b11110000, &mut buffer);
    assert_eq!(buffer, [0xFF, 0xFF, 0x00, 0x00]);
    unpack_pixels_for_display(0b10100110, &mut buffer);
    assert_eq!(buffer, [0xF0, 0xF0, 0x0F, 0xF0]);
    unpack_pixels_for_display(0b01100101, &mut buffer);
    assert_eq!(buffer, [0x0F, 0xF0, 0x0F, 0x0F]);
  }

  #[test]  
  fn test_unpack_line() {
    let test_line: [u8; 16] = [
      0b00000000, 0b00001111, 0b11110000, 0b10100110,
      0b00000000, 0b00001111, 0b11110000, 0b10100110,
      0b00000000, 0b00001111, 0b11110000, 0b10100110,
      0b00000000, 0b00001111, 0b11110000, 0b10100110,
    ];
    let mut result = [0u8; 64];
    unpack_line_for_display(&test_line, &mut result);
    for i in 0..4 {
      let start = i * 16;
      let sub_line = &result[start..start+16];
      assert_eq!(&sub_line[ 0.. 4], [0x00, 0x00, 0x00, 0x00]);
      assert_eq!(&sub_line[ 4.. 8], [0x00, 0x00, 0xFF, 0xFF]);
      assert_eq!(&sub_line[ 8..12], [0xFF, 0xFF, 0x00, 0x00]);
      assert_eq!(&sub_line[12..16], [0xF0, 0xF0, 0x0F, 0xF0]);
    }
  }

}
