use super::Grid;

type Count = u8;

impl Grid<u8> {
    pub fn scenic_score(&self) -> Grid<Count> {
        let u = self.trees_visible_above();
        let d = self.trees_visible_below();
        let l = self.trees_visible_left();
        let r = self.trees_visible_right();
        let n = self.side_len();
        let mut out = Grid::new(n);
        for x in 0..n {
            for y in 0..n {
                let val = u.get(x, y) * d.get(x, y) * l.get(x, y) * r.get(x, y);
                out.set(x, y, val);
            }
        }
        out
    }

    /// How many trees can be seen from below, for each tree?
    /// O(n^2)
    fn trees_visible_below(&self) -> Grid<Count> {
        let n = self.side_len();
        let mut trees_visible = Grid::new(n);
        // All trees along the bottom row can see 0 trees below.
        for x in 0..n {
            trees_visible.set(x, n - 1, 0);
        }
        for y in (0..n - 1).rev() {
            for x in 0..n {
                let other = self.get(x, y + 1);
                // If `other` is shorter, then `self` can see as many trees as `other` can,
                // plus one more (`other` itself.)
                let this = if self.get(x, y) > other {
                    trees_visible.get(x, y + 1) + 1
                } else {
                    1
                };
                trees_visible.set(x, y, this);
            }
        }
        trees_visible
    }

    fn trees_visible_above(&self) -> Grid<Count> {
        let n = self.side_len();
        let mut trees_visible = Grid::new(n);
        // All trees along the top row can see 0 trees above.
        for x in 0..n {
            trees_visible.set(x, 0, 0);
        }
        for y in 1..n {
            for x in 0..n {
                let other = self.get(x, y - 1);
                // If `other` is shorter, then `self` can see as many trees as `other` can,
                // plus one more (`other` itself.)
                let this = if self.get(x, y) > other {
                    trees_visible.get(x, y - 1) + 1
                } else {
                    1
                };
                trees_visible.set(x, y, this);
            }
        }
        trees_visible
    }

    fn trees_visible_left(&self) -> Grid<Count> {
        let n = self.side_len();
        let mut trees_visible = Grid::new(n);
        // All trees along the left column can see 0 trees to the left.
        for y in 0..n {
            trees_visible.set(0, y, 0);
        }
        for x in 1..n {
            for y in 0..n {
                let other = self.get(x - 1, y);
                // If `other` is shorter, then `self` can see as many trees as `other` can,
                // plus one more (`other` itself.)
                let this = if self.get(x, y) > other {
                    trees_visible.get(x - 1, y) + 1
                } else {
                    1
                };
                trees_visible.set(x, y, this);
            }
        }
        trees_visible
    }
    fn trees_visible_right(&self) -> Grid<Count> {
        let n = self.side_len();
        let mut trees_visible = Grid::new(n);
        // All trees along the right column can see 0 trees to the right.
        for y in 0..n {
            trees_visible.set(n - 1, y, 0);
        }
        for x in (0..n - 1).rev() {
            for y in 0..n {
                let other = self.get(x + 1, y);
                // If `other` is shorter, then `self` can see as many trees as `other` can,
                // plus one more (`other` itself.)
                let this = if self.get(x, y) > other {
                    trees_visible.get(x + 1, y) + 1
                } else {
                    1
                };
                trees_visible.set(x, y, this);
            }
        }
        trees_visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_below() {
        let input = include_str!("../example");
        let trees: Grid<u8> = Grid::parse(input);
        let vis = trees.trees_visible_below();
        // All of the trees around the edge of the grid are visible - since they are already on
        // the edge, there are no trees to block the view.
        assert_eq!(vis.get(0, 4), 0);
        assert_eq!(vis.get(1, 4), 0);
        assert_eq!(vis.get(2, 4), 0);
        assert_eq!(vis.get(3, 4), 0);
        assert_eq!(vis.get(4, 4), 0);

        // From the example
        assert_eq!(vis.get(2, 1), 2);
        assert_eq!(vis.get(2, 3), 1);
    }

    #[test]
    fn test_above() {
        let input = include_str!("../example");
        let trees: Grid<u8> = Grid::parse(input);
        let vis = trees.trees_visible_above();
        // All of the trees around the edge of the grid are visible - since they are already on
        // the edge, there are no trees to block the view.
        assert_eq!(vis.get(0, 0), 0);
        assert_eq!(vis.get(1, 0), 0);
        assert_eq!(vis.get(2, 0), 0);
        assert_eq!(vis.get(3, 0), 0);
        assert_eq!(vis.get(4, 0), 0);

        // From the example
        assert_eq!(vis.get(2, 1), 1);
        assert_eq!(vis.get(2, 3), 2);
    }

    #[test]
    fn test_scores() {
        let input = include_str!("../example");
        let trees: Grid<u8> = Grid::parse(input);
        let scenic_scores = trees.scenic_score();
        assert_eq!(scenic_scores.get(2, 1), 4);
        assert_eq!(scenic_scores.get(2, 3), 8);
    }
}
