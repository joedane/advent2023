use indextree::{Arena, NodeId};
use regex::Regex;
use std::{
    collections::HashMap,
    fmt::Display,
    str::{FromStr, Lines},
};

use super::{read_file, PuzzleRun};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    Gt,
    Lt,
    Jump,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct RuleTest {
    category: Category,
    op: Op,
    test_val: u16,
}

impl RuleTest {
    fn new(category: Category, op: Op, test_val: u16) -> Self {
        Self {
            category,
            op,
            test_val,
        }
    }

    fn negate(&self) -> Self {
        match self.op {
            Op::Gt => Self {
                op: Op::Lt,
                test_val: self.test_val + 1,
                ..*self
            },
            Op::Lt => Self {
                op: Op::Gt,
                test_val: self.test_val - 1,
                ..*self
            },
            Op::Jump => panic!(),
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

    fn apply_tests(&self, mut source_range: Ranges, rule_idx: usize) -> Ranges {
        for idx in 0..rule_idx {
            source_range = source_range.apply(self.rules[idx].test.as_ref().unwrap().negate())
        }
        match self.rules[rule_idx].test {
            Some(test) => source_range.apply(test),
            None => source_range,
        }
    }
}

struct RuleDesc {
    test: Option<RuleTest>,
    target: String,
}

/*
* op_code: 4 bits
* cat_code: 2 bits
* target: 14 bits
* test_val: 12 bits
 */
#[derive(Debug, Clone, Copy)]
struct Instr(u32);

const OPCODE_SHIFT: usize = 28;
const OPCODE_MASK: u32 = 0xF0000000;

const CATCODE_SHIFT: usize = 26;
const CATCODE_MASK: u32 = 0xC000000;
const TARGET_SHIFT: usize = 12;
const TARGET_MASK: u32 = 0x3FFF << TARGET_SHIFT;
const TESTVAL_MASK: u32 = 0xFFF;

const OP_JUMP: u32 = 0;
const OP_TEST_GT: u32 = 1 << OPCODE_SHIFT;
const OP_TEST_LT: u32 = 2 << OPCODE_SHIFT;

const CATCODE_X: u32 = 0;
const CATCODE_M: u32 = 1 << CATCODE_SHIFT;
const CATCODE_A: u32 = 2 << CATCODE_SHIFT;
const CATCODE_S: u32 = 3 << CATCODE_SHIFT;

const PC_ACCEPT: u16 = 0;
const PC_REJECT: u16 = 1;

impl Instr {
    fn make_test(category: Category, op: Op, test_val: u16, target: u16) -> Self {
        let op_code: u32 = match op {
            Op::Gt => OP_TEST_GT,
            Op::Lt => OP_TEST_LT,
            _ => panic!(),
        };
        let cat_code: u32 = match category {
            Category::X => CATCODE_X,
            Category::M => CATCODE_M,
            Category::A => CATCODE_A,
            Category::S => CATCODE_S,
        };

        Self(op_code | cat_code | (target as u32) << TARGET_SHIFT | test_val as u32)
    }

    fn make_jump(target: u16) -> Self {
        let op_code = OP_JUMP;
        Self(op_code | (target as u32) << TARGET_SHIFT)
    }

    fn dummy() -> Self {
        Self(0)
    }

    fn opcode(&self) -> Op {
        match self.0 & OPCODE_MASK {
            OP_JUMP => Op::Jump,
            OP_TEST_GT => Op::Gt,
            OP_TEST_LT => Op::Lt,
            _ => panic!(),
        }
    }

    fn category(&self) -> Category {
        match self.0 & CATCODE_MASK {
            CATCODE_X => Category::X,
            CATCODE_M => Category::M,
            CATCODE_A => Category::A,
            CATCODE_S => Category::S,
            _ => panic!(),
        }
    }

    fn test_val(&self) -> u16 {
        (self.0 & TESTVAL_MASK).try_into().unwrap()
    }

    fn target(&self) -> u16 {
        ((self.0 & TARGET_MASK) >> TARGET_SHIFT).try_into().unwrap()
    }

    fn target_label<'a, 'b>(&'a self, labels: &'b HashMap<usize, &str>) -> &'b str {
        let target = self.target();
        match target {
            0 => "A",
            1 => "R",
            _ => labels.get(&(target as usize)).unwrap(),
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Instr op: {:?}, cat: {:?}, test_val: {}, target: {}",
            self.opcode(),
            self.category(),
            self.test_val(),
            self.target()
        )
    }
}
#[derive(Debug, Clone, Copy)]
enum ItemResult {
    Accept,
    Reject,
}
struct Machine {
    entry: usize,
    code: Vec<Instr>,
    labels: HashMap<String, usize>,
}

fn encode_label(label: &str, offsets: &HashMap<String, usize>) -> u16 {
    if label == "A" {
        PC_ACCEPT
    } else if label == "R" {
        PC_REJECT
    } else {
        TryInto::<u16>::try_into(*offsets.get(label).unwrap()).unwrap()
    }
}
impl Machine {
    fn init(workflows: Vec<WorkflowDesc>) -> Self {
        let mut idx: usize = 2;
        let offsets: HashMap<String, usize> = workflows
            .iter()
            .map(|wf| {
                let ret = (wf.name.clone(), idx);
                idx += wf.rules.len();
                ret
            })
            .collect();
        println!("offsets: {:?}", offsets);
        let mut code: Vec<Instr> = Vec::with_capacity(idx);
        code.push(Instr::dummy());
        code.push(Instr::dummy());

        for (wf_desc) in workflows {
            for rule in wf_desc.rules {
                match rule.test {
                    Some(test) => code.push(Instr::make_test(
                        test.category,
                        test.op,
                        TryInto::<u16>::try_into(test.test_val).unwrap(),
                        encode_label(&rule.target, &offsets),
                    )),
                    None => code.push(Instr::make_jump(encode_label(&rule.target, &offsets))),
                }
            }
        }
        Self {
            code,
            entry: *offsets.get("in").unwrap(),
            labels: offsets,
        }
    }

