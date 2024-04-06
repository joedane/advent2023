use super::PuzzleRun;

pub(crate) fn get_runs() -> std::vec::Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

struct Part1;

impl Part1 {
    fn find_symmetry(data: &Data) -> Option<Symmetry> {
        let h = data.len() / data.width();
        let mut i = 0;

        'select_col: while i < data.width() - 1 {
            if data[(0, i)] == data[(0, i + 1)] {
                for j in 1..h {
                    if data[(j, i)] != data[(j, i + 1)] {
                        i += 1;
                        continue 'select_col;
                    }
                }
                let mut ii = 1;
                while i >= ii && i + 1 + ii < data.width() {
                    for j in 0..h {
                        if data[(j, i - ii)] != data[(j, i + 1 + ii)] {
                            i += 1;
                            continue 'select_col;
                        }
                    }
                    ii += 1;
                }
                return Some(Symmetry::V(i));
            }
            i += 1;
        }
        i = 0;
        'select_row: while i < h - 1 {
            if data[(i, 0)] == data[(i + 1, 0)] {
                for j in 1..(data.width() - 1) {
                    if data[(i, j)] != data[(i + 1, j)] {
                        i += 1;
                        continue 'select_row;
                    }
                }
                let mut ii = 1;
                while i >= ii && i + 1 + ii < h {
                    for j in 0..data.width() {
                        if data[(i - ii, j)] != data[(i + 1 + ii, j)] {
                            i += 1;
                            continue 'select_row;
                        }
                    }
                    ii += 1;
                }
                return Some(Symmetry::H(i));
            }
            i += 1;
        }
        None
    }
}
#[derive(PartialEq, Eq, Debug)]
enum Material {
    Ash,
    Rock,
}

impl std::str::FromStr for Material {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('.') => Ok(Self::Ash),
            Some('#') => Ok(Self::Rock),
            _ => Err("invalid"),
        }
    }
}

impl From<char> for Material {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Symmetry {
    H(usize),
    V(usize),
}

impl Symmetry {
    fn value(&self) -> u32 {
        match self {
            Self::V(v) => *v as u32 + 1,
            Self::H(v) => ((*v as u32) + 1) * 100,
        }
    }
}
struct Data {
    data: Vec<Material>,
    _width: usize,
}

impl Data {
    fn new(data: Vec<Material>, width: usize) -> Self {
        if data.len() % width != 0 {
            panic!()
        }
        Data {
            data,
            _width: width,
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn width(&self) -> usize {
        self._width
    }

    fn height(&self) -> usize {
        self.data.len() / self._width
    }
}
impl std::str::FromStr for Data {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().map(str::trim);
        if let Some(s) = lines.next() {
            let mut data: Vec<Material> = Default::default();
            let width = s.len();
            s.chars().for_each(|c| data.push(c.into()));
            for s in lines {
                s.chars().for_each(|c| data.push(c.into()));
            }
            Ok(Data::new(data, width))
        } else {
            Err("bad data")
        }
    }
}

/**
 * index by (row, col)
 */
impl std::ops::Index<(usize, usize)> for Data {
    type Output = Material;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.width() + index.1]
    }
}
impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        #[rustfmt::skip]
        let s = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        crate::read_file("input/day13.txt")
        //Ok(s)
    }

    fn run(&self, input: &str) -> String {
        let v: u32 = input
            .trim()
            .split_terminator("\n\n")
            //            .inspect(|s| println!("reading: {s}"))
            .map(|s| s.parse::<Data>().unwrap())
            .map(|d| Part1::find_symmetry(&d).unwrap().value())
            .sum();

        format!("{}", v)
    }
}

struct Part2;

impl Part2 {
    fn find_broken_symmetry(d: &Data) -> Result<Symmetry, &'static str> {
        'col: for col in 1..d.width() {
            let mut check = 0;
            let sym_width = std::cmp::min(col, d.width() - col);
            for row in 0..d.height() {
                for i in 1..=sym_width {
                    if d[(row, col - i)] != d[(row, col + i - 1)] {
                        if check > 0 {
                            continue 'col;
                        } else {
                            check += 1;
                        }
                    }
                }
            }
            if check == 1 {
                return Ok(Symmetry::V(col - 1));
            }
        }

        'row: for row in 1..d.height() {
            let mut check = 0;
            let sym_width = std::cmp::min(row, d.height() - row);
            for col in 0..d.width() {
                for i in 1..=sym_width {
                    if d[(row - i, col)] != d[(row + i - 1, col)] {
                        if check > 0 {
                            continue 'row;
                        } else {
                            check += 1;
                        }
                    }
                }
            }
            if check == 1 {
                return Ok(Symmetry::H(row - 1));
            }
        }
        Err("failed to find broken symmetry")
    }
}

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        crate::read_file("input/day13.txt")
    }

    fn run(&self, input: &str) -> String {
        let v: u32 = input
            .trim()
            .split_terminator("\n\n")
            //  .inspect(|s| println!("reading: {s}"))
            .map(|s| s.parse::<Data>().unwrap())
            .map(|d| Part2::find_broken_symmetry(&d).unwrap().value())
            .sum();

        format!("{}", v)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let s = "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.";

        let d: Data = s.parse().unwrap();
        assert_eq!(d.width(), 9);
    }

    #[test]
    fn test_idx() {
        let s = "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.";

        let d: Data = s.parse().unwrap();
        assert_eq!(d[(0, 0)], Material::Rock);
        assert_eq!(d[(0, 1)], Material::Ash);
    }
    #[test]
    fn test_sym1() {
        let s = "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.";

        let d: Data = s.parse().unwrap();
        assert_eq!(Part1::find_symmetry(&d), Some(Symmetry::V(4)))
    }

    #[test]
    fn test_sym2() {
        let s = "#...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";

        let d: Data = s.parse().unwrap();
        assert_eq!(Part1::find_symmetry(&d), Some(Symmetry::H(3)))
    }

    #[test]
    fn test_sym3() {
        let s = "........#..
        .######.##.
        ##....###..
        ##....###..
        .######.##.
        ........#..
        ##....##..#
        #.......#..
        ########...
        #..##..####
        .#.##.#..##
        #.####.####
        ..#..#.....
        .######.###
        ...##...#..";

        let d: Data = s.parse().unwrap();
        assert_eq!(Part1::find_symmetry(&d), Some(Symmetry::H(2)))
    }

    #[test]
    fn test_part1() {
        println!("{}", Part1.run(Part1.input_data().unwrap()));
    }

    #[test]
    fn test_bs() {
        let s = "...#...####...#..
        .....##.##.##....
        ##....######....#
        ..#.##.#..#.##...
        ##.###.####.###.#
        ..###...##...###.
        #####.##..##.####
        #######....######
        ###...#.##.#...##
        ....###.##.###...
        ##.####.##.####.#
        ..###...##...###.
        ##.#.##....##.#.#
        ##..#.#....#.#..#
        ##.###.#..#.###.#
        ###.#...##...#.##
        ..####.####.####.";
        let d: Data = s.parse().unwrap();
        //println!("{:?}", Part1::find_symmetry(&d).unwrap());
        assert_eq!(9, Part2::find_broken_symmetry(&d).unwrap().value());
    }
    #[test]
    fn test_part2() {
        println!("{}", Part2.run(Part2.input_data().unwrap()));
    }
}
