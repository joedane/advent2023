use num_traits::Zero;
use std::{fmt::Display, ops::AddAssign, str::FromStr};

#[derive(Clone)]
pub(crate) struct Grid<T> {
    data: Box<[T]>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl<T> Grid<T> {
    pub(crate) fn new(width: usize, height: usize, data: Box<[T]>) -> Self {
        Self {
            data,
            width,
            height,
        }
    }
}

impl<T> Grid<T>
where
    T: Clone + Default,
{
    pub(crate) fn new_with(width: usize, height: usize, el: T) -> Self {
        let mut v = vec![];
        v.resize(height * width, el);
        Self {
            width,
            height,
            data: v.into_boxed_slice(),
        }
    }

    pub(crate) fn new_from<F: Fn(usize, usize) -> T>(width: usize, height: usize, f: F) -> Self {
        let mut v = Vec::with_capacity(width * height);
        v.resize_with(width * height, Default::default);
        for y in 0..height {
            for x in 0..width {
                v[y * width + x] = f(x, y);
            }
        }
        Self::new(width, height, v.into_boxed_slice())
    }
}

impl<T> Grid<T> {
    pub(crate) fn get(&self, x: usize, y: usize) -> &T {
        &self.data[self.coord(x, y)]
    }

    pub(crate) fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[self.coord(x, y)]
    }

    pub(crate) fn from_coord(&self, coord: usize) -> (usize, usize) {
        (coord / self.width, coord % self.width)
    }

    pub(crate) fn coord(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("Grid ({} by {}):\n", self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                s.push('|');
                s.push_str(&self.get(col, row).to_string());
                s.push('|');
            }
            s.push('\n');
        }
        s.push('\n');
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Dir {
    N,
    S,
    E,
    W,
}

impl<T> Grid<T> {
    pub(crate) fn count_cells<F>(&self, f: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        self.data.iter().filter(|&i| f(i)).count()
    }

