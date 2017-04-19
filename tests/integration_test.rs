
extern crate ssd1325;

use std::cell::RefCell;
use std::error;
use std::io;
use std::rc::Rc;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Event {
  /// The control channel entered Reset mode.
  ControlChannelEnterReset,
  /// The control channel entered Idle mode.
  ControlChannelEnterIdle,
  /// The control channel entered Data mode.
  ControlChannelEnterData,
  /// The control channel entered Command mode.
  ControlChannelEnterCommand,
  /// Data was written.
  SendData,
}

struct MockControlChannel {
  /// Log for events occurring in the mock display.
  event_log: Rc<RefCell<Vec<Event>>>,
  /// Simulate a control channel error.
  pub sim_error: bool,
}

impl ssd1325::ControlChannel for MockControlChannel {
  fn run_in_mode(&mut self, mode: ssd1325::DisplayMode, f: &mut FnMut() -> Result<(),Box<error::Error>>) -> Result<(),Box<error::Error>> {
    {
      // Log the channel entering the specified mode.
      let mut log = self.event_log.borrow_mut();
      match mode {
        ssd1325::DisplayMode::Idle =>
          log.push(Event::ControlChannelEnterIdle),
        ssd1325::DisplayMode::Reset =>
          log.push(Event::ControlChannelEnterReset),
        ssd1325::DisplayMode::Data =>
          log.push(Event::ControlChannelEnterData),
        ssd1325::DisplayMode::Command =>
          log.push(Event::ControlChannelEnterCommand),
      }
    }

    if self.sim_error {
      return Err(Box::new(io::Error::new(io::ErrorKind::Other, "oh no!")));
    }

    // Invoke the requested function.
    f()?;

    {
      // Log the channel entering idle mode.
      self.event_log.borrow_mut().push(Event::ControlChannelEnterIdle);
    }

    Ok(())
  }
}

struct MockDataChannel {
  /// Log for events occurring in the mock display.
  event_log: Rc<RefCell<Vec<Event>>>,
  /// Simulate a short write on subsequent writes.
  pub sim_write_zero: bool,
  /// Simulate a write error on subsequent writes.
  pub sim_write_error: bool,
}

impl io::Write for MockDataChannel {
  fn write(&mut self, data: &[u8]) -> Result<usize, io::Error> {
    if self.sim_write_zero {
      Ok(0)
    } else if self.sim_write_error {
      Err(io::Error::new(io::ErrorKind::Other, "oh no!"))
    } else {
      self.event_log.borrow_mut().push(Event::SendData);
      Ok(data.len())
    }
  }
  fn flush(&mut self) -> Result<(), io::Error> {
    Ok(())
  }
}

/// Returns a mock control and data channel, and the event log shared between them for validation.
fn create_test_setup() -> (MockControlChannel, MockDataChannel, Rc<RefCell<Vec<Event>>>) {
  let log = Rc::new(RefCell::new(Vec::<Event>::new()));
  let control_channel = MockControlChannel { event_log: log.clone(), sim_error: false };
  let data_channel = MockDataChannel { event_log: log.clone(), sim_write_zero: false, sim_write_error: false };
  (control_channel, data_channel, log)
}

#[test]
fn test_init() {
  let (ref mut control, ref mut data, ref log) = create_test_setup();
  let mut display = ssd1325::Ssd1325::new(data, control);

  // Perform the initialization sequence.
  display.init().unwrap();
  
  // Expected initialization flow:
  //  - Enter Reset.
  //  - Enter Idle.
  //  - Enter Command.
  //  - Send Data (Initialization Sequence).
  //  - Enter Idle.
  let event_log = log.borrow_mut();
  assert_eq!(event_log.len(), 5);

  let mut event_log_iter = event_log.iter();
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterReset);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterIdle);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterCommand);
  assert_eq!(event_log_iter.next().unwrap(), &Event::SendData);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterIdle);
}

#[test]
fn test_clear() {
  let (ref mut control, ref mut data, ref log) = create_test_setup();
  let mut display = ssd1325::Ssd1325::new(data, control);

  // Perform the clear sequence.
  display.clear().unwrap();
  
  // Expected initialization flow:
  //  - Enter Command.
  //  - Send Data (Clear Sequence).
  //  - Enter Idle.
  let event_log = log.borrow_mut();
  assert_eq!(event_log.len(), 3);

  let mut event_log_iter = event_log.iter();
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterCommand);
  assert_eq!(event_log_iter.next().unwrap(), &Event::SendData);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterIdle);
}

