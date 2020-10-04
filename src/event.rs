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

pub struct Config {
    _exit_key: Key,
    tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            _exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(125),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);
        let _input_handle = thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                if let Ok(key) = evt {
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
            thread::sleep(config.tick_rate);
        });

        Events {
            rx,
            _input_handle,
            _tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
