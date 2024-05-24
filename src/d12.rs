#![allow(dead_code)]
use crate::{read_file, PuzzleRun};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

#[derive(Debug)]
struct Record {
    line: String,
    counts: Vec<u8>,
    _total_broken: u32,
    _known_broken: u32,
}

impl Record {
    fn count_choices(&self) -> u32 {
        let mut cache: HashMap<CacheKey, u32> = Default::default();
        count_matches(
            self.line.as_str(),
            self.line.clone().into_bytes(),
            self.counts.clone(),
            false,
            String::new(),
            &mut cache,
        )
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<&str> for Condition {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "." => Ok(Self::Operational),
            "#" => Ok(Self::Damaged),
            "?" => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}

impl Part1 {
    fn count_choices(r: &Record) -> Result<usize, &'static str> {
        let indicies: Vec<usize> = r
            .line
            .chars()
            .enumerate()
            .filter_map(|(i, b)| if b == '?' { Some(i) } else { None })
            .collect();
        let need_to_place: u32 = r._total_broken - r._known_broken;

        Ok(indicies
            .iter()
            .cloned()
            .combinations(need_to_place as usize)
            //            .inspect(|i| println!("{:?}", i))
            .filter(|c| r.is_valid(c))
            .count())
    }
}
impl Record {
    fn new(line: String, counts: Vec<u8>, _total_broken: u32, _known_broken: u32) -> Self {
        Self {
            line,
            counts,
            _total_broken,
            _known_broken,
        }
    }

    fn parse(s: &str) -> Result<Self, &'static str> {
        let (line, counts) = s
            .split_ascii_whitespace()
            .next_tuple()
            .ok_or("parse error")?;

        let line: String = line.into();

        let counts: Vec<u8> = counts
            .split(',')
            .map(|s| s.parse::<u8>().or(Err("parse failure")))
            .collect::<Result<Vec<u8>, _>>()?;

        let total = counts.iter().map(|&c| c as u32).sum();
        let known: u32 = line
            .chars()
            .filter(|b| *b == '#')
            .count()
            .try_into()
            .unwrap();
        Ok(Record::new(line, counts, total, known))
    }

    fn is_valid(&self, choices: &Vec<usize>) -> bool {
        let mut this_choice: String = self.line.to_string();

        for c in choices {
            unsafe {
                this_choice.as_mut_vec()[*c] = b'#';
            }
        }

        let mut next_i: usize = 0;

        for required_len in &self.counts {
            while next_i < this_choice.len() && this_choice.as_bytes()[next_i] != b'#' {
                next_i += 1;
            }
            if next_i == this_choice.len() {
                return false;
            }
            let mut this_run_len = 1u8;
            next_i += 1;
            while next_i < this_choice.len() && this_choice.as_bytes()[next_i] == b'#' {
                this_run_len += 1;
                next_i += 1;
            }
            if this_run_len == *required_len {
                continue;
            } else {
                return false;
            }
        }
        true
    }
}

struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day12.txt")
    }

    fn run(&self, input: &str) -> String {
        let counts: Result<usize, &str> = input
            .lines()
            .map(str::trim)
            .map(Record::parse)
            .map(|r| r.and_then(|r| Part1::count_choices(&r)))
            .sum();

        format!("{}", counts.unwrap())
    }
}

struct Part2;

fn print_seen(seen: &str, mark: &str) {
    println!("{seen} ({mark})");
}

#[derive(PartialEq, Eq, Hash)]
struct CacheKey {
    remaining: Box<[u8]>,
    groups: Box<[u8]>,
}

impl CacheKey {
    fn new(remaining: &[u8], groups: &[u8]) -> Self {
        CacheKey {
            remaining: remaining.into(),
            groups: groups.into(),
        }
    }
}

fn count_matches(
    original: &str,
    remaining: Vec<u8>,
    groups: Vec<u8>,
    in_run: bool,
    seen: String,
    cache: &mut HashMap<CacheKey, u32>,
) -> u32 {
    let k = CacheKey::new(&remaining, &groups);
    match cache.get(&k) {
        Some(n) => *n,
        None => {
            let n = _count_matches(original, remaining, groups, in_run, seen.clone(), cache);
            cache.insert(k, n);
            n
        }
    }
}

