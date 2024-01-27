use crate::{read_file, PuzzleRun};
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

#[derive(Debug)]
struct Record {
    line: String,
    counts: Vec<u8>,
    total_broken: u32,
    known_broken: u32,
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
        let need_to_place: u32 = (r.total_broken - r.known_broken);

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
    fn new(line: String, counts: Vec<u8>, total_broken: u32, known_broken: u32) -> Self {
        Self {
            line,
            counts,
            total_broken,
            known_broken,
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
            .into_iter()
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

        for mut required_len in &self.counts {
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
            .map(|s| Record::parse(s))
            .map(|r| r.and_then(|r| Part1::count_choices(&r)))
            .sum();

        format!("{}", counts.unwrap())
    }
}

struct Part2;

trait Stateful {
    fn next(&self) -> impl Stateful;
}

impl Stateful for &str {
    fn next(&self) -> impl Stateful {
        &self[1..]
    }
}

impl Stateful for String {
    fn next(&self) -> impl Stateful {
        &<std::string::String as AsRef<str>>::as_ref(self)[1..]
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum GroupState {
    Outside,
    Inside(u8),
}

#[derive(Debug, Eq)]
struct StateInContext<'a> {
    state: GroupState,
    current: Condition,
    remaining: &'a str,
    groups: &'a [u8],
    seen: String,
}

impl<'a> Hash for StateInContext<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.state.hash(state);
        self.current.hash(state);
        self.remaining.hash(state);
        self.groups.hash(state);
    }
}

impl<'a> PartialEq for StateInContext<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.current == other.current
            && self.remaining == other.remaining
            && self.groups == other.groups
    }
}

#[derive(Debug)]
enum StateTransition<'a> {
    Single(StateInContext<'a>),
    Alternate(StateInContext<'a>, StateInContext<'a>),
    Valid,
    Invalid,
}

impl<'a> StateInContext<'a> {
    fn start(pat: &'a str, groups: &'a [u8]) -> Self {
        let (first, rest) = pat.split_at(1);
        let mut seen = String::with_capacity(pat.len());
        Self {
            state: GroupState::Outside,
            current: Condition::try_from(first).unwrap(),
            remaining: rest,
            groups,
            seen,
        }
    }

    fn next(self) -> StateTransition<'a> {
        if matches!(self.current, Condition::Unknown) {
            return StateTransition::Alternate(
                Self {
                    state: self.state,
                    current: Condition::Damaged,
                    remaining: self.remaining,
                    groups: self.groups,
                    seen: self.seen.clone(),
                },
                Self {
                    state: self.state,
                    current: Condition::Operational,
                    remaining: self.remaining,
                    groups: self.groups,
                    seen: self.seen.clone(),
                },
            );
        }
        match (self.state, self.remaining.len()) {
            (GroupState::Outside, 0) => {
                if self.groups.is_empty() && matches!(self.current, Condition::Operational) {
                    let mut seen = self.seen.clone();
                    seen.push(match self.current {
                        Condition::Damaged => '#',
                        Condition::Operational => '.',
                        Condition::Unknown => panic!(),
                    });
                    //println!("valid from outside (l: {}):\t{}", seen.len(), seen);
                    StateTransition::Valid
                } else if self.groups.len() == 1
                    && self.groups[0] == 1
                    && matches!(self.current, Condition::Damaged)
                {
                    let mut seen = self.seen.clone();
                    seen.push(match self.current {
                        Condition::Damaged => '#',
                        Condition::Operational => '.',
                        Condition::Unknown => panic!(),
                    });
                    //println!("valid from outside (1) (l: {}):\t{}", seen.len(), seen);
                    StateTransition::Valid
                } else {
                    StateTransition::Invalid
                }
            }
            (GroupState::Outside, _) => {
                let (first, remaining) = self.remaining.split_at(1);
                match self.current {
                    Condition::Operational => {
                        let mut seen = self.seen.clone();
                        seen.push('.');
                        StateTransition::Single(Self {
                            state: GroupState::Outside,
                            current: Condition::try_from(first).unwrap(),
                            remaining,
                            groups: self.groups,
                            seen: seen,
                        })
                    }
                    Condition::Damaged if self.groups.is_empty() => StateTransition::Invalid,
                    Condition::Damaged => {
                        let mut seen = self.seen.clone();
                        seen.push('#');
                        StateTransition::Single(Self {
                            state: GroupState::Inside(
                                self.groups[0] - 1, /* subtract the one seen */
                            ),
                            current: Condition::try_from(first).unwrap(),
                            remaining,
                            groups: &self.groups[1..],
                            seen,
                        })
                    }
                    Condition::Unknown => panic!(),
                }
            }
            (GroupState::Inside(l), 0) => {
                if l == 0
                    && self.groups.is_empty()
                    && matches!(self.current, Condition::Operational)
                {
                    let mut seen = self.seen.clone();
                    seen.push(match self.current {
                        Condition::Damaged => '#',
                        Condition::Operational => '.',
                        Condition::Unknown => panic!(),
                    });
                    //println!("valid from inside (0) (l: {}):\t{}", seen.len(), seen);
                    StateTransition::Valid
                } else if l == 1
                    && matches!(self.current, Condition::Damaged)
                    && self.groups.is_empty()
                {
                    let mut seen = self.seen.clone();
                    seen.push(match self.current {
                        Condition::Damaged => '#',
                        Condition::Operational => '.',
                        Condition::Unknown => panic!(),
                    });
                    //println!("valid from inside (1) (l: {}):\t{}", seen.len(), seen);
                    StateTransition::Valid
                } else {
                    StateTransition::Invalid
                }
            }
            (GroupState::Inside(l), _) => {
                let (first, remaining) = self.remaining.split_at(1);
                match self.current {
                    Condition::Operational if l == 0 => {
                        let mut seen = self.seen.clone();
                        seen.push('.');
                        StateTransition::Single(Self {
                            state: GroupState::Outside,
                            current: Condition::try_from(first).unwrap(),
                            remaining,
                            groups: self.groups,
                            seen,
                        })
                    }
                    Condition::Operational => StateTransition::Invalid,
                    Condition::Damaged if l > 0 => {
                        let mut seen = self.seen.clone();
                        seen.push('#');
                        StateTransition::Single(Self {
                            state: GroupState::Inside(l - 1),
                            current: Condition::try_from(first).unwrap(),
                            remaining,
                            groups: self.groups,
                            seen,
                        })
                    }
                    Condition::Damaged if l == 0 => StateTransition::Invalid,
                    Condition::Damaged => StateTransition::Invalid,
                    Condition::Unknown => panic!(),
                }
            }
        }
    }
}

