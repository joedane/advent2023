use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    ops::{Index, IndexMut},
};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use super::{read_file, PuzzleRun};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum State {
    Empty,
    Round,
    Cube,
}

impl State {}
impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            'O' => Self::Round,
            '#' => Self::Cube,
            _ => panic!("invalid state: {}", value),
        }
    }
}

impl From<State> for char {
    fn from(value: State) -> Self {
        match value {
            State::Empty => '.',
            State::Round => 'O',
            State::Cube => '#',
        }
    }
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Grid {
    data: Box<[State]>,
    width: usize,
    height: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("Grid ({} by {}):\n", self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                s.push(self.get(row, col).into());
            }
            s.push('\n');
        }
        s.push('\n');
        write!(f, "{}", s)
    }
}
impl Grid {
    fn new(input: &str) -> Self {
        let mut v: Vec<State> = Default::default();
        let mut width = 0;
        for line in input.lines().map(str::trim) {
            let mut w = 0;
            for c in line.chars() {
                v.push(c.into());
                w += 1;
            }
            width = w;
        }
        Grid {
            height: v.len() / width,
            data: v.into_boxed_slice(),
            width,
        }
    }

    fn get(&self, row: usize, col: usize) -> State {
        self.data[self.coord(col, row)]
    }

    fn from_coord(&self, coord: usize) -> (usize, usize) {
        (coord / self.width, coord % self.width)
    }

    fn coord(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn print_row(&self, row: usize) {
        let mut msg = format!("row {}: ", row);
        for c in 0..self.width {
            msg.push(self.get(row, c).into());
        }
        println!("{}", msg);
    }
    /*
       can't be called from the first row/column
    */
    fn can_move_from(&self, row: usize, col: usize, dir: Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::N => {
                let mut r = row;
                if matches!(self.get(r - 1, col), State::Cube | State::Round) {
                    None
                } else {
                    while r > 0 && matches!(self.get(r - 1, col), State::Empty) {
                        r -= 1;
                    }
                    Some((r, col))
                }
            }
            Dir::S => {
                let mut r = row;
                if matches!(self.get(r + 1, col), State::Cube | State::Round) {
                    None
                } else {
                    while r < (self.height - 1) && matches!(self.get(r + 1, col), State::Empty) {
                        r += 1;
                    }
                    Some((r, col))
                }
            }
            Dir::E => {
                let mut c = col;
                if matches!(self.get(row, c + 1), State::Cube | State::Round) {
                    None
                } else {
                    while c < (self.width - 1) && matches!(self.get(row, c + 1), State::Empty) {
                        c += 1;
                    }
                    Some((row, c))
                }
            }
            Dir::W => {
                let mut c = col;
                if matches!(self.get(row, c - 1), State::Cube | State::Round) {
                    None
                } else {
                    while c > 0 && matches!(self.get(row, c - 1), State::Empty) {
                        c -= 1;
                    }
                    Some((row, c))
                }
            }
        }
    }

    fn swap(&mut self, x_i: usize, y_i: usize, x_j: usize, y_j: usize) {
        self.data.swap(self.coord(x_i, y_i), self.coord(x_j, y_j));
    }

    fn score_for(&self, state: State, coord: usize, dir: Dir) -> usize {
        match state {
            State::Empty => 0,
            State::Cube => 0,
            State::Round => match dir {
                Dir::N => {
                    let (r, _) = self.from_coord(coord);
                    self.height - r
                }
                Dir::S => todo!(),
                Dir::E => todo!(),
                Dir::W => todo!(),
            },
        }
    }
    fn score(&self, dir: Dir) -> usize {
        self.data
            .par_iter()
            .enumerate()
            .map(|(i, s)| self.score_for(*s, i, dir))
            .sum()
    }

