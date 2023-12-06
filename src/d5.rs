use crate::{read_file, PuzzleRun};
use itertools::Itertools;
use rayon::prelude::*;

use std::collections::HashMap;

use regex::Regex;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

#[derive(Debug)]
struct MapRange {
    src_base: u64,
    dest_base: u64,
    len: u64,
}

impl MapRange {
    fn new(src_base: u64, dest_base: u64, len: u64) -> Self {
        Self {
            src_base,
            dest_base,
            len,
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn new() -> Self {
        Self {
            ranges: Default::default(),
        }
    }

    fn add_range(&mut self, mut v: impl Iterator<Item = u64>) {
        let Some(dest_start) = v.next() else {
            panic!();
        };
        let Some(src_start) = v.next() else {
            panic!();
        };
        let Some(len) = v.next() else {
            panic!();
        };
        self.ranges.push(MapRange::new(src_start, dest_start, len));
        self.ranges.sort_unstable_by_key(|r| r.src_base);
    }
}

struct MapSeq {
    maps: Vec<Map>,
}

impl MapSeq {
    fn new(maps: Vec<Map>) -> Self {
        MapSeq { maps }
    }

    fn apply(&self, n: u64) -> u64 {
        let mut t = n;
        for m in &self.maps {
            'range: for r in &m.ranges {
                if t >= r.src_base && t < r.src_base + r.len {
                    t = r.dest_base + (t - r.src_base);
                    break 'range;
                }
            }
        }
        //        println!("mapping {} to {}", n, t);
        t
    }

    fn maybe_mins(&self, start: u64, len: u64) -> Vec<u64> {
        let mut v = vec![start];
        for r in &self.maps[0].ranges {
            if r.src_base > start && r.src_base < r.src_base + len {
                v.push(r.src_base);
            }
        }
        v
    }
}

fn build_maps(input: &str) -> (Vec<u64>, MapSeq) {
    let seeds_re = Regex::new(r"seeds: (.+)").unwrap();
    let map_header_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
    let mut seeds: Option<Vec<u64>> = None;
    let mut lines = input.lines().map(str::trim);
    let mut maps: HashMap<String, Map> = Default::default();

    loop {
        let Some(line) = lines.next() else {
            break;
        };

        if line.len() == 0 {
            continue;
        }
        if let Some((_, [seed_str])) = seeds_re.captures(line).map(|c| c.extract()) {
            seeds.replace(
                seed_str
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            );
        } else if let Some((_, [source_str, dest_str])) =
            map_header_re.captures(line).map(|c| c.extract())
        {
            let mut map = Map::new();
            loop {
                let Some(line) = lines.next() else {
                    break;
                };
                if line.trim().len() == 0 {
                    break;
                }
                map.add_range(line.split_whitespace().map(|s| s.parse().unwrap()));
            }
            maps.insert(source_str.to_string(), map);
        }
    }
    let seeds = seeds.unwrap();
    let order = [
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
    ];

    let map_layers = MapSeq::new(order.iter().map(|s| maps.remove(*s).unwrap()).collect());
    (seeds, map_layers)
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day5.txt")
    }

    fn run(&self, input: &str) -> String {
        let (seeds, map_layers) = build_maps(input);
        format!(
            "{}",
            seeds.iter().map(|s| map_layers.apply(*s)).min().unwrap()
        )
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day5.txt")
        /* Ok("seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4")
        */
    }

    fn run(&self, input: &str) -> String {
        let (seeds, map_layers) = build_maps(input);
        let mut lowest_location = u64::MAX;

        for (start, len) in seeds.iter().tuples() {
            println!("starting range from {} len {}", start, len);
            let this_min = (*start..(start + len))
                .into_par_iter()
                .map(|s| map_layers.apply(s))
                .min()
                .unwrap();
            if this_min < lowest_location {
                lowest_location = this_min;
            }
            /*
            for maybe_min in map_layers.maybe_mins(*start, *len) {
                let l = map_layers.apply(maybe_min);
                if l < lowest_location {
                    lowest_location = l;
                }
            }
            */
        }
        format!("{}", lowest_location)
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
