use rand::{thread_rng, Rng};
use std::collections::VecDeque;

#[derive(PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite_dir(dir: &Direction) -> Direction {
        match dir {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

pub struct Game {
    pub snake: VecDeque<Point>,
    pub dir: Direction,
    pub board: (i32, i32),
    borders: bool,
    pub apple: Point,
    state: State,
}

impl Default for Game {
    fn default() -> Self {
        Game::new(5, 5, Direction::Right, 30, 20, true)
    }
}

impl Game {
    pub fn new(x: i32, y: i32, dir: Direction, width: i32, height: i32, borders: bool) -> Self {
        const STARTING_SIZE: i32 = 3;

        assert!(
            x - STARTING_SIZE < width && y < height,
            "x and y should be less than size\n"
        );
        let mut snake = VecDeque::new();
        for i in (0..STARTING_SIZE).rev() {
        snake.push_back(Point::new(x - i, y));
        }
        let mut rng = thread_rng();
        let apple_x = rng.gen_range(0..width);
        let apple_y = rng.gen_range(0..height);
        let apple = Point::new(apple_x, apple_y);
        Game {
            snake,
            dir,
            board: (width, height),
            borders,
            apple,
            state: State::Running,
        }
    }

    pub fn move_snake(&mut self, mut dir: Direction) {
        let mut new_head = Point::new(self.snake.back().unwrap().x, self.snake.back().unwrap().y);
        if self.dir == Direction::opposite_dir(&dir) {
            dir = self.dir.clone();
        }
        match dir {
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
        }
        if !self.check_border(&mut new_head) {
            return;
        }
        self.check_overlap(&new_head);
        if !self.check_apple() {
            self.snake.pop_front();
        }
        self.snake.push_back(new_head);
        self.dir = dir;
    }

    fn check_overlap(&mut self, new_head: &Point) {
        for p in &self.snake {
            if new_head == p {
                return self.game_over();
            }
        }
    }

    fn check_border(&mut self, new_head: &mut Point) -> bool {
        if self.borders
            && (new_head.x < 0
                || new_head.x >= self.board.0
                || new_head.y < 0
                || new_head.y >= self.board.1)
        {
            self.game_over();
            false
        } else {
            new_head.x = new_head.x.rem_euclid(self.board.0);
            new_head.y = new_head.y.rem_euclid(self.board.1);
            true
        }
    }

    fn check_apple(&mut self) -> bool {
        if self.snake.back().unwrap() == &self.apple {
            let mut app = self.gen_apple();
            while self.snake.contains(&app) {
                app = self.gen_apple();
            }
            self.apple = app;
            true
        } else {
            false
        }
    }

    fn gen_apple(&self) -> Point {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..self.board.0);
        let y = rng.gen_range(0..self.board.1);
        Point::new(x, y)
    }

    fn game_over(&mut self) {
        self.state = State::GameOver;
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, State::Running)
    }

    pub fn is_game_over(&self) -> bool {
        matches!(self.state, State::GameOver)
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            State::Paused => self.state = State::Running,
            State::Running => self.state = State::Paused,
            _ => (),
        }
    }
}

#[derive(PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

enum State {
    Running,
    Paused,
    GameOver,
}
