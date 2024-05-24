use regex::Regex;
use std::{collections::HashMap, str::FromStr};

use super::{read_file, PuzzleRun};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

enum Op {
    Gt,
    Lt,
}

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err("too long")
        } else {
            match s.chars().next().unwrap() {
                'x' | 'X' => Ok(Category::X),
                'm' | 'M' => Ok(Category::M),
                'a' | 'A' => Ok(Category::A),
                's' | 'S' => Ok(Category::S),
                _ => Err("invalid category"),
            }
        }
    }
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err("too long")
        } else {
            match s.chars().next().unwrap() {
                '>' => Ok(Op::Gt),
                '<' => Ok(Op::Lt),
                _ => Err("invalid"),
            }
        }
    }
}

struct WorkflowDesc {
    name: String,
    rules: Vec<RuleDesc>,
}

impl WorkflowDesc {
    fn new(name: &str, rules: Vec<RuleDesc>) -> Self {
        Self {
            name: name.to_owned(),
            rules,
        }
    }
}

struct RuleDesc {
    test: Option<String>,
    target: String,
}

struct Rule<'a> {
    category: Category,
    op: Op,
    val: i64,
    target: &'a Workflow<'a>,
}

impl<'a> Rule<'a> {
    fn new(category: Category, op: Op, val: i64, target: &'a Workflow) -> Self {
        Self {
            category,
            op,
            val,
            target,
        }
    }

    fn extract_val(&self, part: Part) -> i64 {
        match self.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        }
    }
    fn run(&self, part: Part) -> TestResult<'a> {
        let val = self.extract_val(part);
        match self.op {
            Op::Gt => {
                if val > self.val {
                    TestResult::Jump(self.target)
                } else {
                    TestResult::NoMatch
                }
            },
            Op::Lt => {
                if val < self.val {
                    TestResult::Jump(self.target)
                } else {
                    TestResult::NoMatch
                }

            }
        }
    }
}

struct Workflow<'a> {
    name: String,
    rules: Vec<Rule<'a>>,
}
enum TestResult<'a> {
    Accept,
    Reject,
    NoMatch,
    Jump(&'a Workflow<'a>),
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Debug)]
enum PartParseError {
    NumParseError(String),
    InvalidFormatError(String),
}

impl From<std::num::ParseIntError> for PartParseError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::NumParseError(value.to_string())
    }
}

fn parse_workflows(input: &str) -> HashMap<String, WorkflowDesc> {
    let wf_re = Regex::new(r"(\D+){(.+)})").unwrap();
    let rule_re = Regex::new(r"(.+):(.+)").unwrap();
    let mut wf: HashMap<String, WorkflowDesc> = Default::default();

    for line in input.lines().map(str::trim) {
        match wf_re.captures(line).map(|c| c.extract()) {
            Some((_, [name, rules])) => {
                let rules: Vec<RuleDesc> = rules
                    .split(',')
                    .map(|s| match rule_re.captures(s).map(|c| c.extract()) {
                        Some((_, [test, target])) => RuleDesc {
                            test: Some(test.to_owned()),
                            target: target.to_owned(),
                        },
                        None => RuleDesc {
                            test: None,
                            target: s.to_owned(),
                        },
                    })
                    .collect();
                wf.insert(name.to_owned(), WorkflowDesc::new(name, rules));
            }
            None => panic!("failed to parse workflow"),
        }
    }
    wf
}

impl FromStr for Part {
    type Err = PartParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap();
        match re.captures(s).map(|c| c.extract()) {
            Some((_, [x, m, a, s])) => Ok(Part {
                x: x.parse()?,
                m: m.parse()?,
                a: a.parse()?,
                s: s.parse()?,
            }),
            None => Err(PartParseError::InvalidFormatError(s.to_owned())),
        }
    }
}
fn parse_items(input: &str) -> Vec<Part> {
    input.lines().map(|s| s.trim().parse().unwrap()).collect()
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        Ok("px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}")
    }

    fn run(&self, input: &str) -> String {
        let i = input.find("\n\n").unwrap();
        let wfs = parse_workflows(&input[0..i]);
        
        let wfs = wfs.
        
        let items = parse_items(&input[i..]);
        
        for item in items.iter() {
            match 
        }

        "".to_owned()
    }
}
