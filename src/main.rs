mod interface;
mod event;
mod game;

use std::error::Error;
use interface::run_ui;

fn main() -> Result<(), Box<dyn Error>> {
    run_ui()
}
