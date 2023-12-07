use crate::{read_file, PuzzleRun};
use bumpalo::{collections::Vec, Bump};
use itertools::Itertools;
use num::integer::lcm;

use regex::Regex;
use std::collections::HashMap;

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

struct State<'a> {
    label: &'a str,
    right: Option<usize>,
    left: Option<usize>,
}

impl<'a> State<'a> {
    fn new(label: &'a str) -> Self {
        Self {
            label,
            right: None,
            left: None,
        }
    }
}
enum Dir {
    Left,
    Right,
}

impl From<u8> for Dir {
    fn from(s: u8) -> Self {
        match s {
            b'L' => Dir::Left,
            b'R' => Dir::Right,
            _ => panic!("bad instruction"),
        }
    }
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day8.txt")
        /*
        Ok("RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)")
        */
        /*
        Ok("LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)")
        */
    }

    fn run(&self, input: &str) -> String {
        let mut lines = input.lines();
        let instructions: std::vec::Vec<Dir> =
            lines.next().unwrap().bytes().map(|i| i.into()).collect();
        lines.next();
        let bump = Bump::new();
        let mut states = Vec::new_in(&bump);

        let mut tmp_map: HashMap<&str, (&str, &str)> = Default::default();
        let regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        for line in lines.map(|s| s.trim()) {
            match regex.captures(line).map(|c| c.extract()) {
                Some((_, [label, left, right])) => {
                    states.push(State::new(label));
                    tmp_map.insert(label, (left, right));
                }
                None => panic!("{}", line),
            }
        }
        states.sort_by_cached_key(|k| k.label);

        for (label, (left, right)) in tmp_map.into_iter() {
            let state_idx = states.binary_search_by_key(&label, |k| k.label).unwrap();
            states[state_idx].left = Some(states.binary_search_by_key(&left, |k| k.label).unwrap());
            states[state_idx].right =
                Some(states.binary_search_by_key(&right, |k| k.label).unwrap());
        }

        let mut current_state = states.binary_search_by_key(&"AAA", |k| k.label).unwrap();
        let mut moves: u32 = 0;

        while states[current_state].label != "ZZZ" {
            for i in instructions.iter() {
                moves += 1;
                match i {
                    Dir::Left => {
                        current_state = states[current_state].left.unwrap();
                    }
                    Dir::Right => {
                        current_state = states[current_state].right.unwrap();
                    }
                }
                if states[current_state].label == "ZZZ" {
                    break;
                }
            }
        }
        format!("{}", moves)
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day8.txt")

        /*         Ok("LR

               11A = (11B, XXX)
               11B = (XXX, 11Z)
               11Z = (11B, XXX)
               22A = (22B, XXX)
               22B = (22C, 22C)
               22C = (22Z, 22Z)
               22Z = (22B, 22B)
               XXX = (XXX, XXX)")
        */
    }

    fn run(&self, input: &str) -> String {
        let mut lines = input.lines();
        let instructions: std::vec::Vec<Dir> =
            lines.next().unwrap().bytes().map(|i| i.into()).collect();
        lines.next();
        let bump = Bump::new();
        let mut states = Vec::new_in(&bump);

        let mut tmp_map: HashMap<&str, (&str, &str)> = Default::default();
        let regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        for line in lines.map(|s| s.trim()) {
            match regex.captures(line).map(|c| c.extract()) {
                Some((_, [label, left, right])) => {
                    states.push(State::new(label));
                    tmp_map.insert(label, (left, right));
                }
                None => panic!("{}", line),
            }
        }
        states.sort_by_cached_key(|k| k.label);

        for (label, (left, right)) in tmp_map.into_iter() {
            let state_idx = states.binary_search_by_key(&label, |k| k.label).unwrap();
            states[state_idx].left = Some(states.binary_search_by_key(&left, |k| k.label).unwrap());
            states[state_idx].right =
                Some(states.binary_search_by_key(&right, |k| k.label).unwrap());
        }

        let mut currents: std::vec::Vec<usize> = states
            .iter()
            .enumerate()
            .filter(|(idx, state)| state.label.ends_with('A'))
            .map(|(idx, state)| idx)
            .collect();

        println!("starts: {:?}", currents);

        let mut cycles: std::vec::Vec<u32> = std::vec::Vec::with_capacity(currents.len());
        let starts = currents.clone();

        for i in 0..currents.len() {
            let mut moves: u32 = 0;
            for instruction in instructions.iter().cycle() {
                moves += 1;
                match instruction {
                    Dir::Left => {
                        currents[i] = states[currents[i]].left.unwrap();
                    }
                    Dir::Right => {
                        currents[i] = states[currents[i]].right.unwrap();
                    }
                }
                if states[currents[i]].label.ends_with('Z') {
                    println!(
                        "seq starting with {} ended at {} after {} steps",
                        states[starts[i]].label, states[currents[i]].label, moves,
                    );
                    cycles.push(moves);
                    println!("{} steps", moves);
                    break;
                }
            }
        }
        println!("{:?}", cycles);
        let mut ans: u64 = 1;
        while cycles.len() > 0 {
            ans = lcm(ans, cycles.pop().unwrap() as u64);
            println!("lcm: {}", ans);
        }
        format!("{}", ans)
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
