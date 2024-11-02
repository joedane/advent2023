use itertools::Itertools;
use regex::Regex;

use super::{read_file, PuzzleRun};

struct Part1;

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u16,
    y: u16,
    z: u16,
}

impl Point {
    fn new(x: u16, y: u16, z: u16) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
struct Obj {
    idx: u16,
    label: String,
    start: Point,
    end: Point,
}

struct PointIterator {
    start: Point,
    end: Point,
    seen: u16,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, end) = (self.start, self.end);
        if start.x != end.x {
            if start.x + self.seen > end.x {
                None
            } else {
                let r = Some(Point::new(start.x + self.seen, start.y, start.z));
                self.seen += 1;
                r
            }
        } else if start.y != end.y {
            if start.y + self.seen > end.y {
                None
            } else {
                let r = Some(Point::new(start.x, start.y + self.seen, start.z));
                self.seen += 1;
                r
            }
        } else if start.z != end.z {
            if start.z + self.seen > end.z {
                None
            } else {
                let r = Some(Point::new(start.x, start.y, start.z + self.seen));
                self.seen += 1;
                r
            }
        } else {
            if self.seen > 0 {
                None
            } else {
                let r = Some(Point::new(start.x, start.y, start.z));
                self.seen += 1;
                r
            }
        }
    }
}
impl Obj {
    fn new(label: String, start: Point, end: Point) -> Self {
        if start.x != end.x {
            if start.x < end.x {
                Self {
                    idx: 0,
                    label,
                    start,
                    end,
                }
            } else {
                Self {
                    idx: 0,
                    label,
                    start: end,
                    end: start,
                }
            }
        } else if start.y != end.y {
            if start.y < end.y {
                Self {
                    idx: 0,
                    label,
                    start,
                    end,
                }
            } else {
                Self {
                    idx: 0,
                    label,
                    start: end,
                    end: start,
                }
            }
        } else if start.z != end.z {
            if start.z < end.z {
                Self {
                    idx: 0,
                    label,
                    start,
                    end,
                }
            } else {
                Self {
                    idx: 0,
                    label,
                    start: end,
                    end: start,
                }
            }
        } else {
            // start and end are the same point
            Self {
                idx: 0,
                label,
                start,
                end,
            }
        }
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        PointIterator {
            start: self.start,
            end: self.end,
            seen: 0,
        }
    }

    fn can_drop(&self, grid: &Grid) -> bool {
        self.points()
            .all(|p| p.z > 1 && grid.at(p.x, p.y, p.z - 1) == 0)
    }
    fn drop(&mut self, grid: &mut Grid) {
        self.start.z -= 1;
        self.end.z -= 1;
        for p in self.points() {
            grid.set(p.x, p.y, p.z, 0);
            grid.set(p.x, p.y, p.z, self.idx);
        }
    }
}

impl std::str::FromStr for Obj {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
        if let Some(c) = r.captures(s) {
            let (label, [x_1, y_1, z_1, x_2, y_2, z_2]) = c.extract();
            Ok(Obj {
                idx: 0,
                label: label.to_string(),
                start: Point::new(
                    x_1.parse().unwrap(),
                    y_1.parse().unwrap(),
                    z_1.parse().unwrap(),
                ),
                end: Point::new(
                    x_2.parse().unwrap(),
                    y_2.parse().unwrap(),
                    z_2.parse().unwrap(),
                ),
            })
        } else {
            Err("failed to parse")
        }
    }
}

#[derive(Debug)]
struct Grid {
    data: Box<[u16]>,
    x_offset: usize,
    y_offset: usize,
    z_offset: usize,
    x_size: usize,
    y_size: usize,
    z_size: usize,
}

impl Grid {
    fn _idx(
        x_offset: usize,
        y_offset: usize,
        z_offset: usize,
        x_size: usize,
        y_size: usize,
        z_size: usize,
        x: u16,
        y: u16,
        z: u16,
    ) -> usize {
        (z as usize - z_offset) * x_size * y_size
            + (y as usize - y_offset) * x_size
            + (x as usize - x_offset)
    }