fn _count_matches(
    original: &str,
    remaining: Vec<u8>,
    groups: Vec<u8>,
    in_run: bool,
    seen: String,
    cache: &mut HashMap<CacheKey, u32>,
) -> u32 {
    let _original_len = original.len();
    let _seen_len = seen.len();

    //println!("{this_str}");
    if original.len() != remaining.len() + seen.len() {
        panic!();
    }
    if remaining.is_empty() {
        if groups.is_empty() || groups[0] == 0 {
            print_seen(&seen, "A");
            1
        } else {
            0
        }
    } else {
        let (first, rest) = remaining.split_at(1);
        if first[0] == b'.' {
            if in_run {
                0
            } else {
                let mut seen = seen.clone();
                seen.push(first[0].into());
                count_matches(original, rest.to_vec(), groups, false, seen, cache)
            }
        } else if first[0] == b'#' {
            let mut seen = seen.clone();
            seen.push(first[0].into());
            if groups.is_empty() {
                0
            } else if groups[0] == 0 {
                panic!()
            } else if groups[0] == 1 {
                if rest.is_empty() {
                    if groups.len() == 1 {
                        print_seen(&seen, "B");
                        1
                    } else {
                        0
                    }
                } else if rest.len() == 1 {
                    if rest[0] == b'#' {
                        // run too long
                        0
                    } else if groups.len() == 1 {
                        seen.push_str(std::str::from_utf8(rest).unwrap());
                        print_seen(&seen, "C");
                        1
                    } else {
                        0
                    }
                } else if rest[0] == b'#' {
                    // run too long
                    0
                } else {
                    seen.push(rest[0].into());
                    count_matches(
                        original,
                        rest[1..].to_vec(),
                        groups[1..].to_vec(),
                        false,
                        seen,
                        cache,
                    )
                }
            } else {
                let mut new_groups = groups.clone();
                new_groups[0] -= 1;
                count_matches(original, rest.to_vec(), new_groups, true, seen, cache)
            }
        } else if first[0] == b'?' {
            let mut new_pat: Vec<u8> = remaining.clone();
            new_pat[0] = b'.';
            let c = count_matches(
                original,
                new_pat,
                groups.clone(),
                in_run,
                seen.clone(),
                cache,
            );
            let mut new_pat: Vec<u8> = remaining.clone();
            new_pat[0] = b'#';
            c + count_matches(original, new_pat, groups, in_run, seen, cache)
        } else {
            panic!()
        }
    }
}

fn check(pat: &str, groups: Vec<u8>) -> bool {
    _check(pat, groups, false)
}

fn _check(pat: &str, groups: Vec<u8>, in_run: bool) -> bool {
    if pat.is_empty() {
        return groups.is_empty();
    }
    match (pat.chars().next().unwrap(), in_run) {
        ('.', true) => false,
        ('.', false) => _check(&pat[1..], groups, false),
        ('#', _) if groups.is_empty() => false,
        ('#', _) if groups[0] == 1 => {
            if pat.len() == 1 {
                true
            } else if pat.chars().nth(1).unwrap() == '.' {
                _check(&pat[1..], groups[1..].to_vec(), false)
            } else {
                false
            }
        }
        ('#', _) => {
            let mut g = groups.clone();
            g[0] -= 1;
            _check(&pat[1..], g, true)
        }
        _ => panic!(),
    }
}

