use clap::ArgMatches;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;

pub struct Options {
    width: i32,
    height: i32,
    head_x: i32,
    head_y: i32,
    pub speed: f64,
    length: i32,
    direction: Direction,
    borders: bool,
}

impl From<ArgMatches> for Options {
    fn from(matches: ArgMatches) -> Self {
        let direction = Direction::from(matches.get_one::<String>("direction").unwrap());

        Options {
            width: *matches.get_one::<i32>("width").unwrap(),
            height: *matches.get_one::<i32>("height").unwrap(),
            head_x: *matches.get_one::<i32>("head_x").unwrap(),
            head_y: *matches.get_one::<i32>("head_y").unwrap(),
            speed: *matches.get_one::<f64>("speed").unwrap(),
            length: *matches.get_one::<i32>("length").unwrap(),
            direction,
            borders: !matches.get_flag("no_border"),
        }
    }
}

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

impl From<&String> for Direction {
    fn from(direction: &String) -> Self {
        match direction.as_str() {
            "left" => Direction::Left,
            "right" => Direction::Right,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!(),
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

impl Game {
    pub fn new(options: &Options) -> Self {
        assert!(
            Game::validate_initial_state(options),
            "initial state of the snake is invalid"
        );

        let mut snake = VecDeque::new();

        for i in (0..options.length).rev() {
            match options.direction {
                Direction::Up => snake.push_back(Point::new(options.head_x, options.head_y + i)),
                Direction::Down => snake.push_back(Point::new(options.head_x, options.head_y - i)),
                Direction::Left => snake.push_back(Point::new(options.head_x + i, options.head_y)),
                Direction::Right => snake.push_back(Point::new(options.head_x - i, options.head_y)),
            }
        }

        let mut rng = thread_rng();
        let apple_x = rng.gen_range(0..options.width);
        let apple_y = rng.gen_range(0..options.height);
        let apple = Point::new(apple_x, apple_y);
        Game {
            snake,
            dir: options.direction.clone(),
            board: (options.width, options.height),
            borders: options.borders,
            apple,
            state: State::Running,
        }
    }

    fn validate_initial_state(options: &Options) -> bool {
        if options.head_x < 0
            || options.head_y < 0
            || options.head_x >= options.width
            || options.head_y >= options.height
            || options.length <= 0
        {
            return false;
        }

        match options.direction {
            Direction::Up => options.head_y + options.length <= options.height,
            Direction::Down => options.head_y - options.length + 1 >= 0,
            Direction::Left => options.head_x + options.length <= options.width,
            Direction::Right => options.head_x - options.length + 1 >= 0,
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
        self.snake.push_back(new_head);
        if !self.check_apple() {
            self.snake.pop_front();
        }
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
