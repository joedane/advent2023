use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    ops::{Add, AddAssign},
    str::FromStr,
};

use num::Bounded;
use num_traits::Zero;

use petgraph::{
    algo::dijkstra,
    visit::{
        Data, EdgeRef, GraphBase, IntoEdgeReferences, IntoEdges, IntoNeighbors, VisitMap, Visitable,
    },
};

use super::{read_file, PuzzleRun};
use crate::grid::{Dir, Grid};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

struct Part1;

impl FromStr for Grid<u16> {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut v: Vec<u16> = Default::default();
        let mut width = 0;
        for line in input.lines().map(str::trim) {
            let mut w = 0;
            for c in line.chars() {
                v.push(c.to_digit(10).unwrap().try_into().unwrap());
                w += 1;
            }
            width = w;
        }
        Ok(Grid::new(width, v.len() / width, v.into_boxed_slice()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MyEdgeId<T> {
    source: MyNode,
    target: MyNode,
    weight: T,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub struct MyNode {
    x: usize,
    y: usize,
    dir_from: Dir,
    dist_from: usize,
}

impl MyNode {
    fn new(x: usize, y: usize, dir_from: Dir, dist_from: usize) -> Self {
        Self {
            x,
            y,
            dir_from,
            dist_from,
        }
    }
}
impl<T: Copy + PartialEq> GraphBase for Grid<T> {
    #[doc = r" edge identifier"]
    type EdgeId = MyEdgeId<T>;

    #[doc = r" node identifier"]
    type NodeId = MyNode;
}

pub struct MyEdgesType<T>(std::vec::IntoIter<MyEdgeRef<T>>);

impl<T> Iterator for MyEdgesType<T> {
    type Item = MyEdgeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
impl<T> IntoEdges for &Grid<T>
where
    T: Copy + PartialEq + AddAssign<T> + Add<Output = T> + Zero + std::fmt::Debug,
{
    type Edges = MyEdgesType<T>;

    fn edges(self, a: Self::NodeId) -> Self::Edges {
        if a.x == 0 && a.y == 0 {
            let edges: Vec<MyEdgeRef<T>> = self
                .cardinal_neighbors(0, 0, 3..10)
                .into_iter()
                .map(|(this_x, this_y, this_dir)| {
                    MyEdgeRef(MyEdgeId {
                        source: a.clone(),
                        target: MyNode::new(
                            this_x,
                            this_y,
                            this_dir,
                            self.distance_between(0, 0, this_x, this_y, this_dir),
                        ),
                        weight: self.weight_between(0, 0, this_x, this_y),
                    })
                })
                .collect();
            return MyEdgesType(edges.into_iter());
        }
        let mut v: Vec<MyEdgeRef<T>> = self
            .cardinal_neighbors(a.x, a.y, 3..10)
            .into_iter()
            .filter_map(|(this_x, this_y, this_dir)| match (a.dir_from, this_dir) {
                (Dir::N | Dir::S, Dir::N | Dir::S) => None,
                (Dir::E | Dir::W, Dir::E | Dir::W) => None,
                _ => Some(MyEdgeRef(MyEdgeId {
                    source: a.clone(),
                    target: MyNode::new(
                        this_x,
                        this_y,
                        this_dir,
                        self.distance_between(a.x, a.y, this_x, this_y, this_dir),
                    ),
                    weight: self.weight_between(a.x, a.y, this_x, this_y),
                })),
            })
            .collect();
        if a.x == 0 && a.y == 0 {
            dbg!(&v);
        }
        MyEdgesType(v.into_iter())
    }
}

pub struct MyNeighbors;

impl Iterator for MyNeighbors {
    type Item = MyNode;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T: Copy + PartialEq> IntoNeighbors for &Grid<T> {
    type Neighbors = MyNeighbors;

    #[doc = r" Return an iterator of the neighbors of node `a`."]
    fn neighbors(self, a: Self::NodeId) -> Self::Neighbors {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MyEdgeRef<T>(MyEdgeId<T>);

impl<T: Copy> EdgeRef for MyEdgeRef<T> {
    type NodeId = MyNode;

    type EdgeId = MyEdgeId<T>;

    type Weight = T;

    fn source(&self) -> Self::NodeId {
        self.0.source
    }

    fn target(&self) -> Self::NodeId {
        self.0.target
    }

    fn weight(&self) -> &Self::Weight {
        &self.0.weight
    }

    fn id(&self) -> Self::EdgeId {
        self.0
    }
}
pub struct MyEdgeReferences<T>(PhantomData<T>);

impl<T> Iterator for MyEdgeReferences<T> {
    type Item = MyEdgeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
//impl GraphRef for Grid<MyU8> {}

impl<T: Copy + PartialEq> IntoEdgeReferences for &Grid<T> {
    type EdgeRef = MyEdgeRef<T>;

    type EdgeReferences = MyEdgeReferences<T>;

    fn edge_references(self) -> Self::EdgeReferences {
        todo!()
    }
}

pub struct MyNodeWeight;

impl<T: Copy + PartialEq> Data for Grid<T> {
    type NodeWeight = MyNodeWeight;

    type EdgeWeight = T;
}

pub struct MyVisitableMap {
    visits: HashSet<MyNode>,
}

impl MyVisitableMap {
    fn new() -> Self {
        Self {
            visits: Default::default(),
        }
    }

    fn reset(&mut self) {
        self.visits.clear();
    }
}
impl VisitMap<MyNode> for MyVisitableMap {
    fn visit(&mut self, a: MyNode) -> bool {
        self.visits.insert(a)
    }

    fn is_visited(&self, a: &MyNode) -> bool {
        self.visits.contains(a)
    }
}
impl<T: Copy + PartialEq> Visitable for Grid<T> {
    #[doc = r" The associated map type"]
    type Map = MyVisitableMap;

    #[doc = r" Create a new visitor map"]
    fn visit_map(self: &Self) -> Self::Map {
        MyVisitableMap::new()
    }

    #[doc = r" Reset the visitor map (and resize to new size of graph if needed)"]
    fn reset_map(self: &Self, map: &mut Self::Map) {
        map.reset();
    }
}

impl Part1 {}
impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day17.txt")
        /*
        Ok("2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533")
        */
    }

    fn run(&self, input: &str) -> String {
        let grid: Grid<u16> = Grid::from_str(input).unwrap();
        let start = MyNode {
            x: 0,
            y: 0,
            dir_from: Dir::W,
            dist_from: 0,
        };
        let res = dijkstra(&grid, start, None, |n| *n.weight());
        let mut res_map: HashMap<(usize, usize), Vec<(&MyNode, u16)>> = Default::default();
        for node in res.keys() {
            res_map
                .entry((node.x, node.y))
                .and_modify(|v| v.push((node, *res.get(node).unwrap())))
                .or_insert(vec![(node, *res.get(node).unwrap())]);
        }

        let candidates = res
            .keys()
            .filter(|&k| k.x == grid.width - 1 && k.y == grid.height - 1)
            .collect::<Vec<_>>();
        let min = *candidates
            .iter()
            .map(|&k| res.get(k).unwrap())
            .min()
            .unwrap();
        let min_candidates: Vec<&MyNode> = candidates
            .into_iter()
            .filter(|&k| *res.get(k).unwrap() == min)
            .collect();

        println!("{} possible paths", min_candidates.len());
        if min_candidates.len() != 1 {
            panic!();
        }
        format!("{:?}", min)
    }
}

struct Part2;

impl Part2 {
    fn trace_back(
        res: &HashMap<MyNode, u16>,
        by_coord: &HashMap<(usize, usize), Vec<MyNode>>,
        node: MyNode,
    ) -> MyNode {
        let coord = match node.dir_from {
            Dir::E => (node.x + node.dist_from, node.y),
            Dir::W => (node.x - node.dist_from, node.y),
            Dir::N => (node.x, node.y - node.dist_from),
            Dir::S => (node.x, node.y + node.dist_from),
        };

        let candidates = by_coord.get(&coord).unwrap();
        if candidates.len() == 0 {
            panic!();
        }
        let mut best: Option<MyNode> = None;
        let mut best_score = u16::MAX;
        for i in 0..candidates.len() {
            match (node.dir_from, candidates[i].dir_from) {
                (Dir::E | Dir::W, Dir::N | Dir::S) | (Dir::S | Dir::N, Dir::E | Dir::W) => {
                    if let Some(&this_score) = res.get(&candidates[i]) {
                        if this_score < best_score {
                            best_score = this_score;
                            best = Some(candidates[i]);
                        }
                    }
                }
                _ => continue,
            }
        }
        best.unwrap()
    }
}
impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        /*
        Ok("111111111111
        999999999991
        999999999991
        999999999991
        999999999991")
            */
        /*
        Ok("2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533")
        */
        read_file("input/day17.txt")
    }

    fn run(&self, input: &str) -> String {
        let grid: Grid<u16> = Grid::from_str(input).unwrap();
        let start = MyNode {
            x: 0,
            y: 0,
            dir_from: Dir::W,
            dist_from: 0,
        };
        let res = dijkstra(&grid, start, None, |n| *n.weight());
        let mut coord_map: HashMap<(usize, usize), Vec<MyNode>> = Default::default();

        let candidates = res
            .keys()
            .inspect(|&k| coord_map.entry((k.x, k.y)).or_default().push(*k))
            .filter(|&k| k.x == grid.width - 1 && k.y == grid.height - 1)
            .collect::<Vec<_>>();
        let min = *candidates
            .iter()
            .map(|&k| res.get(k).unwrap())
            .min()
            .unwrap();

        let mut node = *res
            .iter()
            .filter(|&(_, &cost)| cost == min)
            .nth(0)
            .unwrap()
            .0;
        let mut path: Vec<MyNode> = vec![];

        while !(node.x == 0 && node.y == 0) {
            path.push(node);
            let maybe_last: Vec<MyNode> = grid
                .direction_range(node.x, node.y, 4, 11, node.dir_from)
                .into_iter()
                .map(|(x, y)| Part2::trace_back(&res, &coord_map, node))
                .collect();
            let (best_node, best_score): (MyNode, u16) = maybe_last
                .into_iter()
                .filter_map(|n| res.get(&n).map(|&c| (n, c)))
                .min_by(|a, b| a.1.cmp(&b.1))
                .unwrap();
            node = best_node;
        }
        path.reverse();
        dbg!(path);
        format!("{:?}", min)
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
    fn test_edges() {
        let g: Grid<u16> = Grid::from_str(
            "111111111111
        999999999991
        999999999991
        999999999991
        999999999991",
        )
        .unwrap();
        let n = MyNode::new(0, 0, Dir::W, 0);
        let e: Vec<MyEdgeRef<_>> = g.edges(n).collect();
        assert_eq!(e.len(), 7);
    }
    #[test]
    fn test_part2() {
        println!("{}", Part2::run(&Part2, Part2::input_data(&Part2).unwrap()));
    }
}
