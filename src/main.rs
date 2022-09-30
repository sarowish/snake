mod event;
mod game;
mod interface;

use clap::{Arg, ArgAction, Command};
use interface::run_ui;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches: clap::ArgMatches = Command::new(env!("CARGO_PKG_NAME"))
        .arg(
            Arg::new("width")
                .long("width")
                .help("Width of the game area")
                .default_value("30")
                .value_name("SIZE")
                .value_parser(clap::value_parser!(i32)),
        )
        .arg(
            Arg::new("height")
                .long("height")
                .help("Height of the game area")
                .default_value("20")
                .value_name("SIZE")
                .value_parser(clap::value_parser!(i32)),
        )
        .arg(
            Arg::new("speed")
                .short('s')
                .long("speed")
                .help("Movement speed of the snake")
                .default_value("10")
                .value_name("SPEED")
                .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            Arg::new("head_x")
                .short('x')
                .long("head-x")
                .help("Initial x coordinate of the snake's head")
                .default_value("3")
                .value_name("COORD")
                .value_parser(clap::value_parser!(i32)),
        )
        .arg(
            Arg::new("head_y")
                .short('y')
                .long("head-y")
                .help("Initial y coordinate of the snake's head")
                .default_value("3")
                .value_name("COORD")
                .value_parser(clap::value_parser!(i32)),
        )
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .help("Initial length of the snake")
                .default_value("3")
                .value_name("LENGTH")
                .value_parser(clap::value_parser!(i32)),
        )
        .arg(
            Arg::new("direction")
                .short('d')
                .long("dir")
                .help("Initial direction of the snake")
                .default_value("right")
                .value_name("DIRECTION")
                .value_parser(["left", "right", "up", "down"]),
        )
        .arg(
            Arg::new("no_border")
                .long("no-border")
                .help("Disable borders")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    run_ui(game::Options::from(matches))
}
