use crate::{read_file, PuzzleRun};
use std::collections::{HashMap, HashSet};

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LineNo(u32);

impl LineNo {
    fn new(l: usize) -> Self {
        Self(l.try_into().unwrap())
    }

    fn is_first(&self) -> bool {
        self.0 == 0
    }

    fn prev(&self) -> Self {
        if self.0 == 0 {
            panic!();
        } else {
            LineNo(self.0 - 1)
        }
    }

    fn next(&self) -> Self {
        LineNo(self.0 + 1)
    }
}
#[derive(Debug)]
struct PartNo {
    part_no: u32,
    line: LineNo,
    start_col: usize,
    end_col: usize,
}

impl PartNo {
    fn new(part_no: u32, line: LineNo, start_col: usize, end_col: usize) -> Self {
        Self {
            part_no,
            line,
            start_col,
            end_col,
        }
    }
}

#[derive(Debug)]
struct Special {
    code: char,
    line: LineNo,
    col: usize,
    parts: HashSet<u32>,
}

impl Special {
    fn new(code: char, line: LineNo, col: usize) -> Self {
        Self {
            code,
            line,
            col,
            parts: HashSet::new(),
        }
    }
}
struct Part1;

fn add_part(
    part_nos: &mut Vec<PartNo>,
    line: &str,
    line_no: LineNo,
    start_col: usize,
    end_col: usize,
) {
    let part_no: u32 = line[start_col..end_col].parse().unwrap();
    part_nos.push(PartNo::new(part_no, line_no, start_col, end_col - 1));
}

fn is_neighbor(part: &PartNo, special: &mut Special) -> bool {
    let is_it = !(part.end_col < special.col - 1 || part.start_col > special.col + 1);
    if is_it {
        special.parts.insert(part.part_no);
    }
    is_it
}

fn _run(input: &str) -> (Vec<PartNo>, HashMap<LineNo, Vec<Special>>) {
    let mut specials: HashMap<LineNo, Vec<Special>> = HashMap::new();
    let mut part_nos: Vec<PartNo> = Vec::new();

    for (line_no, line) in input.lines().enumerate() {
        let line = line.trim();
        let mut start_col: Option<usize> = None;

        for (col_no, c) in line.chars().enumerate() {
            if c == '.' {
                if let Some(sc) = start_col.take() {
                    let line_no = LineNo::new(line_no);
                    println!(
                        "at line {:?}, reading string '{}'",
                        line_no,
                        &line[sc..col_no]
                    );
                    add_part(&mut part_nos, line, line_no, sc, col_no);
                }
            } else if c.is_ascii_digit() {
                if start_col.is_none() {
                    start_col = Some(col_no);
                }
            } else {
                let e = specials.entry(LineNo::new(line_no));
                e.or_default()
                    .push(Special::new(c, LineNo::new(line_no), col_no));
                if let Some(sc) = start_col.take() {
                    let line_no = LineNo::new(line_no);
                    add_part(&mut part_nos, line, line_no, sc, col_no);
                }
            }
        }
        if let Some(sc) = start_col.take() {
            // ended the line on a number
            let line_no = LineNo::new(line_no);
            add_part(&mut part_nos, line, line_no, sc, line.len());
        }
    }

    part_nos.retain(|mut part| {
        let mut is_n = false;
        if let Some(v) = specials.get_mut(&part.line) {
            for s in v.iter_mut() {
                if is_neighbor(&part, s) {
                    is_n = true;
                }
            }
        }

        if !part.line.is_first() {
            if let Some(v) = specials.get_mut(&(part.line.prev())) {
                for s in v.iter_mut() {
                    if is_neighbor(&part, s) {
                        is_n = true;
                    }
                }
            }
        }
        if let Some(v) = specials.get_mut(&part.line.next()) {
            for s in v.iter_mut() {
                if is_neighbor(&part, s) {
                    is_n = true;
                }
            }
        }
        println!("removing part {}", part.part_no);
        is_n
    });

    println!(
        "retained parts {:?}",
        part_nos.iter().map(|p| p.part_no).collect::<Vec<u32>>()
    );
    (part_nos, specials)
}

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day3.txt")
    }

    fn run(&self, input: &str) -> String {
        let (part_nos, specials) = _run(input);
        let sum: u32 = part_nos.iter().map(|p| p.part_no).sum();

        format!("{}", sum)
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day3.txt")
        /*
        Ok("467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..")
            */
    }

    fn run(&self, input: &str) -> String {
        let (part_nos, specials) = _run(input);

        let v: u32 = specials
            .values()
            .flat_map(|v| v.iter().filter(|s| s.parts.len() == 2))
            .map(|s| {
                s.parts
                    .iter()
                    .copied()
                    .reduce(|acc: u32, e: u32| -> u32 { acc * e })
                    .unwrap()
            })
            .sum();

        format!("{}", v)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1() {
        Part1.run(Part1.input_data().unwrap());
    }
}
