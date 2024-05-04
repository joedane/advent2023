use num_traits::Zero;
use std::{ops::AddAssign, str::FromStr};

pub(crate) struct Grid<T> {
    data: Box<[T]>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}

impl<T> Grid<T>
where
    T: Clone,
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
}
impl<T> FromStr for Grid<T>
where
    T: From<u8>,
{
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut v: Vec<T> = Default::default();
        let mut width = 0;
        for line in input.lines().map(str::trim) {
            let mut w = 0;
            for c in line.bytes() {
                v.push(c.into());
                w += 1;
            }
            width = w;
        }
        Ok(Grid {
            height: v.len() / width,
            data: v.into_boxed_slice(),
            width,
        })
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

impl<T> Grid<T> {
    pub(crate) fn fmt<F>(&self, f: &mut std::fmt::Formatter<'_>, conv: F) -> std::fmt::Result
    where
        F: Fn(&T) -> &str,
    {
        let mut s = format!("Grid ({} by {}):\n", self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                s.push_str(conv(self.get(col, row)));
            }
            s.push('\n');
        }
        s.push('\n');
        write!(f, "{}", s)
    }
}
/*
impl<T> Display for Grid<T>
where
    char: for<'a> From<&'a T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, |i| i.into())
    }
}
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Dir {
    N,
    S,
    E,
    W,
}

impl<T: Copy + PartialEq> Grid<T> {
    pub(crate) fn count_cells<F>(&self, f: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        self.data.iter().filter(|&i| f(i)).count()
    }

    pub(crate) fn try_next_coord(&self, x: usize, y: usize, dir: Dir) -> Option<(usize, usize, T)> {
        match dir {
            Dir::E => {
                if x + 1 < self.width {
                    Some((x + 1, y, *self.get(x + 1, y)))
                } else {
                    None
                }
            }
            Dir::W => {
                if x > 0 {
                    Some((x - 1, y, *self.get(x - 1, y)))
                } else {
                    None
                }
            }
            Dir::N => {
                if y > 0 {
                    Some((x, y - 1, *self.get(x, y - 1)))
                } else {
                    None
                }
            }
            Dir::S => {
                if y + 1 < self.height {
                    Some((x, y + 1, *self.get(x, y + 1)))
                } else {
                    None
                }
            }
        }
    }

    fn try_next(&self, x: usize, y: usize, dir: Dir) -> Option<&T> {
        self.try_next_coord(x, y, dir)
            .and_then(|(x, y, w)| Some(w).as_ref())
    }

    fn try_next_mut(&mut self, x: usize, y: usize, dir: Dir) -> Option<&mut T> {
        self.try_next_coord(x, y, dir)
            .and_then(|(x, y, w)| Some(w).as_mut())
    }
}

impl<T> Grid<T>
where
    T: AddAssign<T> + Zero,
{
    /**
     * Dir in the return tuple is the direction from which we'll move
     */
    pub(crate) fn cardinal_neighbors(
        &self,
        x: usize,
        y: usize,
        cnt: usize,
    ) -> Vec<(usize, usize, Dir, T)> {
        let mut v = vec![];

        let mut weight = T::zero();
        for c in 0..cnt {
            if y >= c {
                if let Some((next_x, next_y, next_weight)) = self.try_next_coord(x, y - c, Dir::N) {
                    weight += next_weight;
                    v.push((next_x, next_y, Dir::S, weight));
                }
            }
        }

        weight = T::zero();
        for c in 0..cnt {
            if let Some((next_x, next_y, next_weight)) = self.try_next_coord(x, y + c, Dir::S) {
                weight += next_weight;
                v.push((next_x, next_y, Dir::N, weight));
            }
        }
        weight = T::zero();
        for c in 0..cnt {
            if let Some((next_x, next_y, next_weight)) = self.try_next_coord(x + c, y, Dir::E) {
                weight += next_weight;
                v.push((next_x, next_y, Dir::W, weight));
            }
        }
        weight = T::zero();
        for c in 0..cnt {
            if x >= c {
                if let Some((next_x, next_y, next_weight)) = self.try_next_coord(x - c, y, Dir::W) {
                    weight += next_weight;
                    v.push((next_x, next_y, Dir::E, weight));
                }
            }
        }

        v
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_neighbors() {
        let g = Grid::new_with(10, 10, 13u32);

        assert_eq!(g.cardinal_neighbors(5, 5, 1).len(), 4);
        assert_eq!(g.cardinal_neighbors(0, 0, 1).len(), 2);
        assert_eq!(g.cardinal_neighbors(5, 0, 2).len(), 6);

        assert_eq!(
            g.cardinal_neighbors(5, 0, 2)
                .iter()
                .filter(|n| false)
                .count(),
            2
        );
    }
}
