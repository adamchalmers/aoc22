use std::{cmp::max, fmt::Debug, ops::BitOr};

fn main() {
    let input = include_str!("../input");
    let trees: Grid<u8> = Grid::parse(input);
    println!("Q1: {}", trees.num_visible());
}

/// (0,0) is the top-left.
struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn side_len(&self) -> usize {
        self.0.len()
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

    // O(n^2)
    pub fn visible_from_some_direction(&self) -> Grid<bool> {
        let l = self.visible_from_left();
        let r = self.visible_from_right();
        let b = self.visible_from_bottom();
        let t = self.visible_from_top();
        l | r | t | b
    }

    // O(n^2)
    pub fn num_visible(&self) -> usize {
        let vis = self.visible_from_some_direction();
        let mut answer = 0;
        for x in 0..self.side_len() {
            for y in 0..self.side_len() {
                if vis.get(x, y) {
                    answer += 1;
                }
            }
        }
        answer
    }

    // O(n^2)
    pub fn visible_from_left(&self) -> Grid<bool> {
        let mut tallest_tree = Grid::new(self.side_len());
        for x in 0..self.side_len() {
            for y in 0..self.side_len() {
                let val = if x == 0 {
                    self.get(x, y)
                } else {
                    max(self.get(x, y), tallest_tree.get(x - 1, y))
                };
                tallest_tree.set(x, y, val);
            }
        }
        let mut visible = Grid::new(self.side_len());
        for x in 0..self.side_len() {
            for y in 0..self.side_len() {
                let val = (x == 0) || self.get(x, y) > tallest_tree.get(x - 1, y);
                visible.set(x, y, val);
            }
        }
        visible
    }

    pub fn visible_from_right(&self) -> Grid<bool> {
        let n = self.side_len();
        let mut tallest_tree = Grid::new(n);
        for x in (0..n).rev() {
            for y in 0..n {
                let val = if x == n - 1 {
                    self.get(x, y)
                } else {
                    max(self.get(x, y), tallest_tree.get(x + 1, y))
                };
                tallest_tree.set(x, y, val);
            }
        }
        let mut visible = Grid::new(n);
        for x in 0..n {
            for y in 0..n {
                let val = (x == n - 1) || self.get(x, y) > tallest_tree.get(x + 1, y);
                visible.set(x, y, val);
            }
        }
        visible
    }

    pub fn visible_from_top(&self) -> Grid<bool> {
        let n = self.side_len();
        let mut tallest_tree = Grid::new(n);
        for y in 0..n {
            for x in 0..n {
                let val = if y == 0 {
                    self.get(x, y)
                } else {
                    max(self.get(x, y), tallest_tree.get(x, y - 1))
                };
                tallest_tree.set(x, y, val);
            }
        }
        let mut visible = Grid::new(n);
        for y in 0..n {
            for x in 0..n {
                let val = (y == 0) || self.get(x, y) > tallest_tree.get(x, y - 1);
                visible.set(x, y, val);
            }
        }
        visible
    }

    pub fn visible_from_bottom(&self) -> Grid<bool> {
        let n = self.side_len();
        let mut tallest_tree = Grid::new(n);
        for y in (0..n).rev() {
            for x in 0..n {
                let val = if y == n - 1 {
                    self.get(x, y)
                } else {
                    max(self.get(x, y), tallest_tree.get(x, y + 1))
                };
                tallest_tree.set(x, y, val);
            }
        }
        tallest_tree.print();
        let mut visible = Grid::new(n);
        for y in 0..n {
            for x in 0..n {
                let val = (y == n - 1) || self.get(x, y) > tallest_tree.get(x, y + 1);
                visible.set(x, y, val);
            }
        }
        visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        assert_eq!(21, Grid::parse(include_str!("../example")).num_visible());
        assert_eq!(1818, Grid::parse(include_str!("../input")).num_visible());
    }

    #[test]
    fn test_visibility() {
        let input = include_str!("../example");
        let trees: Grid<u8> = Grid::parse(input);
        let vis = trees.visible_from_some_direction();
        // All of the trees around the edge of the grid are visible - since they are already on
        // the edge, there are no trees to block the view.
        assert!(vis.get(0, 0));
        assert!(vis.get(1, 0));
        assert!(vis.get(2, 0));
        assert!(vis.get(3, 0));
        assert!(vis.get(4, 0));

        assert!(vis.get(0, 4));
        assert!(vis.get(1, 4));
        assert!(vis.get(2, 4));
        assert!(vis.get(3, 4));
        assert!(vis.get(4, 4));

        assert!(vis.get(0, 0));
        assert!(vis.get(0, 1));
        assert!(vis.get(0, 2));
        assert!(vis.get(0, 3));
        assert!(vis.get(0, 4));

        assert!(vis.get(4, 0));
        assert!(vis.get(4, 1));
        assert!(vis.get(4, 2));
        assert!(vis.get(4, 3));
        assert!(vis.get(4, 4));

        let l = trees.visible_from_left();
        let r = trees.visible_from_right();
        let b = trees.visible_from_bottom();
        let t = trees.visible_from_top();
        // In this example, that only leaves the interior nine trees to consider:
        // The top-left 5 is visible from the left and top.
        assert!(l.get(1, 1));
        assert!(t.get(1, 1));
        // (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
        assert!(!r.get(1, 1));
        assert!(!b.get(1, 1));
        // The top-middle 5 is visible from the top and right.
        assert!(r.get(2, 1));
        assert!(t.get(2, 1));
        assert!(!l.get(2, 1));
        assert!(!b.get(2, 1));
        // The top-right 1 is not visible from any direction; for it to be visible, there would need
        // to only be trees of height 0 between it and an edge.
        assert!(!r.get(3, 1));
        assert!(!b.get(3, 1));
        assert!(!l.get(3, 1));
        assert!(!t.get(3, 1));
        // The left-middle 5 is visible, but only from the right.
        assert!(r.get(1, 2));
        assert!(!b.get(1, 2));
        assert!(!l.get(1, 2));
        assert!(!t.get(1, 2));
        // The center 3 is not visible from any direction; for it to be visible, there would need to
        // be only trees of at most height 2 between it and an edge.
        assert!(!r.get(2, 2));
        assert!(!b.get(2, 2));
        assert!(!l.get(2, 2));
        assert!(!t.get(2, 2));
    }
}