impl Part2 {
    fn expand(s: &str) -> String {
        let mut new_s = String::new();
        let mut new_c = String::new();
        let (line, counts) = s.split_ascii_whitespace().next_tuple().unwrap();
        new_s.push_str(line);
        new_c.push_str(counts);
        for i in 0..4 {
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

    fn count_choices(r: &Record) -> u32 {
        use StateTransition::*;

        let mut states_to_visit: Vec<StateInContext> =
            vec![StateInContext::start(&r.line, &r.counts)];
        let mut completed: u32 = 0;
        //        let mut cache: HashSet<StateInContext> = Default::default();

        'root: while let Some(mut this_state) = states_to_visit.pop() {
            loop {
                match this_state.next() {
                    Valid => {
                        completed += 1;
                        continue 'root;
                    }
                    Invalid => continue 'root,
                    Single(next_state) => {
                        this_state = next_state;
                    }
                    Alternate(s1, s2) => {
                        states_to_visit.push(s1);
                        this_state = s2;
                    }
                }
            }
        }
        completed
    }
}

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day12.txt")
        /* Ok("???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1")
        */
    }

    fn run(&self, input: &str) -> String {
        let counts: Result<u32, &str> = input
            .lines()
            .map(str::trim)
            .map(|s| Part2::expand(s))
            .map(|s| Record::parse(&s))
            .map(|r| r.and_then(|r| Ok(Part2::count_choices(&r))))
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
        assert_eq!(Part1::count_choices(&r).unwrap(), 1);

        let r = Record::parse(".??..??...?##. 1,1,3").unwrap();
        assert_eq!(Part1::count_choices(&r).unwrap(), 4);

        let r = Record::parse("?#?#?#?#?#?#?#? 1,3,1,6").unwrap();
        assert_eq!(Part1::count_choices(&r).unwrap(), 1);

        let r = Record::parse("????.#...#... 4,1,1").unwrap();
        assert_eq!(Part1::count_choices(&r).unwrap(), 1);

        let r = Record::parse("????.######..#####. 1,6,5").unwrap();
        assert_eq!(Part1::count_choices(&r).unwrap(), 4);

        let r = Record::parse("?###???????? 3,2,1").unwrap();
        assert_eq!(Part1::count_choices(&r).unwrap(), 10);
    }

    #[test]
    fn test_part1() {
        println!("{}", Part1.run(Part1.input_data().unwrap()));
    }

    #[test]
    fn test_expand() {
        // let s = "#.#.####..#.### 1,1,3,1,1,3";

        let s = Part2::expand("???.### 1,1,3");
        let r = Record::parse(&s).unwrap();
        assert_eq!(Part2::count_choices(&r), 1);

        let s = Part2::expand(".??..??...?##. 1,1,3");
        let r = Record::parse(&s).unwrap();
        assert_eq!(Part2::count_choices(&r), 16384);

        let s = Part2::expand("?#?#?#?#?#?#?#? 1,3,1,6");
        let r = Record::parse(&s).unwrap();
        assert_eq!(Part2::count_choices(&r), 1);

        let s = Part2::expand("????.#...#... 4,1,1");
        let r = Record::parse(&s).unwrap();
        assert_eq!(Part2::count_choices(&r), 16);

        let s = Part2::expand("????.######..#####. 1,6,5");
        let r = Record::parse(&s).unwrap();
        assert_eq!(Part2::count_choices(&r), 2500);

        let s = Part2::expand("?###???????? 3,2,1");
        let r = Record::parse(&s).unwrap();
        assert_eq!(Part2::count_choices(&r), 506250);
    }
    #[test]
    fn test_part2() {
        println!("{}", Part2.run(Part2.input_data().unwrap()));
    }
}
