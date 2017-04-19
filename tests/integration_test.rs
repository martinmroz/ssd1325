
extern crate ssd1325;

use std::cell::RefCell;
use std::error;
use std::io;
use std::rc::Rc;

#[derive(Debug,Clone,Eq,PartialEq)]
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
  SendData(usize),
}

struct MockControlChannel {
  /// Log for events occurring in the mock display.
  event_log: Rc<RefCell<Vec<Event>>>
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
  event_log: Rc<RefCell<Vec<Event>>>
}

impl io::Write for MockDataChannel {
  fn write(&mut self, data: &[u8]) -> Result<usize, io::Error> {
    self.event_log.borrow_mut().push(Event::SendData(data.len()));
    Ok(data.len())
  }
  fn flush(&mut self) -> Result<(), io::Error> {
    Ok(())
  }
}

/// Returns a mock control and data channel, and the event log shared between them for validation.
fn create_test_setup() -> (MockControlChannel, MockDataChannel, Rc<RefCell<Vec<Event>>>) {
  let log = Rc::new(RefCell::new(Vec::<Event>::new()));
  let control_channel = MockControlChannel { event_log: log.clone() };
  let data_channel = MockDataChannel { event_log: log.clone() };
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
  assert_eq!(event_log_iter.next().unwrap(), &Event::SendData(40));
  assert_eq!(event_log_iter.next().unwrap(), &Event::ControlChannelEnterIdle);
}
