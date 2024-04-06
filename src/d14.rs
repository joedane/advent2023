use std::ops::{Index, IndexMut};

use super::{read_file, PuzzleRun};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug, Copy, Clone)]
enum State {
    Empty,
    Round,
    Cube,
}

impl State {
    fn value(self) -> u64 {
        match self {
            Self::Empty => 0,
            Self::Cube => 0,
            Self::Round => 1,
        }
    }
}
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

#[derive(Debug)]
struct Cell {
    data: Vec<State>,
}

impl Cell {
    fn new(s: State) -> Self {
        Cell { data: vec![s] }
    }

    fn push(&mut self, s: State) {
        self.data.push(s);
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j)
    }

    fn score(&self) -> u64 {
        let l = self.data.len();
        self.data
            .iter()
            .enumerate()
            .map(|(i, s)| (l - i) as u64 * s.value())
            .sum()
    }
}

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

struct Part1;

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
        let mut lines = input.lines().map(str::trim);
        let Some(line) = lines.next() else { panic!() };
        let mut cells: Vec<Cell> = line.chars().map(|c| Cell::new(c.into())).collect();
        while let Some(line) = lines.next() {
            for (i, c) in line.chars().enumerate() {
                cells[i].push(c.into());
            }
        }

        for i in 0..cells.len() {
            let c = &mut cells[i];
            let mut move_to: Option<usize> = None;
            for j in 0..c.len() {
                match c[j] {
                    State::Cube => {
                        move_to = None;
                    }
                    State::Empty => {
                        if move_to.is_none() {
                            move_to = Some(j)
                        }
                    }
                    State::Round => {
                        if let Some(k) = move_to {
                            c.swap(j, k);
                            move_to = Some(k + 1);
                        }
                    }
                }
            }
        }
        format!("{}", cells.into_iter().map(|c| c.score()).sum::<u64>())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1() {
        println!("{}", Part1::run(&Part1, Part1::input_data(&Part1).unwrap()));
    }
}
