use tauri::Window;
use std::{thread, time::Duration, sync::{Mutex, Arc}};

#[derive(Debug, serde::Serialize, Clone)]
struct ClockPayload {
  start_time: u64,
  duration: u64,
  left_time: u64
}

pub fn start_clock(window: Window, start_time: u64, duration: u64) {
  let should_end = Arc::new(Mutex::new(false));
  let current_should_end = Arc::clone(&should_end);

  window.listen("clock:stopped", move |_| {
    let should_end = Arc::clone(&should_end);
    let mut current_should_end = should_end.lock().unwrap();

    *current_should_end = true;
  });

  thread::spawn(move || {
    // duration is in millisecond
    let total_milliseconds = duration;
    let mut current_milliseconds = 0;

    while current_milliseconds <= total_milliseconds {
      if *current_should_end.lock().unwrap() {
        window.emit("clock:stopped", start_time.to_string()).unwrap();
        return;
      }

      let left_time = total_milliseconds - current_milliseconds;

      let payload = ClockPayload {
        start_time,
        duration,
        left_time
      };

      window.emit("clock:run", payload).unwrap();

      current_milliseconds = current_milliseconds + 1000;
      thread::sleep(Duration::from_secs(1));
    }

    window.emit("clock:stopped", start_time.to_string()).unwrap();
  });
}