    pub(crate) fn try_next_coord(&self, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::E => {
                if x + 1 < self.width {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            Dir::W => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Dir::N => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Dir::S => {
                if y + 1 < self.height {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
        }
    }

    fn try_next(&self, x: usize, y: usize, dir: Dir) -> Option<&T> {
        self.try_next_coord(x, y, dir)
            .and_then(|(x, y)| Some(self.get(x, y)))
    }

    fn try_next_mut(&mut self, x: usize, y: usize, dir: Dir) -> Option<&mut T> {
        self.try_next_coord(x, y, dir)
            .and_then(|(x, y)| Some(self.get_mut(x, y)))
    }
}

impl<T> Grid<T>
where
    T: AddAssign<T> + Zero + Copy,
{
    pub(crate) fn weight_between(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> T {
        let mut weight = T::zero();
        if x1 == x2 && y1 == y2 {
            panic!();
        }
        if x1 == x2 {
            if y2 > y1 {
                for i in 0..(y2 - y1) {
                    weight += *self.get(x1, y1 + 1 + i);
                }
            } else {
                for i in 0..(y1 - y2) {
                    weight += *self.get(x1, y1 - 1 - i);
                }
            }
        } else if y1 == y2 {
            if x2 > x1 {
                for i in 0..(x2 - x1) {
                    weight += *self.get(x1 + 1 + i, y1);
                }
            } else {
                for i in 0..(x1 - x2) {
                    weight += *self.get(x1 - 1 - i, y1);
                }
            }
        } else {
            panic!()
        }
        weight
    }
}
impl<T> Grid<T> {
    /**
     * Dir in the return tuple is the direction from which we'll move
     */
    pub(crate) fn cardinal_neighbors(
        &self,
        x: usize,
        y: usize,
        distance: std::ops::Range<usize>,
    ) -> Vec<(usize, usize, Dir)> {
        let mut v = vec![];

        for c in distance {
            if y >= c {
                if let Some((next_x, next_y)) = self.try_next_coord(x, y - c, Dir::N) {
                    v.push((next_x, next_y, Dir::S));
                }
            }
            if let Some((next_x, next_y)) = self.try_next_coord(x, y + c, Dir::S) {
                v.push((next_x, next_y, Dir::N));
            }
            if let Some((next_x, next_y)) = self.try_next_coord(x + c, y, Dir::E) {
                v.push((next_x, next_y, Dir::W));
            }
            if x >= c {
                if let Some((next_x, next_y)) = self.try_next_coord(x - c, y, Dir::W) {
                    v.push((next_x, next_y, Dir::E));
                }
            }
        }
        v
    }

    pub(crate) fn direction_range(
        &self,
        x: usize,
        y: usize,
        min: usize,
        max: usize,
        dir: Dir,
    ) -> Vec<(usize, usize)> {
        match dir {
            Dir::W => (x.saturating_sub(max)..=(x.saturating_sub(min)))
                .map(|c| (c, y))
                .collect(),
            Dir::E => ((x + min)..(x + max)).map(|c| (c, y)).collect(),
            Dir::N => (y.saturating_sub(max)..=(y.saturating_sub(min)))
                .map(|c| (x, c))
                .collect(),
            Dir::S => ((y + min)..(y + max)).map(|c| (x, c)).collect(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[derive(Clone, Default, Debug, Copy, PartialEq, PartialOrd)]
    struct Test(u8);

    impl From<&Test> for char {
        fn from(value: &Test) -> Self {
            char::from_digit(value.0 as u32, 10).unwrap()
        }
    }

    impl Display for Test {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:2}", self.0)
        }
    }
    impl Zero for Test {
        fn zero() -> Self {
            Self(0)
        }

        fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }

    impl std::ops::Add for Test {
        type Output = Test;

        fn add(self, rhs: Self) -> Self::Output {
            Test(self.0 + rhs.0)
        }
    }

    impl std::ops::AddAssign for Test {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }
    #[test]
    fn test_neighbors() {
        let g = Grid::new_with(10, 10, 13u32);

        assert_eq!(g.cardinal_neighbors(5, 5, 0..1).len(), 4);
        assert_eq!(g.cardinal_neighbors(0, 0, 0..1).len(), 2);
        assert_eq!(g.cardinal_neighbors(5, 0, 0..2).len(), 6);

        assert_eq!(
            g.cardinal_neighbors(5, 0, 0..2)
                .iter()
                .filter(|(x, y, d)| g.weight_between(5, 0, *x, *y) == 26)
                .count(),
            3
        );

        let g = Grid::new_with(50, 50, 2u32);
        assert_eq!(g.cardinal_neighbors(0, 0, 4..11).len(), 14);
    }

    #[test]
    fn test_weights() {
        let g: Grid<Test> = Grid::new_from(10, 10, |x, y| Test(2 * (x as u8) + 2 * (y as u8)));
        println!("{}", g);
        assert_eq!(g.weight_between(0, 0, 3, 0), Test(12));
        assert_eq!(g.weight_between(3, 0, 0, 0), Test(6));
        assert_eq!(g.weight_between(0, 0, 0, 3), Test(12));
        assert_eq!(g.weight_between(0, 3, 0, 0), Test(6));
    }

    #[test]
    fn test_part2() {
        let g: Grid<u16> = Grid::from_str(
            "111111111111
         999999999991
         999999999991
         999999999991
         999999999991",
        )
        .unwrap();

        assert_eq!(g.cardinal_neighbors(0, 0, 3..10).len(), 8);
        assert_eq!(g.weight_between(0, 0, 0, 4), 36);
    }

    #[test]
    fn test_range() {
        let g: Grid<u16> = Grid::from_str(
            "111111111111
         999999999991
         999999999991
         999999999991
         999999999991",
        )
        .unwrap();
        assert_eq!(g.direction_range(0, 0, 4, 11, Dir::E).len(), 7);
        assert_eq!(g.direction_range(11, 4, 4, 11, Dir::N).len(), 1);
        assert_eq!(g.direction_range(4, 0, 4, 11, Dir::W).len(), 1);
        assert_eq!(g.direction_range(7, 0, 4, 11, Dir::E).len(), 1);
    }
}