    fn index_iterator(&self, d: Dir) -> GridIndexIterator {
        match d {
            Dir::N => GridIndexIterator {
                width: self.width,
                height: self.height,
                data_len: self.data.len(),
                next: Some((1, 0)),
                dir: Dir::N,
                seen: 0,
            },
            Dir::S => GridIndexIterator {
                width: self.width,
                height: self.height,
                data_len: self.data.len(),
                next: Some((self.height - 2, 0)),
                dir: Dir::S,
                seen: 0,
            },
            Dir::W => GridIndexIterator {
                width: self.width,
                height: self.height,
                data_len: self.data.len(),
                next: Some((0, 1)),
                dir: Dir::W,
                seen: 0,
            },
            Dir::E => GridIndexIterator {
                width: self.width,
                height: self.height,
                data_len: self.data.len(),
                next: Some((0, self.width - 2)),
                dir: Dir::E,
                seen: 0,
            },
        }
    }
}

#[derive(Debug)]
struct GridIndexIterator {
    width: usize,
    height: usize,
    data_len: usize,
    next: Option<(usize, usize)>,
    dir: Dir,
    seen: usize,
}

impl GridIndexIterator {
    fn coord(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn next_idx(&mut self) -> Option<(usize, usize)> {
        let this_return = self.next;
        if let Some((mut next_row, mut next_col)) = self.next {
            match self.dir {
                Dir::N => {
                    next_col += 1;
                    if next_col == self.width {
                        next_col = 0;
                        next_row += 1;
                    }
                    if next_row == self.height {
                        self.next = None;
                    } else {
                        self.next = Some((next_row, next_col))
                    }
                }
                Dir::S => {
                    next_col += 1;
                    let mut next_row_signed = next_row as i32;
                    if next_col == self.width {
                        next_col = 0;
                        next_row_signed -= 1;
                    }
                    if next_row_signed < 0 {
                        self.next = None;
                    } else {
                        self.next = Some((next_row_signed as usize, next_col))
                    }
                }
                Dir::E => {
                    next_row += 1;
                    let mut next_col_signed = next_col as i32;
                    if next_row == self.height {
                        next_row = 0;
                        next_col_signed -= 1;
                    }
                    if next_col_signed < 0 {
                        self.next = None;
                    } else {
                        self.next = Some((next_row, next_col_signed as usize));
                    }
                }
                Dir::W => {
                    next_row += 1;
                    if next_row == self.height {
                        next_row = 0;
                        next_col += 1;
                    }
                    if next_col == self.width {
                        self.next = None;
                    } else {
                        self.next = Some((next_row, next_col))
                    }
                }
            }
            this_return
        } else {
            None
        }
    }
}
impl Iterator for GridIndexIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.seen == self.data_len {
            None
        } else {
            match self.next_idx() {
                Some(x) => {
                    self.seen += 1;
                    Some(x)
                }
                None => None,
            }
        }
    }
}
/*
impl Index<usize> for Cell {
    type Output = State;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Cell {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
*/