    fn idx(&self, x: u16, y: u16, z: u16) -> usize {
        Self::_idx(
            self.x_offset,
            self.y_offset,
            self.z_offset,
            self.x_size,
            self.y_size,
            self.z_size,
            x,
            y,
            z,
        )
    }
    fn new(objs: &Vec<Obj>) -> Self {
        let Some((ll, ur)) = objs
            .iter()
            .map(|o| {
                (
                    Point::new(
                        u16::min(o.start.x, o.end.x),
                        u16::min(o.start.y, o.end.y),
                        u16::min(o.start.z, o.end.z),
                    ),
                    Point::new(
                        u16::max(o.start.x, o.end.x),
                        u16::max(o.start.y, o.end.y),
                        u16::max(o.start.z, o.end.z),
                    ),
                )
            })
            .reduce(|acc, v| {
                (
                    Point::new(
                        u16::min(acc.0.x, v.0.x),
                        u16::min(acc.0.y, v.0.y),
                        u16::min(acc.0.z, v.0.z),
                    ),
                    Point::new(
                        u16::max(acc.1.x, v.1.x),
                        u16::max(acc.1.y, v.1.y),
                        u16::max(acc.1.z, v.1.z),
                    ),
                )
            })
        else {
            panic!()
        };

        let (x_size, y_size, z_size) = (
            (ur.x - ll.x) as usize + 1,
            (ur.y - ll.y) as usize + 1,
            (ur.z - ll.z) as usize + 1,
        );
        let mut v: Vec<u16> = Vec::with_capacity(x_size * y_size * z_size);
        v.extend(std::iter::repeat(0).take(x_size * y_size * z_size));
        for (idx, obj) in objs.iter().enumerate() {
            let idx: u16 = idx.try_into().unwrap();
            for p in obj.points() {
                v[Self::_idx(
                    ll.x as usize,
                    ll.y as usize,
                    ll.z as usize,
                    x_size,
                    y_size,
                    z_size,
                    p.x,
                    p.y,
                    p.z,
                )] = idx;
            }
        }

        Self {
            data: v.into_boxed_slice(),
            x_offset: ll.x as usize,
            y_offset: ll.y as usize,
            z_offset: ll.z as usize,
            x_size,
            y_size,
            z_size,
        }
    }

    fn at(&self, x: u16, y: u16, z: u16) -> u16 {
        let id = self.idx(x, y, z);
        self.data[self.idx(x, y, z)]
    }

    fn set(&mut self, x: u16, y: u16, z: u16, val: u16) {
        self.data[self.idx(x, y, z)] = val;
    }
}
impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        Ok("1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9")

        //        read_file("input/day22.txt")
    }

    fn run(&self, input: &str) -> String {
        let mut objs: Vec<Obj> = input
            .lines()
            .enumerate()
            .map(|(idx, s)| {
                let mut obj: Obj = s.parse().unwrap();
                let idx: u16 = idx.try_into().unwrap();
                obj.idx = idx + 1;
                obj
            })
            .collect();
        objs.sort_by_key(|o| u16::min(o.start.z, o.end.z) as i32);
        let mut grid = Grid::new(&objs);

        for o in objs.iter_mut() {
            while o.can_drop(&grid) {
                println!("dropping obj [{}]", o.label);
                o.drop(&mut grid);
            }
        }

        "FOO".to_string()
    }
}

#[cfg(test)]
mod test {

    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn test_part1() {
        println!("{}", Part1.run(Part1.input_data().unwrap()));
    }

    #[test]
    fn test_grid() {
        let mut objs: Vec<Obj> = Part1
            .input_data()
            .unwrap()
            .lines()
            .enumerate()
            .map(|(idx, s)| {
                let mut obj: Obj = s.parse().unwrap();
                let idx: u16 = idx.try_into().unwrap();
                obj.idx = idx + 1;
                obj
            })
            .collect();
        objs.sort_by_key(|o| u16::min(o.start.z, o.end.z) as i32);
        let grid = Grid::new(&objs);

        assert_eq!(grid.at(1, 0, 1), 1);
    }
    #[test]
    fn test_point_iter() {
        let o = Obj::new(
            "test".to_string(),
            Point::new(5, 5, 5),
            Point::new(10, 5, 5),
        );
        let mut it = o.points();
        assert_eq!(it.next(), Some(Point::new(5, 5, 5)));
        assert_eq!(it.next(), Some(Point::new(6, 5, 5)));
        assert_eq!(it.next(), Some(Point::new(7, 5, 5)));
        assert_eq!(it.next(), Some(Point::new(8, 5, 5)));
        assert_eq!(it.next(), Some(Point::new(9, 5, 5)));
        assert_eq!(it.next(), Some(Point::new(10, 5, 5)));
        assert_eq!(it.next(), None);

        let o = Obj::new("test".to_string(), Point::new(5, 5, 5), Point::new(5, 5, 5));
        let mut it = o.points();
        assert_eq!(it.next(), Some(Point::new(5, 5, 5)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_input_ranges() {
        let objs: Vec<Obj> = read_file("input/day22.txt")
            .unwrap()
            .lines()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut ranges: [BTreeSet<u16>; 3] = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];
        for obj in objs {
            ranges[0].insert(obj.start.x);
            ranges[0].insert(obj.end.x);
            ranges[1].insert(obj.start.y);
            ranges[1].insert(obj.end.y);
            ranges[2].insert(obj.start.z);
            ranges[2].insert(obj.end.z);
        }
        println!(
            "x: ({}, {})\ty: ({}, {})\tz: ({}, {})",
            ranges[0].first().unwrap(),
            ranges[0].last().unwrap(),
            ranges[1].first().unwrap(),
            ranges[1].last().unwrap(),
            ranges[2].first().unwrap(),
            ranges[2].last().unwrap()
        );
    }
}
