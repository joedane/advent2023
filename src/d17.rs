use std::collections::{HashMap, HashSet};

use petgraph::{
    algo::dijkstra,
    visit::{
        Data, EdgeRef, GraphBase, GraphRef, IntoEdgeReferences, IntoEdges, IntoNeighbors, VisitMap,
        Visitable,
    },
    Graph,
};

use super::{read_file, Dir, Grid, PuzzleRun};

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

struct Part1;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct MyU8(u8);

impl From<char> for MyU8 {
    fn from(value: char) -> Self {
        let v = (value as u32) - 48;
        MyU8(v.try_into().unwrap())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MyEdgeId {
    source: MyNodeId,
    //    dir: Dir,
    target: MyNodeId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct MyNodeId {
    x: usize,
    y: usize,
    cnt: u8,
    dir: Dir,
}

impl MyNodeId {
    fn new(x: usize, y: usize, cnt: u8, dir: Dir) -> Self {
        Self { x, y, cnt, dir }
    }
}
impl GraphBase for Grid<MyU8> {
    #[doc = r" edge identifier"]
    type EdgeId = MyEdgeId;

    #[doc = r" node identifier"]
    type NodeId = MyNodeId;
}

struct MyEdgesType(std::vec::IntoIter<MyEdgeRef>);

impl Iterator for MyEdgesType {
    type Item = MyEdgeRef;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
impl IntoEdges for &Grid<MyU8> {
    type Edges = MyEdgesType;

    fn edges(self, a: Self::NodeId) -> Self::Edges {
        let mut v: Vec<MyEdgeRef> = self
            .cardinal_neighbors(a.x, a.y)
            .into_iter()
            .filter_map(|e| {
                if e.2 == a.dir {
                    if a.cnt == 0 {
                        None
                    } else {
                        Some(MyEdgeRef(MyEdgeId {
                            source: a.clone(),
                            target: MyNodeId::new(e.0, e.1, a.cnt - 1, e.2),
                        }))
                    }
                } else {
                    Some(MyEdgeRef(MyEdgeId {
                        source: a.clone(),
                        target: MyNodeId::new(e.0, e.1, 3, e.2),
                    }))
                }
            })
            .collect();
        MyEdgesType(v.into_iter())
    }
}

struct MyNeighbors;

impl Iterator for MyNeighbors {
    type Item = MyNodeId;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl IntoNeighbors for &Grid<MyU8> {
    type Neighbors = MyNeighbors;

    #[doc = r" Return an iterator of the neighbors of node `a`."]
    fn neighbors(self, a: Self::NodeId) -> Self::Neighbors {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
struct MyEdgeRef(MyEdgeId);

impl EdgeRef for MyEdgeRef {
    type NodeId = MyNodeId;

    type EdgeId = MyEdgeId;

    type Weight = MyEdgeWeight;

    fn source(&self) -> Self::NodeId {
        self.0.source
    }

    fn target(&self) -> Self::NodeId {
        self.0.target
    }

    fn weight(&self) -> &Self::Weight {
        &MyEdgeWeight
    }

    fn id(&self) -> Self::EdgeId {
        self.0
    }
}
struct MyEdgeReferences;

impl Iterator for MyEdgeReferences {
    type Item = MyEdgeRef;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
//impl GraphRef for Grid<MyU8> {}

impl IntoEdgeReferences for &Grid<MyU8> {
    type EdgeRef = MyEdgeRef;

    type EdgeReferences = MyEdgeReferences;

    fn edge_references(self) -> Self::EdgeReferences {
        todo!()
    }
}

struct MyNodeWeight;

struct MyEdgeWeight;

impl Data for Grid<MyU8> {
    type NodeWeight = MyNodeWeight;

    type EdgeWeight = MyEdgeWeight;
}

struct MyVisitableMap {
    visits: HashSet<MyNodeId>,
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
impl VisitMap<MyNodeId> for MyVisitableMap {
    fn visit(&mut self, a: MyNodeId) -> bool {
        self.visits.insert(a)
    }

    fn is_visited(&self, a: &MyNodeId) -> bool {
        self.visits.contains(a)
    }
}
impl Visitable for Grid<MyU8> {
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

impl Part1 {
    fn min_for_coord(res: &HashMap<MyNodeId, u32>, x: usize, y: usize) -> u32 {
        res.keys()
            .filter(|k| k.x == x && k.y == y)
            .map(|k| res.get(k).unwrap())
            .copied()
            .min()
            .unwrap()
    }

    fn best_path_from(
        grid: &Grid<MyU8>,
        res: &HashMap<MyNodeId, u32>,
        node_coord: Coord,
    ) -> Vec<(Coord, u32)> {
        let mut completed_paths: Vec<(Vec<(Coord, u32)>)> = vec![];

        let mut paths: Vec<(Coord, Vec<(Coord, u32)>)> =
            vec![(Coord::new(node_coord.x, node_coord.y), vec![])];

        while let Some((mut coord, mut path)) = paths.pop() {
            while !(coord.x == 0 && coord.y == 0) {
                let this_score = Part1::min_for_coord(&res, coord.x, coord.y);
                let neighbor_min = grid
                    .cardinal_neighbors(coord.x, coord.y)
                    .into_iter()
                    .map(|(x, y, _)| Part1::min_for_coord(&res, x, y))
                    .min()
                    .unwrap();
                let mut min_neighbors: Vec<(usize, usize)> = grid
                    .cardinal_neighbors(coord.x, coord.y)
                    .into_iter()
                    .filter(|(x, y, _)| {
                        res.keys()
                            .filter(|k| k.x == *x && k.y == *y)
                            .any(|k| *res.get(&k).unwrap() == neighbor_min)
                    })
                    .map(|(x, y, _)| (x, y))
                    .collect();
                while min_neighbors.len() > 1 {
                    let n = min_neighbors.pop().unwrap();
                    paths.push((Coord::new(n.0, n.1), path.clone()));
                }
                let next = Coord::new(min_neighbors[0].0, min_neighbors[0].1);
                path.push((next, this_score));
                coord = next;
            }
            path.reverse();
            completed_paths.push(path);
        }
        if completed_paths.len() > 1 {
            panic!()
        }
        completed_paths.pop().unwrap()
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
        let grid: Grid<MyU8> = Grid::new(input);

        let mut res = dijkstra(&grid, MyNodeId::new(0, 0, 3, Dir::W), None, |n| {
            let (x, y) = (n.0.target.x, n.0.target.y);
            grid.get(x, y).0 as u32
        });

        print!("   ");
        for x in 0..grid.width {
            print!("{:3}|", x);
        }
        println!("");
        for y in 0..grid.height {
            print!("{:2}:", y);
            for x in 0..grid.width {
                print!("{:3}|", Part1::min_for_coord(&res, x, y));
            }
            println!("");
        }

        let candidates = res
            .keys()
            .filter(|k| k.x == grid.width - 1 && k.y == grid.height - 1)
            .collect::<Vec<_>>();
        let min = candidates
            .iter()
            .map(|k| res.get(k).unwrap())
            .min()
            .unwrap();
        let mut best = candidates
            .iter()
            .filter(|k| res.get(k).unwrap() == min)
            .nth(0)
            .unwrap();

        let mut path: Vec<Coord> = vec![];

        while !(best.x == 0 && best.y == 0) {
            path.push(Coord::new(best.x, best.y));
            best = grid.try_next(x, y, dir)
        }
        for c in candidates {
            println!("{:?}:\t{}", c, res.get(c).unwrap());
        }

        format!("{:?}", 1)
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
