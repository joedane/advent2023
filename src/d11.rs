use crate::{read_file, PuzzleRun};
use std::collections::{BTreeSet, HashSet};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Coord {
    row: u32,
    col: u32,
}

impl Coord {
    fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }

    fn dist(&self, other: &Self) -> u32 {
        let d =
            (self.row as i32 - other.row as i32).abs() + (self.col as i32 - other.col as i32).abs();
        d.try_into().unwrap()
    }
}

fn dump(rows: u32, cols: u32, map: &BTreeSet<Coord>) {
    #![allow(clippy::println_empty_string)]
    let mut id = 0_u32;
    for r in 0..rows {
        for c in 0..cols {
            if map.contains(&Coord::new(r, c)) {
                id += 1;
                print!("{}", id);
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day11.txt")
        /*
        Ok("...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....")
        */
    }

    fn run(&self, input: &str) -> String {
        #![allow(unused_assignments)]
        let mut galaxies: Vec<Coord> = Default::default();
        let mut row: u32 = 0;
        let mut col: u32 = 0;
        let mut num_cols: u32 = 0;
        let mut cols_occupied: HashSet<u32> = Default::default();
        let mut rows_occupied: HashSet<u32> = Default::default();

        for line in input.lines().map(|s| s.trim()) {
            col = 0;
            for c in line.bytes() {
                if c == b'#' {
                    rows_occupied.insert(row);
                    cols_occupied.insert(col);
                    galaxies.push(Coord::new(row, col));
                }
                col += 1;
            }
            // assume a rectangular input
            num_cols = col;
            row += 1;
        }

        let mut empties: Vec<u32> = vec![];
        for c in 0..num_cols {
            if !cols_occupied.contains(&c) {
                empties.push(c);
            }
        }
        let mut n_skips = 0_u32;
        for this_col in &empties {
            num_cols += 1;
            for g in galaxies.iter_mut() {
                if g.col > *this_col + n_skips {
                    g.col += 1;
                }
            }
            n_skips += 1;
        }

        empties.clear();
        for r in 0..row {
            if !rows_occupied.contains(&r) {
                empties.push(r);
            }
        }
        n_skips = 0;
        for e in empties {
            row += 1;
            for g in galaxies.iter_mut() {
                if g.row > (e + n_skips) {
                    g.row += 1;
                }
            }
            n_skips += 1;
        }

        let mut total_dist = 0_u32;
        let galaxy_map: BTreeSet<Coord> = galaxies.into_iter().collect();
        dump(row, num_cols, &galaxy_map);
        for galaxy_i in galaxy_map.iter() {
            for galaxy_j in galaxy_map.range(galaxy_i..) {
                let dist = galaxy_i.dist(galaxy_j);
                total_dist += dist;
                println!("dist from {:?} to {:?}: {}", galaxy_i, galaxy_j, dist)
            }
        }
        format!("{}", total_dist)
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day11.txt")
        /*       Ok("...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....")
        */
    }

    fn run(&self, input: &str) -> String {
        #![allow(unused_assignments)]
        let mut galaxies: Vec<Coord> = Default::default();
        let mut row: u32 = 0;
        let mut col: u32 = 0;
        let mut num_cols: u32 = 0;
        let mut cols_occupied: HashSet<u32> = Default::default();
        let mut rows_occupied: HashSet<u32> = Default::default();

        for line in input.lines().map(|s| s.trim()) {
            col = 0;
            for c in line.bytes() {
                if c == b'#' {
                    rows_occupied.insert(row);
                    cols_occupied.insert(col);
                    galaxies.push(Coord::new(row, col));
                }
                col += 1;
            }
            // assume a rectangular input
            num_cols = col;
            row += 1;
        }

        let mut empties: Vec<u32> = vec![];
        for c in 0..num_cols {
            if !cols_occupied.contains(&c) {
                empties.push(c);
            }
        }
        let mut n_skips = 0_u32;
        for this_col in &empties {
            num_cols += 999999;
            for g in galaxies.iter_mut() {
                if g.col > *this_col + (n_skips * 999999) {
                    g.col += 999999;
                }
            }
            n_skips += 1;
        }

        empties.clear();
        for r in 0..row {
            if !rows_occupied.contains(&r) {
                empties.push(r);
            }
        }
        n_skips = 0;
        for this_row in empties {
            row += 999999;
            for g in galaxies.iter_mut() {
                if g.row > this_row + n_skips * 999999 {
                    g.row += 999999;
                }
            }
            n_skips += 1;
        }

        let mut total_dist = 0_u64;
        let galaxy_map: BTreeSet<Coord> = galaxies.into_iter().collect();
        for galaxy_i in galaxy_map.iter() {
            for galaxy_j in galaxy_map.range((
                std::ops::Bound::Excluded(galaxy_i),
                std::ops::Bound::Unbounded,
            )) {
                let dist = galaxy_i.dist(galaxy_j);
                total_dist += dist as u64;
                println!("dist from {:?} to {:?}: {}", galaxy_i, galaxy_j, dist)
            }
        }
        format!("{}", total_dist)
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