struct Part1;

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day14.txt")
        /*
        Ok("O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....")
        */
    }

    fn run(&self, input: &str) -> String {
        let mut grid = Grid::new(input);
        let dir = Dir::N;

        for (row, col) in grid.index_iterator(dir) {
            if let State::Round = grid.get(row, col) {
                if let Some((to_r, to_c)) = grid.can_move_from(row, col, dir) {
                    grid.swap(col, row, to_c, to_r);
                }
            }
        }
        format!("{}", grid.score(dir))
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day14.txt")
        /*
        Ok("O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....")
        */
    }

    fn run(&self, input: &str) -> String {
        let mut grid = Grid::new(input);
        let original_grid = grid.clone();
        let dirs = [Dir::N, Dir::W, Dir::S, Dir::E];
        let mut cache: HashMap<Grid, usize> = Default::default();
        let mut count: usize = 1;

        let (first, second) = loop {
            for dir in dirs {
                for (row, col) in grid.index_iterator(dir) {
                    if let State::Round = grid.get(row, col) {
                        if let Some((to_r, to_c)) = grid.can_move_from(row, col, dir) {
                            grid.swap(col, row, to_c, to_r);
                        }
                    }
                }
            }
            println!("after {} cycle score is {}", count, grid.score(Dir::N));
            match cache.entry(grid.clone()) {
                Entry::Occupied(e) => break (e.get().clone(), count),
                Entry::Vacant(e) => {
                    e.insert(count);
                }
            }
            count += 1;
        };

        println!(
            "found cycle of length {} starting at {}",
            second - first,
            first
        );
        let cycle_len = second - first;
        let d = first - 1 + ((1_000_000_000 - first + 1) % cycle_len);

        println!("final run of length {}", d);
        let mut grid = original_grid;
        for i in 0..d {
            for dir in dirs {
                for (row, col) in grid.index_iterator(dir) {
                    if let State::Round = grid.get(row, col) {
                        if let Some((to_r, to_c)) = grid.can_move_from(row, col, dir) {
                            grid.swap(col, row, to_c, to_r);
                        }
                    }
                }
            }
            println!("after {} cycle, score is {}", i + 1, grid.score(Dir::N));
        }
        format!("{}", grid.score(Dir::N))
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1() {
        println!("{}", Part1::run(&Part1, Part1::input_data(&Part1).unwrap()));
    }

    #[test]
    fn test_iter() {
        #[rustfmt::skip]
        let grid = Grid::new(
            "O.#
            .OO
            OOO");

        let mut iter = grid.index_iterator(Dir::N);
        assert_eq!(Some((1, 0)), iter.next());
        assert_eq!(Some((1, 1)), iter.next());
        assert_eq!(Some((1, 2)), iter.next());
        assert_eq!(Some((2, 0)), iter.next());
        assert_eq!(Some((2, 1)), iter.next());
        assert_eq!(Some((2, 2)), iter.next());

        let mut iter = grid.index_iterator(Dir::S);
        assert_eq!(Some((1, 0)), iter.next());
        assert_eq!(Some((1, 1)), iter.next());
        assert_eq!(Some((1, 2)), iter.next());
        assert_eq!(Some((0, 0)), iter.next());
        assert_eq!(Some((0, 1)), iter.next());
        assert_eq!(Some((0, 2)), iter.next());

        let mut iter = grid.index_iterator(Dir::E);
        assert_eq!(Some((0, 1)), iter.next());
        assert_eq!(Some((1, 1)), iter.next());
        assert_eq!(Some((2, 1)), iter.next());
        assert_eq!(Some((0, 0)), iter.next());
        assert_eq!(Some((1, 0)), iter.next());
        assert_eq!(Some((2, 0)), iter.next());

        let mut iter = grid.index_iterator(Dir::W);
        assert_eq!(Some((0, 1)), iter.next());
        assert_eq!(Some((1, 1)), iter.next());
        assert_eq!(Some((2, 1)), iter.next());
        assert_eq!(Some((0, 2)), iter.next());
        assert_eq!(Some((1, 2)), iter.next());
        assert_eq!(Some((2, 2)), iter.next());
    }
    #[test]
    fn test_move() {
        #[rustfmt::skip]
        let grid = Grid::new(
       "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....",
        );

        assert_eq!(None, grid.can_move_from(1, 0, Dir::N));
        assert_eq!(Some((0, 2)), grid.can_move_from(1, 2, Dir::N));
        assert_eq!(None, grid.can_move_from(6, 2, Dir::N));
        assert_eq!(Some((3, 5)), grid.can_move_from(5, 5, Dir::N));
        assert_eq!(Some((0, 2)), grid.can_move_from(1, 2, Dir::N));

        assert_eq!(Some((0, 4)), grid.can_move_from(0, 0, Dir::E));
        assert_eq!(Some((6, 4)), grid.can_move_from(6, 2, Dir::E));
    }

    #[test]
    fn test_part2() {
        println!("{}", Part2::run(&Part2, Part2::input_data(&Part2).unwrap()));
    }
}
