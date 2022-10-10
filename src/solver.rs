use crate::game::{Direction, Game, Point};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Clone, Eq, PartialEq)]
struct AStarCost {
    coord: Point,
    f_score: i32,
}

impl AStarCost {
    fn new(coord: Point, f_score: i32) -> Self {
        Self { coord, f_score }
    }
}

impl Ord for AStarCost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}

impl PartialOrd for AStarCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
pub struct Cell {
    parent: Option<Point>,
    visited: bool,
    distance: i32,
    circuit_idx: usize,
}

impl Cell {
    fn new() -> Self {
        Self {
            parent: None,
            visited: false,
            distance: i32::MAX,
            circuit_idx: 0,
        }
    }
}

#[derive(Clone)]
pub enum PathAlgorithm {
    AStar,
    Bfs,
}

impl From<&String> for PathAlgorithm {
    fn from(algorithm: &String) -> Self {
        match algorithm.as_str() {
            "astar" => PathAlgorithm::AStar,
            "bfs" => PathAlgorithm::Bfs,
            _ => panic!(),
        }
    }
}

pub struct Solver<'a> {
    pub game_area: Vec<Vec<Cell>>,
    game: &'a Game,
}

impl<'a> Solver<'a> {
    pub fn new(game: &'a Game, game_area: Option<Vec<Vec<Cell>>>) -> Self {
        let flag = game_area.is_none();
        let game_area = if let Some(mut game_area) = game_area {
            for row in game_area.iter_mut() {
                for mut column in row {
                    column.parent = None;
                    column.distance = i32::MAX;
                }
            }
            game_area
        } else {
            vec![vec![Cell::new(); game.board.0 as usize]; game.board.1 as usize]
        };

        let mut solver = Self { game_area, game };

        if flag {
            solver.build_cycle();
        }

        solver
    }

    fn get_cell(&self, coord: &Point) -> &Cell {
        &self.game_area[coord.y as usize][coord.x as usize]
    }

    fn get_mut_cell(&mut self, coord: &Point) -> &mut Cell {
        &mut self.game_area[coord.y as usize][coord.x as usize]
    }

    pub fn find_longest_path(&mut self, destination: &Point) -> Vec<Point> {
        let mut path = self.find_shortest_path(destination);
        if path.is_empty() {
            return path;
        }

        for point in &path {
            self.get_mut_cell(point).visited = true;
        }

        let mut idx = 0;

        loop {
            let current = path[idx].clone();
            let next = path[idx + 1].clone();

            let direction = current.direction_to(&next).unwrap();
            let directions_to_test = match direction {
                Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
            };

            let mut extended = false;
            for test_direction in directions_to_test {
                let current_test = current.adjacent_point(&test_direction);
                let next_test = next.adjacent_point(&test_direction);

                if self.validate_point(&current_test) && self.validate_point(&next_test) {
                    self.get_mut_cell(&current_test).visited = true;
                    self.get_mut_cell(&next_test).visited = true;
                    path.insert(idx + 1, current_test);
                    path.insert(idx + 2, next_test);
                    extended = true;
                    break;
                }
            }

            if next == *destination {
                break;
            }

            if !extended {
                idx += 1;
            }
        }

        path
    }

    pub fn find_shortest_path(&mut self, destination: &Point) -> Vec<Point> {
        match self.game.path_alg {
            PathAlgorithm::AStar => self.find_shortest_path_astar(destination),
            PathAlgorithm::Bfs => self.find_shortest_path_bfs(destination),
        }
    }

    fn find_shortest_path_bfs(&mut self, destination: &Point) -> Vec<Point> {
        let head = self.game.snake.back().unwrap();
        self.get_mut_cell(head).distance = 0;
        let mut queue = VecDeque::new();
        queue.push_back(head.clone());

        while !queue.is_empty() {
            let current_coord = queue.pop_front().unwrap();

            if current_coord == *destination {
                return self.traverse_path(destination);
            }

            let adj_points = self.get_adj_coords(&current_coord);

            for point in adj_points {
                if !self.game.check_overlap(&point) {
                    let dist = self.get_cell(&current_coord).distance + 1;
                    let adj_cell = self.get_mut_cell(&point);

                    if adj_cell.distance == i32::MAX {
                        adj_cell.parent = Some(current_coord.clone());
                        adj_cell.distance = dist;
                        queue.push_back(point);
                    }
                }
            }
        }

        Vec::new()
    }

