use crate::{read_file, PuzzleRun};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

fn parse(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve_part1(input: Vec<i32>) -> i32 {
    let mut diffs = vec![input];
    while !diffs[diffs.len() - 1].iter().all(|&i| i == 0) {
        let mut diff = Vec::with_capacity(diffs[0].len());
        for w in diffs.last().unwrap().windows(2) {
            diff.push(w[1] - w[0]);
        }
        diffs.push(diff);
    }
    while diffs.len() > 1 {
        let diff = diffs.pop().unwrap();
        let l = diffs.len();
        let dr: &mut Vec<i32> = &mut diffs[l - 1];
        dr.push(dr.last().unwrap() + diff.last().unwrap());
    }
    *diffs[0].last().unwrap()
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day9.txt")
    }

    fn run(&self, input: &str) -> String {
        format!(
            "{}",
            input
                .lines()
                .map(|line| solve_part1(parse(line.trim())))
                .sum::<i32>()
        )
    }
}

fn solve_part2(input: Vec<i32>) -> i32 {
    let mut diffs = vec![input];
    while !diffs[diffs.len() - 1].iter().all(|&i| i == 0) {
        let mut diff = Vec::with_capacity(diffs[0].len());
        for w in diffs.last().unwrap().windows(2) {
            diff.push(w[1] - w[0]);
        }
        diffs.push(diff);
    }
    while diffs.len() > 1 {
        let diff = diffs.pop().unwrap();
        let l = diffs.len();
        let dr: &mut Vec<i32> = &mut diffs[l - 1];
        dr.insert(0, dr.first().unwrap() - diff.first().unwrap());
    }
    *diffs[0].first().unwrap()
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day9.txt")
        /* Ok("0 3 6 9 12 15
                  1 3 6 10 15 21
                  10 13 16 21 30 45")
        */
    }

    fn run(&self, input: &str) -> String {
        format!(
            "{}",
            input
                .lines()
                .map(|line| solve_part2(parse(line.trim())))
                .sum::<i32>()
        )
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
