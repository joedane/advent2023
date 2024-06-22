use petgraph::{
    dot::Dot,
    graph::{Graph, NodeIndex},
    visit::Dfs,
    Directed,
    Direction::{Incoming, Outgoing},
};
use regex::Regex;
use std::collections::{HashMap, VecDeque};

use super::{read_file, PuzzleRun};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Copy)]
struct Signal {
    pulse: Pulse,
    source: NodeIndex,
    target: NodeIndex,
}

#[derive(Debug, Clone, Copy)]
enum ModType {
    Broadcast,
    FlipFlop,
    Conjunction,
    Output,
}

#[derive(Debug)]
struct ModDesc<'a> {
    mod_type: ModType,
    label: &'a str,
    targets: Vec<&'a str>,
}

#[derive(Debug)]
enum Module<'a> {
    Broadcast,
    Output,
    FlipFlop(&'a str, bool),
    Conjunction(&'a str, HashMap<NodeIndex, Pulse>),
}

impl<'a> From<&'a str> for ModDesc<'a> {
    fn from(s: &'a str) -> Self {
        let re = Regex::new(r"(?<name>\S+) -> (?<targets>.+)").unwrap();
        let Some(caps) = re.captures(s.trim()) else {
            panic!("invalid module description");
        };
        let (mod_type, label) = match caps.name("name").unwrap().as_str() {
            "broadcaster" => (ModType::Broadcast, caps.name("name").unwrap().as_str()),
            label => match label.chars().nth(0) {
                Some('%') => (ModType::FlipFlop, &label[1..]),
                Some('&') => (ModType::Conjunction, &label[1..]),
                _ => {
                    panic!("module without type info");
                }
            },
        };
        let targets: Vec<&str> = caps
            .name("targets")
            .unwrap()
            .as_str()
            .split(',')
            .map(str::trim)
            .collect();

        ModDesc {
            mod_type,
            label,
            targets,
        }
    }
}

struct SignalQueue {
    q: VecDeque<Signal>,
    low_count: u64,
    high_count: u64,
}

impl SignalQueue {
    fn new() -> Self {
        Self {
            q: Default::default(),
            low_count: 0,
            high_count: 0,
        }
    }

    fn score(&self) -> u64 {
        self.low_count * self.high_count
    }

    fn send(&mut self, source: NodeIndex, pulse: Pulse, graph: &Graph<String, (), Directed>) {
        let mut neighbors: Vec<NodeIndex> = graph.neighbors_directed(source, Outgoing).collect();
        neighbors.reverse();

        for n in neighbors {
            self.add(source, n, pulse);
        }
    }

    fn add(&mut self, source: NodeIndex, target: NodeIndex, pulse: Pulse) {
        //println!("{:?} sending {:?} to {:?}", source, pulse, target);
        self.q.push_back(Signal {
            pulse,
            source,
            target,
        });
        match pulse {
            Pulse::Low => self.low_count += 1,
            Pulse::High => self.high_count += 1,
        };
    }

    fn take(&mut self) -> Option<Signal> {
        self.q.pop_front()
    }
}

struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        /*
                Ok("broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a")
                */
        /*
                Ok("broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output")
                */
        read_file("input/day20.txt")
    }

    fn run<'a>(&self, input: &'a str) -> String {
        let v: Vec<ModDesc<'a>> = input.lines().map(|s| s.into()).collect();
        let mut graph: Graph<String, (), Directed> = Graph::new();
        let mut node_map: HashMap<&str, (NodeIndex, ModType, Vec<&'a str>)> = v
            .into_iter()
            .map(|d| {
                (
                    d.label,
                    (
                        graph.add_node({
                            let mut s = match d.mod_type {
                                ModType::Broadcast => "",
                                ModType::FlipFlop => "%",
                                ModType::Conjunction => "&",
                                ModType::Output => "",
                            }
                            .to_string();
                            s.push_str(d.label);
                            s
                        }),
                        d.mod_type,
                        d.targets,
                    ),
                )
            })
            .collect();

        let mut v = vec![];

        for (label, (idx, mod_type, targets)) in &node_map {
            for &t in targets {
                match node_map.get(t) {
                    Some(_) => {}
                    None => v.push(t),
                }
            }
        }
        for label in v {
            node_map.insert(
                label,
                (graph.add_node(label.to_string()), ModType::Output, vec![]),
            );
        }

        let mut node_data: HashMap<NodeIndex, Module> = node_map
            .iter()
            .map(|(label, (idx, mod_type, _targets))| {
                let target_mod = match mod_type {
                    ModType::Broadcast => Module::Broadcast,
                    ModType::Output => Module::Output,
                    ModType::FlipFlop => Module::FlipFlop(label, false),
                    ModType::Conjunction => Module::Conjunction(label, Default::default()),
                };
                (*idx, target_mod)
            })
            .collect();

        for (_, (idx, _, targets)) in node_map.iter() {
            for target in targets {
                graph.add_edge(*idx, node_map.get(target).unwrap().0, ());
            }
        }
        let button = graph.add_node("button".to_string());
        graph.add_edge(button, node_map.get("broadcaster").unwrap().0, ());

        println!("{:?}", Dot::new(&graph));

        panic!();

        // set up conjunction module inputs
        let mut source_map: HashMap<NodeIndex, Vec<NodeIndex>> = Default::default();
        for (_, (idx, _, targets)) in node_map.iter() {
            for t in targets.iter() {
                source_map
                    .entry(node_map.get(*t).unwrap().0)
                    .or_default()
                    .push(*idx);
            }
        }

        for (idx, inputs) in source_map.iter() {
            let mut data = node_data.get_mut(idx).unwrap();
            if let Module::Conjunction(_, ref mut node_inputs) = data {
                for i in inputs {
                    node_inputs.insert(*i, Pulse::Low);
                }
            }
        }

        let mut signals = SignalQueue::new();

        for _ in 0..1000 {
            signals.send(button, Pulse::Low, &graph);
            //        signals.send(node_map.get("broadcaster").unwrap().0, Pulse::Low, &graph);

            while let Some(signal) = signals.take() {
                let target = node_data.get_mut(&signal.target).unwrap();
                match target {
                    Module::Output => {}
                    Module::Broadcast => {
                        signals.send(signal.target, signal.pulse, &graph);
                    }
                    Module::FlipFlop(_, is_on) => {
                        if matches!(signal.pulse, Pulse::High) {
                            // ignore
                        } else {
                            let next_pulse = if *is_on { Pulse::Low } else { Pulse::High };
                            signals.send(signal.target, next_pulse, &graph);
                            *is_on = !*is_on;
                        }
                    }
                    Module::Conjunction(_, input_state) => {
                        input_state.insert(signal.source, signal.pulse).unwrap();
                        let all_high = input_state.values().all(|p| matches!(*p, Pulse::High));
                        signals.send(
                            signal.target,
                            if all_high { Pulse::Low } else { Pulse::High },
                            &graph,
                        );
                    }
                }
            }
        }
        format!("{}", signals.score())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_regex() {
        let re = Regex::new(r"(?<name>\S+) -> (?<targets>.+)").unwrap();
        assert!(re.is_match("broadcaster -> a, b, c"));
        assert!(re.is_match("%a -> b"));
    }
    #[test]
    fn test_parse() {
        let s = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let v: Vec<ModDesc> = s.lines().map(|s| s.into()).collect();
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn test_parse2() {
        let input = Part1::input_data(&Part1).unwrap();

        let v: Vec<ModDesc<'_>> = input.lines().map(|s| s.into()).collect();
        let mut graph: Graph<&str, (), Directed> = Graph::new();
        let mut node_map: HashMap<&str, (NodeIndex, ModType, Vec<&'_ str>)> = v
            .into_iter()
            .map(|d| (d.label, (graph.add_node(d.label), d.mod_type, d.targets)))
            .collect();

        let mut v = vec![];

        for (label, (idx, mod_type, targets)) in &node_map {
            for &t in targets {
                match node_map.get(t) {
                    Some(_) => {}
                    None => v.push(t),
                }
            }
        }
        for label in v {
            node_map.insert(label, (graph.add_node(label), ModType::Output, vec![]));
        }
    }
    #[test]
    fn test_part1() {
        println!("{}", Part1::run(&Part1, Part1::input_data(&Part1).unwrap()));
    }
}
