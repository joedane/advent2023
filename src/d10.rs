use crate::{read_file, PuzzleRun};
use std::ops::{Index, IndexMut};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Debug)]
struct Node {
    symbol: Symbol,
    onpath: bool,
}

impl Node {
    fn new(_coord: Coord, symbol: Symbol) -> Self {
        Self {
            symbol,
            onpath: false,
        }
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
        if c.row > 0 {
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
        &self.cells[i]
    }
}

impl IndexMut<Coord> for Grid {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let i = index.row as usize * self.cols as usize + index.col as usize;
        &mut self.cells[i]
    }
}
struct Part1;

fn parse(input: &str) -> (Grid, Coord) {
    let mut num_rows: u16 = 0;
    let mut num_cols: u16 = 0;
    let mut start: Option<Coord> = None;

    for line in input.lines().map(|s| s.trim()) {
        num_rows += 1;
        num_cols = 0;
        for _c in line.bytes().enumerate() {
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

fn walk_path(grid: &mut Grid, start: Coord) -> (u16, u16, Symbol) {
    #[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
    enum Dir {
        North,
        South,
        East,
        West,
    }
    use Dir::*;

    let mut maybe_first_steps: Vec<(Coord, Dir)> = vec![];

    if let Some(g) = grid.north(start) {
        if grid[g].symbol == Symbol::Vert
            || grid[g].symbol == Symbol::Seven
            || grid[g].symbol == Symbol::Eff
        {
            maybe_first_steps.push((g, Dir::North));
        }
    }
    if let Some(g) = grid.south(start) {
        if grid[g].symbol == Symbol::Vert
            || grid[g].symbol == Symbol::Ell
            || grid[g].symbol == Symbol::Jay
        {
            maybe_first_steps.push((g, Dir::South));
        }
    }
    if let Some(g) = grid.west(start) {
        if grid[g].symbol == Symbol::Horz
            || grid[g].symbol == Symbol::Ell
            || grid[g].symbol == Symbol::Eff
        {
            maybe_first_steps.push((g, Dir::West));
        }
    }
    if let Some(g) = grid.east(start) {
        if grid[g].symbol == Symbol::Horz
            || grid[g].symbol == Symbol::Jay
            || grid[g].symbol == Symbol::Seven
        {
            maybe_first_steps.push((g, Dir::East));
        }
    }
    if maybe_first_steps.len() != 2 {
        panic!("invalid starting position");
    }

    maybe_first_steps.sort_by_key(|d| d.1);

    let start_symbol: Symbol = match (maybe_first_steps[0].1, maybe_first_steps[1].1) {
        (North, South) => Symbol::Vert,
        (East, West) => Symbol::Horz,
        (North, East) => Symbol::Ell,
        (North, West) => Symbol::Jay,
        (South, East) => Symbol::Eff,
        (South, West) => Symbol::Seven,
        _ => panic!(),
    };

    let (mut last_l, mut last_r) = (start, start);
    let (mut steps_l, mut steps_r) = (1_u16, 1_u16);
    let (mut step_l, mut step_r) = (maybe_first_steps[0].0, maybe_first_steps[1].0);
    grid[start].onpath = true;
    grid[step_l].onpath = true;
    grid[step_r].onpath = true;

    let mut move_l = true;

    while step_l != step_r {
        if move_l {
            let next = step_l.next(last_l, grid[step_l].symbol);
            grid[next].onpath = true;
            last_l = step_l;
            step_l = next;
            steps_l += 1;
            move_l = false;
        } else {
            let next = step_r.next(last_r, grid[step_r].symbol);
            grid[next].onpath = true;
            last_r = step_r;
            step_r = next;
            steps_r += 1;
            move_l = true;
        };
    }
    (steps_r, steps_l, start_symbol)
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
        let (mut grid, start) = parse(input);
        let (steps_r, steps_l, _) = walk_path(&mut grid, start);
        format!("right: {}, left: {}", steps_r, steps_l)
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day10.txt")
        /*
        Ok("...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........")
        */
        /*
            Ok(".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...")
        */
        /*
            Ok("FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L")
            */
    }

    fn run(&self, input: &str) -> String {
        #[derive(Clone, Copy, Debug)]
        enum InOut {
            Outside,
            OnFromNorth(bool), // true if we came from inside
            OnFromSouth(bool),
            Inside,
        }
        use InOut::*;
        use Symbol::*;

        let (mut grid, start) = parse(input);
        let mut inside_count = 0_u32;

        let (_, _, start_symbol) = walk_path(&mut grid, start);
        grid[start].symbol = start_symbol;

        for row in 0..grid.rows {
            let row_start = Coord::new(row, 0);
            let mut current_state = if grid[row_start].onpath {
                match grid[row_start].symbol {
                    Ell => OnFromNorth(false),
                    Eff => OnFromSouth(false),
                    Vert => Inside,
                    _ => panic!("can't start a row on symbol {:?}", grid[row_start].symbol),
                }
            } else {
                Outside
            };

            for col in 1..grid.cols - 1 {
                // first/last columns can't be inside
                let c = Coord::new(row, col);
                if !grid[c].onpath {
                    if matches!(current_state, Inside) {
                        inside_count += 1;
                    }
                    continue;
                }

                // if we're here, the current token is on the path
                current_state = match (current_state, grid[c].symbol) {
                    (Outside, Symbol::Vert) => Inside,
                    (Inside, Symbol::Vert) => Outside,
                    (Outside, Symbol::Eff) => OnFromSouth(false),
                    (Inside, Symbol::Eff) => OnFromSouth(true),
                    (Outside, Symbol::Ell) => OnFromNorth(false),
                    (Inside, Symbol::Ell) => OnFromNorth(true),
                    (OnFromSouth(true), Symbol::Seven) => Inside,
                    (OnFromSouth(false), Symbol::Seven) => Outside,
                    (OnFromSouth(true), Symbol::Jay) => Outside,
                    (OnFromSouth(false), Symbol::Jay) => Inside,
                    (OnFromNorth(true), Symbol::Jay) => Inside,
                    (OnFromNorth(false), Symbol::Jay) => Outside,
                    (OnFromNorth(true), Symbol::Seven) => Outside,
                    (OnFromNorth(false), Symbol::Seven) => Inside,
                    v @ (OnFromNorth(_) | OnFromSouth(_), Symbol::Horz) => v.0,
                    x => panic!("at coord {:?} failed to match: {:?}", c, x),
                }
            }
        }
        format!("{}", inside_count)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1() {
        println!("{}", Part1.run(Part1.input_data().unwrap()));
    }

    #[test]
    fn test_part2() {
        println!("{}", Part2.run(Part2.input_data().unwrap()));
    }
}
