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
enum Orientation {
    X,
    Y,
    Z,
}

#[derive(Debug)]
struct Obj {
    label: String,
    start: Point,
    end: Point,
}

struct PointIter {
    steps: usize,
    next: Point,
    end: Point,
    dir: Orientation,
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.end {
            None
        } else {
            let next = self.next;
            self.next = match self.dir {
                Orientation::X => Point::new(next.x + 1, next.y, next.z),
                Orientation::Y => Point::new(next.x, next.y + 1, next.z),
                Orientation::Z => Point::new(next.x, next.y, next.z + 1),
            };
            Some(next)
        }
    }
}
impl Obj {
    fn get_orientation(&self) -> Orientation {
        if self.start.x != self.end.x {
            Orientation::X
        } else if self.start.y != self.end.y {
            Orientation::Y
        } else if self.start.z != self.end.z {
            Orientation::Z
        } else {
            panic!()
        }
    }
    fn points(&self) -> impl Iterator<Item = Point> {
        PointIter {
            steps: 0,
            next: self.start,
            end: self.end,
            dir: self.get_orientation(),
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

fn display_grid(grid: &Vec<Vec<u16>>) {
    for i in 0..grid.len() {
        print!("|");
        for j in 0..grid[i].len() {
            print!("{:3}|", grid[i][j])
        }
        println!("");
    }
}
impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        /*
                Ok("1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9")
                */
        read_file("input/day22.txt")
    }

    fn run(&self, input: &str) -> String {
        let mut objs: Vec<Obj> = input.lines().map(|s| s.parse().unwrap()).collect();
        let Some((ll, ur)) = objs
            .iter()
            .map(|o| {
                (
                    Point::new(
                        u16::min(o.start.x, o.end.x),
                        u16::min(o.start.y, o.end.y),
                        0,
                    ),
                    Point::new(
                        u16::max(o.start.x, o.end.x),
                        u16::max(o.start.y, o.end.y),
                        0,
                    ),
                )
            })
            .reduce(|acc, v| {
                (
                    Point::new(u16::min(acc.0.x, v.0.x), u16::min(acc.0.y, v.0.y), 0),
                    Point::new(u16::max(acc.1.x, v.1.x), u16::max(acc.1.y, v.1.y), 0),
                )
            })
        else {
            panic!()
        };

        let mut max_z: Vec<Vec<u16>> = vec![];
        assert!(ur.x > ll.x);
        assert!(ur.y > ll.y);
        max_z.resize_with((ur.y - ll.y) as usize + 1, || {
            let mut v = vec![];
            v.resize((ur.x - ll.x) as usize + 1, 0);
            v
        });

        let mut grounded = max_z.clone();
        let mut grounded_idx: Vec<usize> = vec![];

        for (idx, obj) in objs.iter().enumerate() {
            let is_grounded = u16::min(obj.start.z, obj.end.z) == 1;
            if is_grounded {
                grounded_idx.push(idx);
            }
            for p in obj.points() {
                max_z[p.x as usize][p.y as usize] =
                    u16::max(max_z[p.y as usize][p.x as usize], p.z);
                if is_grounded {
                    grounded[p.y as usize][p.x as usize] = p.z;
                }
            }
        }
        for idx in grounded_idx {
            objs.swap_remove(idx);
        }
        objs.sort_by_key(|o| -1 * u16::min(o.start.z, o.end.z) as i32);
        while let Some(mut o) = objs.pop() {
            let d = o
                .points()
                .map(|p| p.z - max_z[p.y as usize][p.x as usize])
                .min()
                .unwrap();
            println!("obj [{}] falling by {}", o.label, d);
            o.start.z -= d;
            o.end.z -= d;
            for p in o.points() {
                max_z[p.y as usize][p.x as usize] = u16::min(o.start.z, o.end.z);
            }
        }

        format!("ll: {:?}, ur: {:?}", ll, ur)
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
