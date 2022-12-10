use std::{fmt::Debug, ops::BitOr};
mod scenic;
mod visibility;

fn main() {
    let input = include_str!("../input");
    let trees: Grid<u8> = Grid::parse(input);
    println!("Q1: {}", trees.num_visible());
    println!("Q2: {}", trees.scenic_score().max());
}

/// (0,0) is the top-left.
struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn side_len(&self) -> usize {
        self.0.len()
    }
}

impl<T> Grid<T>
where
    T: Ord + Copy,
{
    pub fn max(&self) -> T {
        let mut greatest_so_far = self.get(0, 0);
        let n = self.side_len();
        for x in 0..n {
            for y in 0..n {
                let this = self.get(x, y);
                if this > greatest_so_far {
                    greatest_so_far = this;
                }
            }
        }
        greatest_so_far
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(side_len: usize) -> Self {
        Self(vec![vec![Default::default(); side_len]; side_len])
    }
}

impl<T: BitOr + Copy + Default> BitOr for Grid<T>
where
    T::Output: Default + Copy,
{
    type Output = Grid<T::Output>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let n = self.side_len();
        let mut new = Grid::new(n);
        for x in 0..n {
            for y in 0..n {
                let val = self.get(x, y) | rhs.get(x, y);
                new.set(x, y, val);
            }
        }
        new
    }
}

impl<T: Copy> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> T {
        self.0[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.0[x][y] = val;
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[allow(dead_code)] // Useful for debugging.
impl<T: Debug + Clone> Grid<T> {
    fn print(&self) {
        let tr = transpose(self.0.clone());
        for y in 0..self.side_len() {
            println!("{:?}", tr[y]);
        }
    }
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let v = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        Self(transpose(v))
    }
}
