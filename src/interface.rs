use crate::event::{Event, Events};
use crate::game::{self, Direction, Game};
use crate::solver::Solver;
use std::{error::Error, io};
use termion::event::Key;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::layout::Rect;
use tui::style::Color;
use tui::style::Style;
use tui::text::{Span, Spans, Text};
use tui::widgets::Paragraph;
use tui::{
    backend::TermionBackend,
    widgets::{Block, Borders},
    Terminal,
};

pub fn run_ui(options: game::Options) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new(options.speed);

    let mut game: Game = Game::new(&options);
    let game_area = Solver::new(&game, None).game_area;

    let mut dir = game.dir.clone();
    let apple_char = "🍎";
    let snake_char = "██";

    loop {
        let mut grid = vec![vec![Span::raw("  "); game.board.0 as usize]; game.board.1 as usize];

        grid[game.apple.y as usize][game.apple.x as usize] =
            Span::styled(apple_char, Style::default().fg(Color::Red));

        grid[game.snake.back().unwrap().y as usize][game.snake.back().unwrap().x as usize] =
            Span::styled(snake_char, Style::default().fg(Color::Blue));

        for p in game.snake.iter().rev().skip(1) {
            grid[p.y as usize][p.x as usize] =
                Span::styled(snake_char, Style::default().fg(Color::Yellow));
        }

        terminal.draw(|f| {
            let board_width = game.board.0 * 2 + 2;
            let board_height = game.board.1 + 2;
            let mut x = 0;
            let mut y = 0;
            let mut enough_space = true;

            if let Some(res) = f.size().width.checked_sub(board_width as u16) {
                x = res / 2;
            } else {
                enough_space = false;
            }

            if let Some(res) = f.size().height.checked_sub(board_height as u16 + 1) {
                y = res / 2;
            } else {
                enough_space = false;
            }

            if !enough_space {
                if game.is_running() {
                    game.toggle_pause();
                }

                let text = Paragraph::new("Not enough screen space");
                f.render_widget(text, f.size());
                return;
            }

            let key_help = if game.is_game_over() {
                Spans::from(vec![
                    Span::raw("Game is over. press "),
                    Span::styled("q", Style::default().fg(Color::Red)),
                    Span::raw(" to quit, "),
                    Span::raw("press "),
                    Span::styled("r", Style::default().fg(Color::Yellow)),
                    Span::raw(" to replay"),
                ])
            } else if !game.is_running() {
                Spans::from(Span::raw("Paused"))
            } else {
                Spans::from(Span::raw("Controls: hjkl ←↓↑→, Quit: q, Pause: p"))
            };

            let chunks = vec![
                Rect {
                    x,
                    y,
                    width: (game.board.0 * 2 + 2) as u16,
                    height: (game.board.1 + 2) as u16,
                },
                Rect {
                    x,
                    y: y + (game.board.1 + 2) as u16,
                    width: (key_help.width() as u16).min(f.size().width - x),
                    height: 1,
                },
            ];
            let mut grid_text = Text::default();
            for line in grid {
                grid_text.extend(Text::from(Spans::from(line)));
            }

            let text = Paragraph::new(grid_text).block(
                Block::default()
                    .border_style(Style::default().fg(if game.is_game_over() {
                        Color::Red
                    } else if !game.is_running() {
                        Color::Yellow
                    } else {
                        Color::Green
                    }))
                    .borders(Borders::ALL)
                    .title("Snake"),
            );
            f.render_widget(text, chunks[0]);
            f.render_widget(Paragraph::new(key_help), chunks[1]);
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => break,
                Key::Char('h') | Key::Left => dir = Direction::Left,
                Key::Char('j') | Key::Down => dir = Direction::Down,
                Key::Char('k') | Key::Up => dir = Direction::Up,
                Key::Char('l') | Key::Right => dir = Direction::Right,
                Key::Char('r') => {
                    game = Game::new(&options);
                    dir = game.dir.clone();
                    continue;
                }
                Key::Char('p') => game.toggle_pause(),
                _ => {}
            },
            Event::Tick if game.is_running() => {
                if game.self_play {
                    let mut solver = Solver::new(&game, Some(game_area.clone()));
                    dir = solver.next_direction();
                }

                game.move_snake(dir.clone());
            }
            _ => {}
        }
    }
    Ok(())
}
