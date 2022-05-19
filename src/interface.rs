use crate::event::{Event, Events};
use crate::game::Direction;
use crate::game::Game;
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

pub fn run_ui() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut game: Game = Default::default();
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
            let chunks = vec![
                Rect {
                    x: 0,
                    y: 0,
                    width: (game.board.0 * 2 + 2) as u16,
                    height: (game.board.1 + 2) as u16,
                },
                Rect {
                    x: 0,
                    y: (game.board.1 + 2) as u16,
                    width: 50,
                    height: 1,
                },
            ];
            let mut text = Text::default();
            for line in grid {
                text.extend(Text::from(Spans::from(line)));
            }

            let text = Paragraph::new(text).block(
                Block::default()
                    .border_style(Style::default().fg(if game.is_game_over() {
                        Color::Red
                    } else {
                        Color::Green
                    }))
                    .borders(Borders::ALL)
                    .title("Snake"),
            );
            f.render_widget(text, chunks[0]);
            if game.is_game_over() {
                let text = Spans::from(vec![
                    Span::raw("Game is over. press "),
                    Span::styled("q", Style::default().fg(Color::Red)),
                    Span::raw(" to quit, "),
                    Span::raw("press "),
                    Span::styled("r", Style::default().fg(Color::Yellow)),
                    Span::raw(" to replay"),
                ]);
                f.render_widget(Paragraph::new(text), chunks[1]);
            } else if !game.is_running() {
                let text = Span::raw("Paused");
                f.render_widget(Paragraph::new(text), chunks[1]);
            }
        })?;

        match events.next()? {
            Event::Input(Key::Char(c)) => match c {
                'q' => break,
                'h' => dir = Direction::Left,
                'j' => dir = Direction::Down,
                'k' => dir = Direction::Up,
                'l' => dir = Direction::Right,
                'r' => {
                    game = Default::default();
                    dir = game.dir.clone();
                    continue;
                }
                'p' => game.toggle_pause(),
                _ => {}
            },
            Event::Tick if game.is_running() => game.move_snake(dir.clone()),
            _ => {}
        }
    }
    Ok(())
}
