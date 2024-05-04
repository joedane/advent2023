use itertools::Itertools;
use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use super::{read_file, PuzzleRun};
use crate::grid::{Dir, Grid};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

struct Part1;

#[derive(Clone, Copy, Debug)]
enum Symbol {
    None,
    Pipe,
    Dash,
    Slash,
    Backslash,
}

impl From<Symbol> for char {
    fn from(value: Symbol) -> Self {
        match value {
            Symbol::None => '.',
            Symbol::Pipe => '|',
            Symbol::Dash => '-',
            Symbol::Slash => '/',
            Symbol::Backslash => '\\',
        }
    }
}
impl From<Symbol> for u8 {
    fn from(value: Symbol) -> Self {
        match value {
            Symbol::None => b'.',
            Symbol::Pipe => b'|',
            Symbol::Dash => b'-',
            Symbol::Slash => b'/',
            Symbol::Backslash => b'\\',
        }
    }
}
impl From<&Cell> for char {
    fn from(value: &Cell) -> Self {
        value.symbol.into()
    }
}
impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::None,
            '|' => Self::Pipe,
            '-' => Self::Dash,
            '/' => Self::Slash,
            '\\' => Self::Backslash,
            _ => panic!(),
        }
    }
}
impl From<u8> for Symbol {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::None,
            b'|' => Self::Pipe,
            b'-' => Self::Dash,
            b'/' => Self::Slash,
            b'\\' => Self::Backslash,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
struct Cell {
    symbol: Symbol,
    visited: Vec<Dir>,
}

impl Cell {
    fn new(symbol: Symbol) -> Self {
        Self {
            symbol,
            visited: Default::default(),
        }
    }

    fn visit(&mut self, dir: Dir) {
        self.visited.push(dir);
    }

    fn was_visited_from(&self, dir: Dir) -> bool {
        self.visited.contains(&dir)
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Cell::new(value.into())
    }
}
impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        Cell::new(value.into())
    }
}

#[derive(Clone, Debug)]
struct RayState {
    x: usize,
    y: usize,
    dir: Dir,
}

impl RayState {
    fn start() -> Self {
        RayState {
            x: 0,
            y: 0,
            dir: Dir::E,
        }
    }

    fn new(x: usize, y: usize, dir: Dir) -> Self {
        Self { x, y, dir }
    }

    fn copy_with_new_dir(&self, dir: Dir) -> Self {
        let mut d = self.clone();
        d.dir = dir;
        d
    }
}

struct DebugGrid<'a, T>(&'a Grid<T>);

/*
impl<'a> Display for DebugGrid<'a, Cell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Grid::fmt(&self.0, f, |c| {
            if !c.visited.is_empty() {
                TryInto::<u8>::try_into(c.visited.len() % 10)
                    .unwrap()
                    .into()
            } else {
                c.symbol.into()
            }
        })
    }
}
*/

fn score(grid: &Grid<Cell>, start: RayState) -> usize {
    let mut grid = grid.clone();
    let mut stack: Vec<RayState> = vec![];

    stack.push(start);

    'stack: while let Some(mut current) = stack.pop() {
        loop {
            ///println!("current: {:?}", current);
            //println!("{}", DebugGrid(&grid));
            let cell = grid.get_mut(current.x, current.y);
            cell.visit(current.dir);
            let next_dir = match (current.dir, cell.symbol) {
                (_, Symbol::None)
                | (Dir::N | Dir::S, Symbol::Pipe)
                | (Dir::E | Dir::W, Symbol::Dash) => current.dir,
                (Dir::N, Symbol::Slash) => Dir::E,
                (Dir::E, Symbol::Slash) => Dir::N,
                (Dir::S, Symbol::Slash) => Dir::W,
                (Dir::W, Symbol::Slash) => Dir::S,
                (Dir::N, Symbol::Backslash) => Dir::W,
                (Dir::E, Symbol::Backslash) => Dir::S,
                (Dir::S, Symbol::Backslash) => Dir::E,
                (Dir::W, Symbol::Backslash) => Dir::N,
                (Dir::N | Dir::S, Symbol::Dash) => {
                    stack.push(current.copy_with_new_dir(Dir::E));
                    Dir::W
                }
                (Dir::E | Dir::W, Symbol::Pipe) => {
                    stack.push(current.copy_with_new_dir(Dir::N));
                    Dir::S
                }
            };
            let next = grid.try_next_coord(current.x, current.y, next_dir);
            if let Some((next_x, next_y, _)) = next {
                if grid.get(next_x, next_y).was_visited_from(next_dir) {
                    continue 'stack;
                } else {
                    current = RayState::new(next_x, next_y, next_dir);
                }
            } else {
                continue 'stack;
            }
        }
    }
    grid.count_cells(|c| !c.visited.is_empty())
}
impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day16.txt")
        /*
            Ok(r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....")
        */
    }

    fn run(&self, input: &str) -> String {
        let mut grid: Grid<Cell> = Grid::from_str(input).unwrap();
        format!("{}", score(&grid, RayState::start()))
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day16.txt")
        /*
        Ok(r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....")
        */
    }

    fn run(&self, input: &str) -> String {
        let mut grid: Grid<Cell> = Grid::from_str(input).unwrap();
        let max = (0..grid.width)
            .cartesian_product([Dir::N, Dir::S])
            .chain((0..grid.height).cartesian_product([Dir::E, Dir::W]))
            .map(|(i, dir)| match dir {
                Dir::N => score(&grid.clone(), RayState::new(i, grid.height - 1, Dir::N)),
                Dir::S => score(&grid.clone(), RayState::new(i, 0, Dir::S)),
                Dir::E => score(&grid.clone(), RayState::new(0, i, Dir::E)),
                Dir::W => score(&grid.clone(), RayState::new(grid.width - 1, i, Dir::W)),
            })
            .max()
            .unwrap();

        format!("{}", max)
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_Part1() {
        println!("{}", Part1::run(&Part1, Part1::input_data(&Part1).unwrap()));
    }

    #[test]
    fn test_Part2() {
        println!("{}", Part2::run(&Part2, Part2::input_data(&Part2).unwrap()));
    }
}