    fn extract_category(op_code: u32) -> Category {
        if (op_code & CATCODE_MASK) == CATCODE_X {
            Category::X
        } else if (op_code & CATCODE_MASK) == CATCODE_M {
            Category::M
        } else if (op_code & CATCODE_MASK) == CATCODE_A {
            Category::A
        } else if (op_code & CATCODE_MASK) == CATCODE_S {
            Category::S
        } else {
            panic!();
        }
    }

    fn disassemble(&self) {
        let ordered_labels: HashMap<usize, &str> =
            self.labels.iter().map(|(k, &v)| (v, k.as_str())).collect();
        let mut pc: usize = 2;
        let mut s = String::new();
        'wf: while pc < self.code.len() {
            let Some(label) = ordered_labels.get(&pc) else {
                panic!()
            };
            s.push_str(label);
            s.push('{');
            'step: while pc < self.code.len() {
                let instr = self.code[pc];
                match instr.opcode() {
                    Op::Gt => {
                        match instr.category() {
                            Category::X => s.push_str("x>"),
                            Category::M => s.push_str("m>"),
                            Category::A => s.push_str("a>"),
                            Category::S => s.push_str("s>"),
                        };
                        s.push_str(&instr.test_val().to_string());
                        s.push(':');
                        s.push_str(instr.target_label(&ordered_labels));
                        s.push(',');
                    }
                    Op::Lt => {
                        match instr.category() {
                            Category::X => s.push_str("x<"),
                            Category::M => s.push_str("m<"),
                            Category::A => s.push_str("a<"),
                            Category::S => s.push_str("s<"),
                        };
                        s.push_str(&instr.test_val().to_string());
                        s.push(':');
                        s.push_str(instr.target_label(&ordered_labels));
                        s.push(',');
                    }
                    Op::Jump => {
                        s.push_str(instr.target_label(&ordered_labels));
                        s.push_str("}");
                    }
                }
                pc += 1;
                if ordered_labels.contains_key(&pc) {
                    println!("{}", s);
                    s.clear();
                    continue 'wf;
                }
            }
            println!("{}", s);
            s.clear();
        }
    }

    fn run(&self, item: Part) -> ItemResult {
        let mut pc = self.entry;
        println!("testing part: {:?}", item);
        loop {
            println!("PC: {}", pc);
            if pc == PC_ACCEPT as usize {
                return ItemResult::Accept;
            } else if pc == PC_REJECT as usize {
                return ItemResult::Reject;
            } else {
                println!("instr: {}", self.code[pc]);
                let op_code = self.code[pc].0;
                if op_code & OPCODE_MASK == OP_JUMP {
                    let target = (op_code & TARGET_MASK) >> TARGET_SHIFT;
                    pc = target as usize;
                    continue;
                } else if op_code & OPCODE_MASK == OP_TEST_GT {
                    let test_val = TryInto::<u16>::try_into(op_code & TESTVAL_MASK).unwrap();
                    let category = Self::extract_category(op_code);

                    if item.extract_val(category) > test_val {
                        let target = (op_code & TARGET_MASK) >> TARGET_SHIFT;
                        pc = target as usize;
                        continue;
                    }
                } else if op_code & OPCODE_MASK == OP_TEST_LT {
                    let test_val = TryInto::<u16>::try_into(op_code & TESTVAL_MASK).unwrap();
                    let category = Self::extract_category(op_code);

                    if item.extract_val(category) < test_val {
                        let target = (op_code & TARGET_MASK) >> TARGET_SHIFT;
                        pc = target as usize;
                        continue;
                    }
                } else {
                    panic!();
                }
                pc += 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn score(&self) -> u64 {
        self.x as u64 + self.m as u64 + self.a as u64 + self.s as u64
    }

    fn extract_val(&self, category: Category) -> u16 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
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

fn parse_test(input: &str) -> RuleTest {
    RuleTest {
        category: input[0..1].parse().unwrap(),
        op: input[1..2].parse().unwrap(),
        test_val: input[2..].parse().unwrap(),
    }
}

fn parse_rules(input: &str) -> Vec<RuleDesc> {
    let rule_re = Regex::new(r"(.+):(.+)").unwrap();
    input
        .split(',')
        .map(|s| match rule_re.captures(s).map(|c| c.extract()) {
            Some((_, [test, target])) => {
                let test = parse_test(test);
                RuleDesc {
                    test: Some(test),
                    target: target.to_owned(),
                }
            }
            None => RuleDesc {
                test: None,
                target: s.to_owned(),
            },
        })
        .collect()
}
fn parse_workflows(input: &mut Lines) -> Vec<WorkflowDesc> {
    let wf_re = Regex::new(r"(\D+)\{(.+)\}").unwrap();
    let mut wf: Vec<WorkflowDesc> = Default::default();

    for line in input {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        match wf_re.captures(line).map(|c| c.extract()) {
            Some((_, [name, rules])) => {
                let rules = parse_rules(rules);
                wf.push(WorkflowDesc::new(name, rules));
            }
            None => panic!("failed to parse workflow"),
        }
    }
    wf
}

impl FromStr for Part {
    type Err = PartParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
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
fn parse_items<'a, T: Iterator<Item = &'a str>>(input: T) -> Vec<Part> {
    input.map(|s| s.trim().parse().unwrap()).collect()
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        /*
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
        */
        read_file("input/day19.txt")
    }

    fn run(&self, input: &str) -> String {
        let mut lines = input.lines();
        let wf_descs = parse_workflows(&mut lines);
        let machine = Machine::init(wf_descs);
        let items = parse_items(lines);
        let mut num_accepted: usize = 0;
        let mut accepted_score: u64 = 0;

        for item in items.iter() {
            match machine.run(*item) {
                ItemResult::Accept => {
                    num_accepted += 1;
                    accepted_score += item.score();
                }
                ItemResult::Reject => {}
            }
        }

        format!("{} accepted, total score {}", num_accepted, accepted_score)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ranges {
    x: (u16, u16),
    m: (u16, u16),
    a: (u16, u16),
    s: (u16, u16),
}

impl Ranges {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn range_for(&self, category: Category) -> (u16, u16) {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn count_items(&self) -> u64 {
        u16::max(self.x.1 - self.x.0 + 1, 0) as u64
            * u16::max(self.m.1 - self.m.0 + 1, 0) as u64
            * u16::max(self.a.1 - self.a.0 + 1, 0) as u64
            * u16::max(self.s.1 - self.s.0 + 1, 0) as u64
    }

    fn with_lb(&self, category: Category, lb: u16) -> Self {
        match category {
            Category::X => Self {
                x: (u16::max(self.x.0, lb), self.x.1),
                ..*self
            },
            Category::M => Self {
                m: (u16::max(self.m.0, lb), self.m.1),
                ..*self
            },
            Category::A => Self {
                a: (u16::max(self.a.0, lb), self.a.1),
                ..*self
            },
            Category::S => Self {
                s: (u16::max(self.s.0, lb), self.s.1),
                ..*self
            },
        }
    }

    fn with_ub(&self, category: Category, ub: u16) -> Self {
        match category {
            Category::X => Self {
                x: (self.x.0, u16::min(self.x.1, ub)),
                ..*self
            },
            Category::M => Self {
                m: (self.m.0, u16::min(self.m.1, ub)),
                ..*self
            },
            Category::A => Self {
                a: (self.a.0, u16::min(self.a.1, ub)),
                ..*self
            },
            Category::S => Self {
                s: (self.s.0, u16::min(self.s.1, ub)),
                ..*self
            },
        }
    }

    fn from(&self, new_range: (u16, u16), category: Category) -> Self {
        match category {
            Category::X => Self {
                x: new_range,
                ..*self
            },
            Category::M => Self {
                m: new_range,
                ..*self
            },
            Category::A => Self {
                a: new_range,
                ..*self
            },
            Category::S => Self {
                s: new_range,
                ..*self
            },
        }
    }
    fn apply(&self, test: RuleTest) -> Self {
        match test.op {
            Op::Gt => self.with_lb(test.category, test.test_val + 1),
            Op::Lt => self.with_ub(test.category, test.test_val - 1),
            Op::Jump => panic!(),
        }
    }
}

struct Part2;

#[derive(Debug, Clone, Copy)]
struct RuleRef<'a> {
    wf: &'a str,
    index: usize,
}

impl<'a> RuleRef<'a> {
    fn new(wf: &'a str, index: usize) -> Self {
        Self { wf, index }
    }
}

impl WorkflowDesc {
    fn find_rules_with_target(&self, target: &str) -> Vec<RuleRef> {
        self.rules
            .iter()
            .enumerate()
            .filter_map(|(idx, r)| {
                if r.target == target {
                    Some(RuleRef::new(self.name.as_str(), idx))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn find_rules_with_target<'a>(wfs: &'a Vec<WorkflowDesc>, target: &'a str) -> Vec<RuleRef<'a>> {
    wfs.iter()
        .flat_map(|wf| wf.find_rules_with_target(target))
        .collect()
}

#[derive(Debug)]
enum NodeType<'a> {
    Workflow(Ranges, RuleRef<'a>),
    Accept(Ranges),
    Reject(Ranges),
}

impl<'a> NodeType<'a> {
    fn get_ranges(&self) -> Ranges {
        match (self) {
            Self::Workflow(ranges, _) => *ranges,
            Self::Accept(ranges) => *ranges,
            Self::Reject(ranges) => *ranges,
        }
    }
}
impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        /*
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
         */
        read_file("input/day19.txt")
    }

    fn run(&self, input: &str) -> String {
        let mut lines = input.lines();
        let wf_descs = parse_workflows(&mut lines);
        let wfs_by_labels: HashMap<&str, &WorkflowDesc> =
            wf_descs.iter().map(|wf| (wf.name.as_str(), wf)).collect();

        let root: &mut Arena<NodeType> = &mut Arena::new();
        let mut wf_stack: Vec<(&WorkflowDesc, indextree::NodeId)> = Vec::new();
        let start = *wfs_by_labels.get("in").unwrap();
        let start_node = root.new_node(NodeType::Workflow(Ranges::new(), RuleRef::new("in", 0)));
        wf_stack.push((start, start_node));
        while let Some((wf, node)) = wf_stack.pop() {
            let current_range = root.get(node).unwrap().get().get_ranges();
            for (index, r) in wf.rules.iter().enumerate() {
                match &r.test {
                    Some(test) => {
                        let target_name = r.target.as_str();
                        match target_name {
                            "A" => {
                                node.append_value(
                                    NodeType::Accept(wf.apply_tests(current_range, index)),
                                    root,
                                );
                            }
                            "R" => {
                                node.append_value(
                                    NodeType::Reject(wf.apply_tests(current_range, index)),
                                    root,
                                );
                            }
                            name => {
                                let target = *wfs_by_labels.get(target_name).unwrap();
                                let node = node.append_value(
                                    NodeType::Workflow(
                                        wf.apply_tests(current_range, index),
                                        RuleRef::new(&target.name, index),
                                    ),
                                    root,
                                );
                                wf_stack.push((target, node));
                            }
                        }
                    }
                    None => match r.target.as_str() {
                        "A" => {
                            node.append_value(
                                NodeType::Accept(wf.apply_tests(current_range, index)),
                                root,
                            );
                        }
                        "R" => {
                            node.append_value(
                                NodeType::Reject(wf.apply_tests(current_range, index)),
                                root,
                            );
                        }
                        next_wf => {
                            let target = *wfs_by_labels.get(next_wf).unwrap();
                            let node = node.append_value(
                                NodeType::Workflow(
                                    wf.apply_tests(current_range, index),
                                    RuleRef::new(&target.name, index),
                                ),
                                root,
                            );
                            wf_stack.push((target, node));
                        }
                    },
                }
            }
        }
        println!("{} nodes", root.count());
        //println!("{:?}", start_node.debug_pretty_print(root));
        let total: u64 = root
            .iter()
            .filter_map(|n| {
                let node = n.get();
                match node {
                    NodeType::Workflow(_, _) => None,
                    NodeType::Accept(range) => Some(range.count_items()),
                    NodeType::Reject(_) => None,
                }
            })
            .sum();
        format!("total: {}", total)
    }
}

#[cfg(test)]
mod test {

    use num_traits::Pow;

    use super::*;

    #[test]
    fn test_parse_part() {
        let s = "{x=787,m=2655,a=1222,s=2876}";
        let p: Part = s.parse().unwrap();
        assert_eq!(p.score(), 787 + 2655 + 1222 + 2876)
    }

    #[test]
    fn test_parse_wf() {
        let wf_re = Regex::new(r"(\D+)\{(.+)\}").unwrap();
        let s = "rfg{s<537:gd,x>2440:R,A}";
        let c = wf_re.captures(s).unwrap();
        assert_eq!(c.get(1).unwrap().as_str(), "rfg");
        assert_eq!(c.get(2).unwrap().as_str(), "s<537:gd,x>2440:R,A");

        let s = c.get(2).unwrap().as_str();
        let rules = parse_rules(s);
        assert_eq!(rules.len(), 3);
        assert_eq!(
            rules[0].test,
            Some(RuleTest {
                category: Category::S,
                op: Op::Lt,
                test_val: 537
            })
        );
        assert_eq!(rules[2].test, None);
        assert_eq!(rules[2].target, "A");
    }

    #[test]
    fn test_check_testlen() {
        let s = Part1::input_data(&Part1).unwrap();
        let wf = parse_workflows(&mut s.lines());
        println!(
            "{:?}",
            wf.iter()
                .flat_map(|wf| wf
                    .rules
                    .iter()
                    .filter_map(|r| r.test.as_ref().map(|t| t.test_val)))
                .max()
        );
    }
    #[test]
    fn test_dis() {
        let s = Part1::input_data(&Part1).unwrap();
        let m = Machine::init(parse_workflows(&mut s.lines()));
        m.disassemble();
    }
    #[test]
    fn test_part1() {
        println!("{}", Part1::run(&Part1, Part1::input_data(&Part1).unwrap()));
    }

    #[test]
    fn test_ranges() {
        let r = Ranges::new();
        let test = RuleTest::new(Category::A, Op::Lt, 2006);
        let r = r.apply(test);
        println!("{:?}", r);

        let test = RuleTest::new(Category::X, Op::Gt, 1000);
        let r = Ranges::new().apply(test);
        println!("{:?}", r);

        let test = RuleTest::new(Category::X, Op::Lt, 1001);
        let r = r.apply(test);
        println!("{:?}", r);
    }

    #[test]
    fn test_find_targets() {
        let input = Part2::input_data(&Part2).unwrap();
        let wf_descs = parse_workflows(&mut input.lines());

        let t = find_rules_with_target(&wf_descs, "A");
        println!("{:?}", t);
    }

    #[test]
    fn test_range_count() {
        let r = Ranges::new();
        assert_eq!(r.count_items(), 4000_u64.pow(4));
        let r = r.with_ub(Category::A, 2000);
        assert_eq!(r.count_items(), 4000_u64.pow(4) / 2)
    }
    #[test]
    fn test_part2() {
        println!("{}", Part2::run(&Part2, Part2::input_data(&Part2).unwrap()));
    }
}
