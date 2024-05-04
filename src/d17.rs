use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    marker::PhantomData,
    ops::{Add, AddAssign},
    str::FromStr,
};

use num_traits::Zero;

use petgraph::{
    algo::dijkstra,
    visit::{
        Data, EdgeRef, GraphBase, GraphRef, IntoEdgeReferences, IntoEdges, IntoNeighbors, VisitMap,
        Visitable,
    },
    Graph,
};

use super::{read_file, PuzzleRun};
use crate::grid::{Dir, Grid};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

struct Part1;

/*
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct MyU8(u8);

impl From<char> for MyU8 {
    fn from(value: char) -> Self {
        let v = (value as u32) - 48;
        MyU8(v.try_into().unwrap())
    }
}

impl AddAssign<MyU8> for MyU8 {
    fn add_assign(&mut self, rhs: MyU8) {
        self.0 += rhs.0;
    }
}
*/

/*
struct MyGraph {
    grid: Grid<MyU8>,
    nodes: RefCell<SlotMap<DefaultKey, MyNode>>,
}

impl MyGraph {
    fn new(grid: Grid<MyU8>) -> Self {
        Self {
            grid,
            nodes: Default::default(),
        }
    }

    fn add_node(&mut self, node: MyNode) -> DefaultKey {
        self.nodes.borrow_mut().insert(node)
    }

    fn get_node(&self, key: DefaultKey) -> MyNode {
        self.nodes.borrow().get(key).unwrap().clone()
    }

    fn get_node_weight(&self, key: DefaultKey) -> u32 {
        let n = self.get_node(key);
        self.grid.get(n.x, n.y).0.into()
    }
    fn width(&self) -> usize {
        self.grid.width
    }

    fn height(&self) -> usize {
        self.grid.height
    }

    fn edges(&self, node_key: DefaultKey) -> MyEdgesType {
    }
}

*/
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
    dir: Dir,
}

impl MyNode {
    fn new(x: usize, y: usize, dir: Dir) -> Self {
        Self { x, y, dir }
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
    T: Copy + PartialEq + AddAssign<T> + Add<Output = T> + Zero,
{
    type Edges = MyEdgesType<T>;

    fn edges(self, a: Self::NodeId) -> Self::Edges {
        let mut v: Vec<MyEdgeRef<T>> = self
            .cardinal_neighbors(a.x, a.y, 2)
            .into_iter()
            .filter_map(|(this_x, this_y, this_dir, this_weight)| match a.dir {
                Dir::N | Dir::S => match this_dir {
                    Dir::E | Dir::W => Some(MyEdgeRef(MyEdgeId {
                        source: a.clone(),
                        target: MyNode::new(this_x, this_y, this_dir),
                        weight: this_weight,
                    })),
                    _ => None,
                },
                Dir::E | Dir::W => match this_dir {
                    Dir::E | Dir::W => None,
                    Dir::N | Dir::S => Some(MyEdgeRef(MyEdgeId {
                        source: a.clone(),
                        target: MyNode::new(this_x, this_y, this_dir),
                        weight: this_weight,
                    })),
                },
            })
            .collect();
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
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
    }

    fn run(&self, input: &str) -> String {
        let grid: Grid<u16> = Grid::from_str(input).unwrap();
        let start = MyNode {
            x: 0,
            y: 0,
            dir: Dir::W,
        };
        let mut res = dijkstra(&grid, start, None, |n| *n.weight());

        let candidates = res
            .keys()
            .filter(|&k| k.x == grid.width - 1 && k.y == grid.height - 1)
            .collect::<Vec<_>>();
        let min = candidates
            .iter()
            .map(|&k| res.get(k).unwrap())
            .min()
            .unwrap();

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
}
