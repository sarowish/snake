use crossterm::event::{Event as CEvent, KeyEvent};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum Event {
    Input(KeyEvent),
    Tick,
}

pub struct EventHandle {
    rx: mpsc::Receiver<Event>,
    _input_handle: thread::JoinHandle<()>,
    _tick_handle: thread::JoinHandle<()>,
}

impl EventHandle {
    pub fn new(speed: f64) -> EventHandle {
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);
        let _input_handle = thread::spawn(move || {
            while let Ok(event) = crossterm::event::read() {
                if let CEvent::Key(key) = event {
                    if let Err(err) = tx.send(Event::Input(key)) {
                        eprintln!("{}", err);
                        return;
                    }
                }
            }
        });
        let _tick_handle = thread::spawn(move || loop {
            if tx1.send(Event::Tick).is_err() {
                break;
            }

            thread::sleep(Duration::from_micros(f64::floor(1_000_000.0 / speed) as u64));
        });

        EventHandle {
            rx,
            _input_handle,
            _tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
