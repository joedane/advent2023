use crate::{read_file, PuzzleRun};
use std::ops::Index;

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Clone, Copy, PartialEq)]
enum Symbol {
    Vert,
    Horz,
    Ell,
    Jay,
    Seven,
    Eff,
    Start,
    Dot,
}

impl From<u8> for Symbol {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Symbol::Vert,
            b'-' => Symbol::Horz,
            b'L' => Symbol::Ell,
            b'J' => Symbol::Jay,
            b'7' => Symbol::Seven,
            b'F' => Symbol::Eff,
            b'S' => Symbol::Start,
            b'.' => Symbol::Dot,
            _ => panic!("bad symbol: {}", value),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    row: u16,
    col: u16,
}

impl Coord {
    fn new(row: u16, col: u16) -> Self {
        Self { row, col }
    }

    fn north(&self) -> Self {
        Self {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn south(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn west(&self) -> Self {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn east(&self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }
    fn next(&self, last: Coord, symbol: Symbol) -> Self {
        match symbol {
            Symbol::Vert => {
                if last.row < self.row {
                    self.south()
                } else {
                    self.north()
                }
            }
            Symbol::Horz => {
                if last.col < self.col {
                    self.east()
                } else {
                    self.west()
                }
            }
            Symbol::Ell => {
                if last.row < self.row {
                    self.east()
                } else {
                    self.north()
                }
            }
            Symbol::Jay => {
                if last.row < self.row {
                    self.west()
                } else {
                    self.north()
                }
            }
            Symbol::Seven => {
                if last.row == self.row {
                    self.south()
                } else {
                    self.west()
                }
            }
            Symbol::Eff => {
                if last.row == self.row {
                    self.south()
                } else {
                    self.east()
                }
            }
            Symbol::Start => panic!("can't move from start"),
            Symbol::Dot => panic!("Can't move from dot"),
        }
    }
}
struct Node {
    coord: Coord, // maybe we don't need this?
    symbol: Symbol,
}

impl Node {
    fn new(coord: Coord, symbol: Symbol) -> Self {
        Self { coord, symbol }
    }
}

struct Grid {
    cells: Vec<Node>,
    rows: u16,
    cols: u16,
}

impl Grid {
    fn new(cells: Vec<Node>, rows: u16, cols: u16) -> Self {
        Self { cells, rows, cols }
    }

    fn north(&self, c: Coord) -> Option<Coord> {
        if c.row >= 0 {
            Some(c.north())
        } else {
            None
        }
    }
    fn south(&self, c: Coord) -> Option<Coord> {
        if c.row + 1 < self.rows {
            Some(c.south())
        } else {
            None
        }
    }
    fn east(&self, c: Coord) -> Option<Coord> {
        if c.col + 1 < self.cols {
            Some(c.east())
        } else {
            None
        }
    }
    fn west(&self, c: Coord) -> Option<Coord> {
        if c.col > 0 {
            Some(c.west())
        } else {
            None
        }
    }
}

impl Index<Coord> for Grid {
    type Output = Node;

    fn index(&self, index: Coord) -> &Self::Output {
        let i = index.row as usize * self.cols as usize + index.col as usize;
        &self.cells.get(i).unwrap()
    }
}

struct Part1;

fn parse(input: &str) -> (Grid, Coord) {
    let mut num_rows: u16 = 0;
    let mut num_cols: u16 = 0;
    let mut start: Option<Coord> = None;

    for (line_no, line) in input.lines().map(|s| s.trim()).enumerate() {
        num_rows += 1;
        num_cols = 0;
        for (col_no, c) in line.bytes().enumerate() {
            num_cols += 1;
        }
    }

    let mut v = Vec::with_capacity(num_rows as usize * num_cols as usize);

    for (line_no, line) in input.lines().map(|s| s.trim()).enumerate() {
        for (col_no, c) in line.bytes().enumerate() {
            v.push(Node::new(
                Coord::new(line_no.try_into().unwrap(), col_no.try_into().unwrap()),
                c.into(),
            ));
            if c == b'S' {
                start.replace(Coord::new(
                    line_no.try_into().unwrap(),
                    col_no.try_into().unwrap(),
                ));
            }
        }
    }
    (Grid::new(v, num_rows, num_cols), start.unwrap())
}

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day10.txt")

        /*Ok("..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...")*/
    }

    fn run(&self, input: &str) -> String {
        let (grid, start) = parse(input);
        let mut maybe_first_steps = vec![];

        if let Some(g) = grid.north(start) {
            if grid[g].symbol == Symbol::Vert
                || grid[g].symbol == Symbol::Seven
                || grid[g].symbol == Symbol::Eff
            {
                maybe_first_steps.push(g);
            }
        }
        if let Some(g) = grid.south(start) {
            if grid[g].symbol == Symbol::Vert
                || grid[g].symbol == Symbol::Ell
                || grid[g].symbol == Symbol::Jay
            {
                maybe_first_steps.push(g);
            }
        }
        if let Some(g) = grid.west(start) {
            if grid[g].symbol == Symbol::Horz
                || grid[g].symbol == Symbol::Ell
                || grid[g].symbol == Symbol::Eff
            {
                maybe_first_steps.push(g);
            }
        }
        if let Some(g) = grid.east(start) {
            if grid[g].symbol == Symbol::Horz
                || grid[g].symbol == Symbol::Jay
                || grid[g].symbol == Symbol::Seven
            {
                maybe_first_steps.push(g);
            }
        }
        if maybe_first_steps.len() != 2 {
            panic!("invalid starting position");
        }
        let (mut last_l, mut last_r) = (start, start);
        let (mut steps_l, mut steps_r) = (1_u16, 1_u16);
        let (mut step_l, mut step_r) = (maybe_first_steps[0], maybe_first_steps[1]);
        let mut move_l = true;

        while step_l != step_r {
            if move_l {
                let next = step_l.next(last_l, grid[step_l].symbol);
                last_l = step_l;
                step_l = next;
                steps_l += 1;
                move_l = false;
            } else {
                let next = step_r.next(last_r, grid[step_r].symbol);
                last_r = step_r;
                step_r = next;
                steps_r += 1;
                move_l = true;
            };
        }
        format!("right: {}, left: {}", steps_r, steps_l)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1() {
        println!("{}", Part1.run(Part1.input_data().unwrap()));
    }
}
