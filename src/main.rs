use clap::{Parser, Subcommand};
use std::{fmt::Display, path::Path};

mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

#[macro_use]
extern crate lazy_static;

pub trait PuzzleRun {
    fn input_data(&self) -> anyhow::Result<&str>;

    fn run(&self, input: &str) -> String;
}

fn read_file<P: AsRef<Path>>(filename: P) -> anyhow::Result<&'static str> {
    let data = std::fs::read_to_string(filename)?;
    Ok(data.leak())
}

#[derive(Parser)]
struct Args {
    /// Enable debug logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Which day's puzzles to run?
    #[command(subcommand)]
    day: Option<PuzzleDay>,
}
#[derive(Subcommand, Debug)]
enum PuzzleDay {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let puzzles = match args.day {
        Some(PuzzleDay::Day1) => d1::get_runs(),
        Some(PuzzleDay::Day2) => d2::get_runs(),
        Some(PuzzleDay::Day3) => d3::get_runs(),
        Some(PuzzleDay::Day4) => d4::get_runs(),
        Some(PuzzleDay::Day5) => d5::get_runs(),
        Some(PuzzleDay::Day6) => d6::get_runs(),
        Some(PuzzleDay::Day7) => d7::get_runs(),
        Some(PuzzleDay::Day8) => d8::get_runs(),
        Some(PuzzleDay::Day9) => d9::get_runs(),
        Some(PuzzleDay::Day10) => d10::get_runs(),
        Some(PuzzleDay::Day11) => d11::get_runs(),
        Some(PuzzleDay::Day12) => d12::get_runs(),
        Some(PuzzleDay::Day13) => d13::get_runs(),
        Some(PuzzleDay::Day14) => d14::get_runs(),
        Some(PuzzleDay::Day15) => d15::get_runs(),
        Some(PuzzleDay::Day16) => d16::get_runs(),
        _ => {
            println!("not found: {:?}", args.day);
            panic!()
        }
    };

    for puzzle in puzzles {
        println!("{}", puzzle.run(puzzle.input_data()?));
    }
    Ok(())
}

struct Grid<T> {
    data: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}
impl<T> Grid<T>
where
    T: From<char>,
{
    fn new(input: &str) -> Self {
        let mut v: Vec<T> = Default::default();
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
}

impl<T> Grid<T> {
    fn get(&self, x: usize, y: usize) -> &T {
        &self.data[self.coord(x, y)]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[self.coord(x, y)]
    }

    fn from_coord(&self, coord: usize) -> (usize, usize) {
        (coord / self.width, coord % self.width)
    }

    fn coord(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl<T> Grid<T> {
    fn fmt<F>(&self, f: &mut std::fmt::Formatter<'_>, conv: F) -> std::fmt::Result
    where
        F: Fn(&T) -> &str,
    {
        let mut s = format!("Grid ({} by {}):\n", self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                s.push_str(conv(self.get(col, row)));
            }
            s.push('\n');
        }
        s.push('\n');
        write!(f, "{}", s)
    }
}
/*
impl<T> Display for Grid<T>
where
    char: for<'a> From<&'a T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, |i| i.into())
    }
}
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl<T> Grid<T> {
    fn count_cells<F>(&self, f: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        self.data.iter().filter(|&i| f(i)).count()
    }

    /**
     * Dir in the return tuple is the direction from which we'll move
     */
    fn cardinal_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize, Dir)> {
        let mut v = vec![];
        if let Some(next) = self.try_next_coord(x, y, Dir::N) {
            v.push((next.0, next.1, Dir::S));
        }
        if let Some(next) = self.try_next_coord(x, y, Dir::S) {
            v.push((next.0, next.1, Dir::N));
        }
        if let Some(next) = self.try_next_coord(x, y, Dir::E) {
            v.push((next.0, next.1, Dir::W));
        }
        if let Some(next) = self.try_next_coord(x, y, Dir::W) {
            v.push((next.0, next.1, Dir::E));
        }
        v
    }
    fn try_next_coord(&self, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::E => {
                if x + 1 < self.width {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            Dir::W => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Dir::N => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Dir::S => {
                if y + 1 < self.height {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
        }
    }

    fn try_next(&self, x: usize, y: usize, dir: Dir) -> Option<&T> {
        self.try_next_coord(x, y, dir)
            .and_then(|(x, y)| Some(self.get(x, y)))
    }

    fn try_next_mut(&mut self, x: usize, y: usize, dir: Dir) -> Option<&mut T> {
        self.try_next_coord(x, y, dir)
            .and_then(|(x, y)| Some(self.get_mut(x, y)))
    }
}