#[test]
fn test_set_on_off() {
  let (ref mut control, ref mut data, ref log) = create_test_setup();
  let mut display = ssd1325::Ssd1325::new(data, control);

  // Perform the on/off sequence.
  display.set_on(true).unwrap();
  display.set_on(false).unwrap();
  
  // Expected display on/off flow:
  //  - Enter Command.
  //  - Send Data (On).
  //  - Enter Idle.
  //  - Enter Command.
  //  - Send Data (Off).
  //  - Enter Idle.
  let event_log = log.borrow_mut();
  assert_eq!(event_log.len(), 6);

  let mut event_log_iter = event_log.iter();
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterCommand);
  assert_eq!(event_log_iter.next().unwrap(), &Event::SendData);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterIdle);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterCommand);
  assert_eq!(event_log_iter.next().unwrap(), &Event::SendData);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterIdle);
}

#[test]
fn test_set_inverted_normal() {
  let (ref mut control, ref mut data, ref log) = create_test_setup();
  let mut display = ssd1325::Ssd1325::new(data, control);

  // Perform the invert/normal sequence.
  display.set_inverted(true).unwrap();
  display.set_inverted(false).unwrap();
  
  // Expected invert on/off flow:
  //  - Enter Command.
  //  - Send Data (On).
  //  - Enter Idle.
  //  - Enter Command.
  //  - Send Data (Off).
  //  - Enter Idle.
  let event_log = log.borrow_mut();
  assert_eq!(event_log.len(), 6);

  let mut event_log_iter = event_log.iter();
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterCommand);
  assert_eq!(event_log_iter.next().unwrap(), &Event::SendData);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterIdle);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterCommand);
  assert_eq!(event_log_iter.next().unwrap(), &Event::SendData);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterIdle);
}

#[test]
fn test_blit_l1() {
  let (ref mut control, ref mut data, ref log) = create_test_setup();
  let mut display = ssd1325::Ssd1325::new(data, control);

  // Build an all-on image test sequence to blit.
  let test_sequence = &[[0xFFu8; 16]; 64];

  // Blit the image to the screen.
  display.blit_l1(test_sequence).unwrap();

  // Expected initialization flow:
  //  - Enter Command.
  //  - Send Data (6).
  //  - Enter Idle.
  // [ 64x
  //    - Enter Data.
  //    - Send Data (64).
  //    - Enter Idle.
  // ]
  let event_log = log.borrow_mut();
  assert_eq!(event_log.len(), 3 + (64 * 3));

  // Check the blit preamble was sent.
  let mut event_log_iter = event_log.iter();
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterCommand);
  assert_eq!(event_log_iter.next().unwrap(), &Event::SendData);
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterIdle);

  // Check all 64 lines were sent.
  for _ in 0 .. 64 {
    assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterData);
    assert_eq!(event_log_iter.next().unwrap(), &Event::SendData);
    assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterIdle);
  }
}

#[test]
fn test_simulate_write_zero_length() {
  let (ref mut control, ref mut data, _) = create_test_setup();
  data.sim_write_zero = true;

  let mut display = ssd1325::Ssd1325::new(data, control);

  // Invoke something that would yield a write over the data channel.
  // The transport will indicate that it wrote zero bytes, which should yield an error.
  assert_eq!(display.set_on(true).is_err(), true);
}

#[test]
fn test_simulate_write_error() {
  let (ref mut control, ref mut data, _) = create_test_setup();
  data.sim_write_error = true;

  let mut display = ssd1325::Ssd1325::new(data, control);

  // Invoke something that would yield a write over the data channel.
  // The transport will indicate that the write failed, which should yield an error.
  assert_eq!(display.set_on(true).is_err(), true);
}

#[test]
fn test_simulate_control_error() {
  let (ref mut control, ref mut data, _) = create_test_setup();
  control.sim_error = true;

  let mut display = ssd1325::Ssd1325::new(data, control);

  // Invoke something that would yield a control event.
  // The control channel will indicate a failure occurred, which should yield an error.
  assert_eq!(display.set_on(true).is_err(), true);
}
