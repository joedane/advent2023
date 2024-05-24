use regex::Regex;
use std::str::FromStr;

use super::{read_file, PuzzleRun};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug)]
enum Dir {
    U,
    D,
    R,
    L,
}

impl FromStr for Dir {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Dir::U),
            "D" => Ok(Dir::D),
            "R" => Ok(Dir::R),
            "L" => Ok(Dir::L),
            _ => Err("bad direction"),
        }
    }
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    count: u32,
    code: String,
}

impl Step {
    fn new(dir: Dir, count: u32, code: String) -> Self {
        Self { dir, count, code }
    }
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        /*
        Ok("R 6 (#70c710)
         D 5 (#0dc571)
         L 2 (#5713f0)
         D 2 (#d2c081)
         R 2 (#59c680)
         D 2 (#411b91)
         L 5 (#8ceee2)
         U 2 (#caa173)
         L 1 (#1b58a2)
         U 2 (#caa171)
         R 2 (#7807d2)
         U 3 (#a77fa3)
         L 2 (#015232)
         U 2 (#7a21e3)")
         */
        read_file("input/day18.txt")
    }

    fn run(&self, input: &str) -> String {
        let re = Regex::new(r"(\D) (\d+) \(#(.+)\)").unwrap();
        let steps: Vec<Step> = input
            .lines()
            .map(str::trim)
            .map(|s| match re.captures(s).map(|c| c.extract()) {
                Some((_, [dir, count, code])) => Step::new(
                    dir.parse().unwrap(),
                    count.parse::<u32>().unwrap(),
                    code.to_owned(),
                ),
                None => panic!("{}", s),
            })
            .collect();
        // using Pick's theoreom: https://www.reddit.com/r/adventofcode/comments/18lj7wx/comment/kdz5a7v/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

        let mut area: i64 = 0;
        let mut perim: i64 = 0;
        let (mut x_1, mut y_1): (i64, i64) = (0, 0);
        for i in 0..steps.len() {
            println!("processing step: {:?}", steps[i]);
            let (x_2, y_2): (i64, i64) = match steps[i].dir {
                Dir::L => (x_1 - steps[i].count as i64, y_1),
                Dir::R => (x_1 + steps[i].count as i64, y_1),
                Dir::U => (x_1, y_1 - steps[i].count as i64),
                Dir::D => (x_1, y_1 + steps[i].count as i64),
            };
            area += x_1 * y_2 - x_2 * y_1;
            println!("added {} to area", x_1 * y_2 - x_2 * y_1);
            perim += (x_2 - x_1).abs() + (y_2 - y_1).abs();
            (x_1, y_1) = (x_2, y_2);
            println!("moved to {:?}", (x_1, y_1));
        }
        println!("area: {}", area / 2);
        format!("{}", (area / 2) + (perim / 2) + 1)
    }
}
struct Part2;

impl Part2 {
    fn decode(code: &str) -> (Dir, i64) {
        (
            match code.chars().skip(5).next().unwrap() {
                '0' => Dir::R,
                '1' => Dir::D,
                '2' => Dir::L,
                '3' => Dir::U,
                _ => panic!(),
            },
            i64::from_str_radix(code.get(0..5).unwrap(), 16).unwrap(),
        )
    }
}
impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        /*
        Ok("R 6 (#70c710)
         D 5 (#0dc571)
         L 2 (#5713f0)
         D 2 (#d2c081)
         R 2 (#59c680)
         D 2 (#411b91)
         L 5 (#8ceee2)
         U 2 (#caa173)
         L 1 (#1b58a2)
         U 2 (#caa171)
         R 2 (#7807d2)
         U 3 (#a77fa3)
         L 2 (#015232)
         U 2 (#7a21e3)")
        */
        read_file("input/day18.txt")
    }

    fn run(&self, input: &str) -> String {
        let re = Regex::new(r"(\D) (\d+) \(#(.+)\)").unwrap();
        let steps: Vec<Step> = input
            .lines()
            .map(str::trim)
            .map(|s| match re.captures(s).map(|c| c.extract()) {
                Some((_, [dir, count, code])) => Step::new(
                    dir.parse().unwrap(),
                    count.parse::<u32>().unwrap(),
                    code.to_owned(),
                ),
                None => panic!("{}", s),
            })
            .collect();
        // using Pick's theoreom: https://www.reddit.com/r/adventofcode/comments/18lj7wx/comment/kdz5a7v/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

        let mut area: i64 = 0;
        let mut perim: i64 = 0;
        let (mut x_1, mut y_1): (i64, i64) = (0, 0);
        for i in 0..steps.len() {
            println!("processing step: {:?}", steps[i]);
            let (dir, count) = Part2::decode(&steps[i].code);
            let (x_2, y_2): (i64, i64) = match dir {
                Dir::L => (x_1 - count as i64, y_1),
                Dir::R => (x_1 + count as i64, y_1),
                Dir::U => (x_1, y_1 - count as i64),
                Dir::D => (x_1, y_1 + count as i64),
            };
            area += x_1 * y_2 - x_2 * y_1;
            println!("added {} to area", x_1 * y_2 - x_2 * y_1);
            perim += (x_2 - x_1).abs() + (y_2 - y_1).abs();
            (x_1, y_1) = (x_2, y_2);
            println!("moved to {:?}", (x_1, y_1));
        }
        println!("area: {}", area / 2);
        format!("{}", (area / 2) + (perim / 2) + 1)
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
    fn test_part2() {
        println!("{}", Part2::run(&Part2, Part2::input_data(&Part2).unwrap()));
    }
}