    fn find_shortest_path_astar(&mut self, destination: &Point) -> Vec<Point> {
        let mut open_list = BinaryHeap::new();
        let mut closed_list =
            vec![vec![false; self.game.board.0 as usize]; self.game.board.1 as usize];

        let head = self.game.snake.back().unwrap();
        let head_cell = self.get_mut_cell(head);
        head_cell.distance = 0;

        open_list.push(AStarCost::new(
            head.clone(),
            head.manhattan_distance(destination) as i32,
        ));

        while let Some(AStarCost {
            coord: current_coord,
            ..
        }) = open_list.pop()
        {
            if closed_list[current_coord.y as usize][current_coord.x as usize] {
                continue;
            }

            if current_coord == *destination {
                return self.traverse_path(destination);
            }

            closed_list[current_coord.y as usize][current_coord.x as usize] = true;

            let adj_points = self.get_adj_coords(&current_coord);

            for point in adj_points {
                if !self.game.check_overlap(&point) {
                    let distance = self.get_cell(&current_coord).distance + 1;
                    let adj_cell = self.get_mut_cell(&point);

                    if distance < adj_cell.distance
                        || !closed_list[point.y as usize][point.x as usize]
                    {
                        adj_cell.parent = Some(current_coord.clone());
                        adj_cell.distance = distance;
                        let f_score = distance + point.manhattan_distance(destination) as i32;
                        open_list.push(AStarCost::new(point, f_score))
                    }
                }
            }
        }

        Vec::new()
    }

    pub fn traverse_path(&self, destination: &Point) -> Vec<Point> {
        let mut path = Vec::new();
        let mut current_coord = destination.clone();

        loop {
            path.push(current_coord.clone());

            if let Some(parent_cell_coord) = &self.get_cell(&current_coord).parent {
                current_coord = parent_cell_coord.clone();
            } else {
                break;
            }
        }

        path.reverse();
        path
    }

    fn build_cycle(&mut self) {
        let path = self.find_longest_path(self.game.snake.front().unwrap());
        let snake_length = self.game.snake.len();

        path.iter()
            .chain(self.game.snake.range(1..snake_length - 1))
            .enumerate()
            .for_each(|(count, point)| self.get_mut_cell(point).circuit_idx = count);
    }

    fn distance_to_tail(&self, mut checked_idx: usize) -> usize {
        let tail_idx = self.get_cell(self.game.snake.front().unwrap()).circuit_idx;

        if tail_idx > checked_idx {
            checked_idx += self.game.board_size() as usize;
        }

        checked_idx - tail_idx
    }

    pub fn next_direction(&mut self) -> Direction {
        let head_coord = self.game.snake.back().unwrap();
        let cur_idx = self.get_cell(head_coord).circuit_idx;

        let adj_points = self.get_adj_coords(head_coord);
        let mut next_coord = Point::new(0, 0);

        if self.game.snake.len() < (self.game.board_size()) as usize / 2 {
            let path = self.find_shortest_path(&self.game.apple);

            if !path.is_empty() {
                let tail_idx = self.get_cell(self.game.snake.front().unwrap()).circuit_idx;
                let head_idx = self.get_cell(&path[0]).circuit_idx;
                let next_idx = self.get_cell(&path[1]).circuit_idx;
                let apple_idx = self.get_cell(&self.game.apple).circuit_idx;

                if !(path.len() == 1 && apple_idx.abs_diff(tail_idx) == 1) {
                    let head_idx_rel = self.distance_to_tail(head_idx);
                    let next_idx_rel = self.distance_to_tail(next_idx);
                    let apple_idx_rel = self.distance_to_tail(apple_idx);
                    if next_idx_rel > head_idx_rel && next_idx_rel <= apple_idx_rel {
                        return head_coord.direction_to(&path[1]).unwrap();
                    }
                }
            }
        }

        for point in adj_points {
            if self.get_cell(&point).circuit_idx == cur_idx + 1
                || 1 + cur_idx as i32 == self.game.board_size()
                    && self.get_cell(&point).circuit_idx == 0
            {
                next_coord = point;
                break;
            }
        }

        head_coord.direction_to(&next_coord).unwrap()
    }

    fn get_adj_coords(&self, point: &Point) -> Vec<Point> {
        let mut adj_points = Vec::new();

        let x = point.x;
        let y = point.y;

        if x != 0 {
            adj_points.push(Point::new(x - 1, y))
        }

        if y != 0 {
            adj_points.push(Point::new(x, y - 1))
        }

        if x as usize != self.game_area[0].len() - 1 {
            adj_points.push(Point::new(x + 1, y))
        }

        if y as usize != self.game_area.len() - 1 {
            adj_points.push(Point::new(x, y + 1))
        }

        adj_points
    }

    fn validate_point(&self, point: &Point) -> bool {
        !self.game.check_overlap(point)
            && point.x >= 0
            && point.x < self.game.board.0
            && point.y >= 0
            && point.y < self.game.board.1
            && !self.get_cell(point).visited
    }
}
