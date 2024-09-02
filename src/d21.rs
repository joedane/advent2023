use super::{read_file, PuzzleRun};
use std::{
    cell::Cell,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    sync::Arc,
};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn for_index(idx: usize, width: usize) -> Self {
        Self {
            row: idx / width,
            col: idx % width,
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum GridEntry {
    Empty,
    Rock,
    Occupied,
}

enum Dir {
    N,
    E,
    S,
    W,
}

struct Grid {
    width: usize,
    height: usize,
    start: Coord,
    data: Vec<GridEntry>,
    occupied: HashSet<Coord>,
}

impl Grid {
    fn get(&self, row: usize, col: usize) -> &GridEntry {
        self.data.get(self.width * row + col).unwrap()
    }

    fn is_rock(&self, coord: Coord) -> bool {
        matches!(self.get(coord.row, coord.col), GridEntry::Rock)
    }

    fn can_move(&self, coord: Coord, dir: Dir) -> Option<Coord> {
        match dir {
            Dir::N => {
                if coord.row > 0 {
                    let new_idx = self.width * (coord.row - 1) + coord.col;
                    if matches!(self.data[new_idx], GridEntry::Empty | GridEntry::Occupied) {
                        Some(Coord::new(coord.row - 1, coord.col))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Dir::E => {
                if coord.col < self.width - 1 {
                    let new_idx = self.width * coord.row + (coord.col + 1);
                    if matches!(self.data[new_idx], GridEntry::Empty | GridEntry::Occupied) {
                        Some(Coord::new(coord.row, coord.col + 1))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Dir::S => {
                if coord.row < self.height - 1 {
                    let new_idx = self.width * (coord.row + 1) + coord.col;
                    if matches!(self.data[new_idx], GridEntry::Empty | GridEntry::Occupied) {
                        Some(Coord::new(coord.row + 1, coord.col))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Dir::W => {
                if coord.col > 0 {
                    let new_idx = self.width * coord.row + (coord.col - 1);
                    if matches!(self.data[new_idx], GridEntry::Empty | GridEntry::Occupied) {
                        Some(Coord::new(coord.row, coord.col - 1))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf: Vec<u8> = vec![];
        for row in 0..self.height {
            for col in 0..self.width {
                buf.push(match self.get(row, col) {
                    GridEntry::Empty => {
                        if row == self.start.row && col == self.start.col {
                            b'S'
                        } else {
                            b'.'
                        }
                    }
                    GridEntry::Rock => b'#',
                    GridEntry::Occupied => b'O',
                });
            }
            buf.push(b'\n');
        }
        f.write_str(std::str::from_utf8(&buf).unwrap())
    }
}
fn parse<'a, I: Iterator<Item = &'a str>>(input: I) -> Grid {
    let lines: Vec<&str> = input.collect();
    assert!(lines.iter().all(|p| p.len() == lines[0].len()));
    let start = Cell::new(Coord::new(0, 0));
    let mut data: Vec<GridEntry> = lines
        .iter()
        .enumerate()
        .flat_map(|(line, s)| {
            let start = &start; // Rust idiom?
            s.trim().chars().enumerate().map(move |(col, c)| match c {
                '.' => GridEntry::Empty,
                '#' => GridEntry::Rock,
                'S' => {
                    start.set(Coord::new(line, col));
                    GridEntry::Occupied
                }
                _ => panic!(),
            })
        })
        .collect();
    let mut occupied = HashSet::new();
    let start = start.get();
    occupied.insert(start);
    Grid {
        height: lines.len(),
        width: lines[0].len(),
        start,
        occupied,
        data: data,
    }
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day21.txt")
        /* Ok("...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........")
        */
    }

    fn run(&self, input: &str) -> String {
        let mut grid = parse(input.lines());

        for step in 0..64 {
            println!("{:?}", grid);
            let mut new_cells: HashSet<Coord> = HashSet::new();
            for idx in 0..grid.data.len() {
                let coord = Coord::for_index(idx, grid.width);
                if let GridEntry::Occupied = grid.data[idx] {
                    if let Some(moved_to) = grid.can_move(coord, Dir::N) {
                        new_cells.insert(moved_to);
                    }
                    if let Some(moved_to) = grid.can_move(coord, Dir::E) {
                        new_cells.insert(moved_to);
                    }
                    if let Some(moved_to) = grid.can_move(coord, Dir::S) {
                        new_cells.insert(moved_to);
                    }
                    if let Some(moved_to) = grid.can_move(coord, Dir::W) {
                        new_cells.insert(moved_to);
                    }
                }
            }
            let mut new_data: Vec<GridEntry> = Vec::with_capacity(grid.data.len());
            for i in 0..grid.data.len() {
                new_data.push(match grid.data[i] {
                    GridEntry::Rock => GridEntry::Rock,
                    GridEntry::Empty | GridEntry::Occupied => {
                        if new_cells.contains(&Coord::for_index(i, grid.width)) {
                            GridEntry::Occupied
                        } else {
                            GridEntry::Empty
                        }
                    }
                });
            }
            grid.occupied = new_cells;
            grid.data = new_data;
        }
        format!("{}", grid.occupied.len())
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day21.txt")
    }

    fn run(&self, input: &str) -> String {
        let mut grid = parse(input.lines());
        let mut distances: HashMap<Coord, usize> = Default::default();
        let mut q: VecDeque<(Coord, usize)> = Default::default();
        q.push_back((Coord::new(65, 65), 0));

        while let Some((c, dist)) = q.pop_front() {
            if distances.contains_key(&c) {
                continue;
            }
            distances.insert(c, dist);

            for d in [Dir::N, Dir::S, Dir::E, Dir::W] {
                if let Some(new_c) = grid.can_move(c, d) {
                    if !distances.contains_key(&new_c) && !grid.is_rock(new_c) {
                        q.push_back((new_c, dist + 1));
                    }
                }
            }
        }

        let even_corners = distances
            .iter()
            .filter(|(&c, &d)| d % 2 == 0 && d > 65)
            .count();
        let odd_corners = distances
            .iter()
            .filter(|(&c, &d)| d % 2 == 1 && d > 65)
            .count();
        println!(
            "even corners: {}, odd corners: {}",
            even_corners, odd_corners
        );

        let n = 202300;
        let solution = (n + 1) * (n + 1) * distances.iter().filter(|(&c, &d)| d % 2 == 1).count()
            + n * n * distances.iter().filter(|(&c, &d)| d % 2 == 0).count()
            - (n + 1) * odd_corners
            + n * even_corners;

        format!("{}", solution)
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_coord() {
        assert_eq!(Coord::for_index(0, 10), Coord::new(0, 0));
        assert_eq!(Coord::for_index(9, 10), Coord::new(0, 9));
        assert_eq!(Coord::for_index(12, 10), Coord::new(1, 2));
    }
    #[test]
    fn test_part1() {
        println!("{}", Part1::run(&Part1, Part1::input_data(&Part1).unwrap()));
    }

    #[test]
    fn test_part2() {
        println!("{}", Part2::run(&Part2, Part2::input_data(&Part2).unwrap()));
    }
}