impl Part2 {
    fn expand(s: &str) -> String {
        let mut new_s = String::new();
        let mut new_c = String::new();
        let (line, counts) = s.split_ascii_whitespace().next_tuple().unwrap();
        new_s.push_str(line);
        new_c.push_str(counts);
        for _ in 0..4 {
            new_s.push('?');
            new_s.push_str(line);
            new_c.push(',');
            new_c.push_str(counts);
        }
        let mut r = String::new();
        r.push_str(&new_s);
        r.push(' ');
        r.push_str(&new_c);

        r
    }
}

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        //read_file("input/day12.txt")

        Ok("???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1")
    }

    fn run(&self, input: &str) -> String {
        let mut i = 0;
        let counts: Result<u32, &str> = input
            .lines()
            .inspect(move |l| {
                println!("{}: testing {}", i, l);
                i += 1;
            })
            .par_bridge()
            .map(str::trim)
            .map(Part2::expand)
            .map(|s| Record::parse(&s))
            .map(|r| r.map(|r| r.count_choices()))
            .sum();

        format!("{}", counts.unwrap())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let r = Record::parse("???.### 1,1,3").unwrap();
        assert_eq!(r.count_choices(), 1);

        let r = Record::parse(".??..??...?##. 1,1,3").unwrap();
        assert_eq!(r.count_choices(), 4);

        let r = Record::parse("?#?#?#?#?#?#?#? 1,3,1,6").unwrap();
        assert_eq!(r.count_choices(), 1);

        let r = Record::parse("????.#...#... 4,1,1").unwrap();
        assert_eq!(r.count_choices(), 1);

        let r = Record::parse("????.######..#####. 1,6,5").unwrap();
        assert_eq!(r.count_choices(), 4);

        let r = Record::parse("?###???????? 3,2,1").unwrap();
        assert_eq!(r.count_choices(), 10);
    }

    #[test]
    fn test_part1() {
        println!("{}", Part1.run(Part1.input_data().unwrap()));
    }

    #[test]
    fn test_expand() {
        //let s = "???.###????.### 1,1,3,1,1,3";
        /*
        let s = Part2::expand("???.### 1,1,3");
        let r = Record::parse(&s).unwrap();
        assert_eq!(r.count_choices(), 1);
        */
        //let s = "..#...#....##.#..#...#....##.#..#...#....##.#..#...#....##.#..#...#...###. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";
        //let s = ".??..??...?##. 1,1,3";
        let s = Part2::expand(".??..??...?##. 1,1,3");
        println!("{s}");
        let r = Record::parse(&s).unwrap();
        assert_eq!(r.count_choices(), 16384);

        let s = Part2::expand("?#?#?#?#?#?#?#? 1,3,1,6");
        let r = Record::parse(&s).unwrap();
        assert_eq!(r.count_choices(), 1);

        let s = Part2::expand("????.#...#... 4,1,1");
        let r = Record::parse(&s).unwrap();
        assert_eq!(r.count_choices(), 16);

        let s = Part2::expand("????.######..#####. 1,6,5");
        let r = Record::parse(&s).unwrap();
        assert_eq!(r.count_choices(), 2500);

        let s = Part2::expand("?###???????? 3,2,1");
        //let s = ".###?....##?#?###?....##?#?###?....##?#?###?....##?#?###?....##? 3,2,1,3,2,1,3,2,1,3,2,1,3,2,1";
        let r = Record::parse(&s).unwrap();
        assert_eq!(r.count_choices(), 506250);
    }
    #[test]
    fn test_part2() {
        println!("{}", Part2.run(Part2.input_data().unwrap()));
    }

    #[test]
    fn test_check() {
        let strs = [
            ("#.#.### 1,1,3", true),
            (".#...#....###. 1,1,3", true),
            (".#.###.#.###### 1,3,1,6", true),
            ("####.#...#... 4,1,1", true),
            ("#....######..#####. 1,6,5", true),
            (".###.##....# 3,2,1", true),
            ("..###....#... 3,2,1", false),
        ];
        for (s, b) in strs {
            let r = Record::parse(s).unwrap();
            assert_eq!(check(r.line.as_str(), r.counts), b)
        }
    }

    #[test]
    fn test_foo() {
        use std::io::BufRead;

        let file = std::fs::File::open("test.out").unwrap();
        for line in std::io::BufReader::new(file).lines() {
            let line = line.unwrap();
            let (pat, _code) = line.split_at(line.find(' ').unwrap());
            println!("{pat}");
            let mut pat = pat.replace('?', ".");
            pat.push(' ');
            pat.push_str("3,2,1,3,2,1,3,2,1,3,2,1,3,2,1");
            let r = Record::parse(pat.as_str()).unwrap();
            if !check(&r.line, r.counts) {
                panic!("failed: {}", r.line);
            }
        }
    }
}
