use clap::{Parser, Subcommand};
use std::{fmt::Display, path::Path};

mod grid;

mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d2;
mod d20;
mod d21;
mod d22;
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
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
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
        Some(PuzzleDay::Day17) => d17::get_runs(),
        Some(PuzzleDay::Day18) => d18::get_runs(),
        Some(PuzzleDay::Day19) => d19::get_runs(),
        Some(PuzzleDay::Day20) => d20::get_runs(),
        Some(PuzzleDay::Day21) => d21::get_runs(),
        Some(PuzzleDay::Day22) => d22::get_runs(),
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
