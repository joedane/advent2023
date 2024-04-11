use std::fmt::{Display, Write};

use super::{read_file, PuzzleRun};
use itertools::Itertools;
use regex::bytes::Regex;

pub(crate) fn get_runs() -> std::vec::Vec<std::boxed::Box<dyn PuzzleRun>> {
    vec![std::boxed::Box::new(Part1)]
}

struct Part1;

impl Part1 {
    fn compute_hash(input: &str) -> usize {
        let mut current: u16 = 0;

        for b in input.as_bytes().iter() {
            if b.is_ascii_whitespace() {
                continue;
            }
            current = ((current + *b as u16) * 17) % 256;
        }
        current as usize
    }
}
impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day15.txt")
        //Ok("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
    }

    fn run(&self, input: &str) -> String {
        format!(
            "{}",
            input.split(',').map(Part1::compute_hash).sum::<usize>()
        )
    }
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_len: u8,
}

impl<'a> Lens<'a> {
    fn new(label: &'a str, focal_len: u8) -> Self {
        Lens { label, focal_len }
    }
}

impl<'a> Display for Lens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_len)
    }
}
struct BoxEntry<'a> {
    idx: u8,
    lenses: Vec<Lens<'a>>,
}

impl<'a> Display for BoxEntry<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = format!("Box {}:", self.idx);
        let s = self.lenses.iter().map(ToString::to_string).join(" ");
        buf.push_str(&s);
        f.write_str(&buf)
    }
}

impl<'a> BoxEntry<'a> {
    fn new(idx: u8) -> Self {
        Self {
            idx,
            lenses: Default::default(),
        }
    }
    fn remove(&mut self, label: &str) {
        if let Some(i) = self.lenses.iter().position(|e| e.label == label) {
            self.lenses.remove(i);
        }
    }

    fn insert(&mut self, label: &'a str, focal_len: u8) {
        if let Some(i) = self.lenses.iter().position(|e| e.label == label) {
            self.lenses[i].focal_len = focal_len;
        } else {
            self.lenses.push(Lens::new(label, focal_len));
        }
    }

    fn power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(idx, l)| (self.idx as usize + 1) * (idx + 1) * l.focal_len as usize)
            .sum()
    }
}
struct Boxes<'a> {
    boxes: [BoxEntry<'a>; 256],
}

impl<'a> Boxes<'a> {
    fn new() -> Self {
        Self {
            boxes: std::array::from_fn(|i| BoxEntry::new(i.try_into().unwrap())),
        }
    }
    fn remove(&mut self, hash: usize, label: &str) {
        if hash > 255 {
            panic!();
        }
        self.boxes[hash].remove(label);
    }

    fn insert(&mut self, hash: usize, label: &'a str, focal_len: u8) {
        if hash > 255 {
            panic!();
        }
        self.boxes[hash].insert(label, focal_len);
    }

    fn print_state(&self) {
        for i in 0..256 {
            if !self.boxes[i].lenses.is_empty() {
                println!("{}", self.boxes[i]);
            }
        }
    }

    fn power(&self) -> usize {
        self.boxes.iter().map(|b| b.power()).sum()
    }
}
struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day15.txt")
        //Ok("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
    }

    fn run(&self, input: &str) -> String {
        let mut boxes = Boxes::new();

        let by_commas = Regex::new(r",").unwrap();
        let label_re = Regex::new(r"([^=-]+)(-|=\d)").unwrap();
        for instr in by_commas.split(input.as_bytes()) {
            let Some((_, [label, cmd])) = label_re.captures(instr).map(|c| c.extract()) else {
                panic!();
            };
            let label = std::str::from_utf8(label).unwrap();
            let hash = Part1::compute_hash(label);
            if cmd == &[b'-'] {
                boxes.remove(hash, label)
            } else if cmd[0] == b'=' {
                boxes.insert(hash, label, cmd[1] - 48)
            } else {
                panic!("bad command '{}'", std::str::from_utf8(cmd).unwrap());
            }
            //println!("after running '{}'", std::str::from_utf8(instr).unwrap());
            //boxes.print_state();
        }
        format!("{}", boxes.power())
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
