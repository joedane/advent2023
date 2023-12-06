use crate::{read_file, PuzzleRun};
use num::integer::sqrt;
use regex::Regex;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug)]
struct RaceData {
    time: u32,
    record: u32,
}

impl RaceData {
    fn new(time: u32, record: u32) -> Self {
        Self { time, record }
    }
}

fn parse_race_data(input: &str) -> Vec<RaceData> {
    let re = Regex::new(r"(Time:|Distance:)([\s\d]+)").unwrap();
    let mut lines = input.lines();
    let times: Vec<u32> = match re.captures(lines.next().unwrap()).map(|c| c.extract()) {
        Some((_, [t_or_d, nums])) if t_or_d == "Time:" => nums
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(),
        _ => panic!(),
    };
    let dists: Vec<u32> = match re.captures(lines.next().unwrap()).map(|c| c.extract()) {
        Some((_, [t_or_d, nums])) if t_or_d == "Distance:" => nums
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(),
        _ => panic!(),
    };
    assert_eq!(times.len(), dists.len());
    times
        .into_iter()
        .zip(dists.into_iter())
        .map(|(t, d)| RaceData::new(t, d))
        .collect()
}

fn winners(time: u32, record: u32) -> Vec<u32> {
    let mut v = vec![];
    for p in 0..time {
        let d = p * (time - p);
        if d > record {
            v.push(p)
        }
    }
    v
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day6.txt")
        /*
        Ok("Time:      7  15   30
        Distance:  9  40  200")
        */
    }

    fn run(&self, input: &str) -> String {
        let races = parse_race_data(input);
        format!(
            "{}",
            races
                .iter()
                .map(|r| winners(r.time, r.record).len())
                .reduce(|acc, v| acc * v)
                .unwrap()
        )
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day6.txt")
        /*
        Ok("Time:      7  15   30
        Distance:  9  40  200")
        */
    }

    fn run(&self, input: &str) -> String {
        let mut lines = input.lines();
        let line = lines.next().unwrap();
        let mut buf: Vec<char> = Default::default();
        for c in line.chars() {
            if c.is_ascii_digit() {
                buf.push(c);
            }
        }
        let time: u32 = buf.into_iter().collect::<String>().parse().unwrap();

        let line = lines.next().unwrap();
        let mut buf: Vec<char> = Default::default();
        for c in line.chars() {
            if c.is_ascii_digit() {
                buf.push(c);
            }
        }
        let record: u64 = buf.into_iter().collect::<String>().parse().unwrap();

        let D = sqrt(time as u64 * time as u64 - 4 * record as u64);
        let max = (time as u64 + D) / 2;
        let min = (time as u64 - D) / 2;

        format!("{}", max - min)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let input = Part1.input_data().unwrap();
        let re = Regex::new(r"(Time:|Distance:)([\s\d]+)").unwrap();
        let rd = parse_race_data(input);
        assert_eq!(rd.len(), 3);
        println!("{:?}", rd);
    }

    #[test]
    fn test_part1() {
        println!("{}", Part1.run(Part1.input_data().unwrap()));
    }

    #[test]
    fn test_part2() {
        println!("{}", Part2.run(Part2.input_data().unwrap()));
    }
}
