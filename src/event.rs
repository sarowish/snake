use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;

pub enum Event {
    Input(Key),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<Event>,
    _input_handle: thread::JoinHandle<()>,
    _tick_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Events {
        Events::default()
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}

impl Default for Events {
    fn default() -> Events {
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);
        let _input_handle = thread::spawn(move || {
            let stdin = io::stdin();
            for key in stdin.keys().flatten() {
                if let Err(err) = tx.send(Event::Input(key)) {
                    eprintln!("{}", err);
                    return;
                }
            }
        });
        let _tick_handle = thread::spawn(move || loop {
            if tx1.send(Event::Tick).is_err() {
                break;
            }
            thread::sleep(Duration::from_millis(105));
        });

        Events {
            rx,
            _input_handle,
            _tick_handle,
        }
    }
}
