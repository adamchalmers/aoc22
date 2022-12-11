use super::Grid;

impl Grid<u8> {
    pub fn scenic_score(&self, x: usize, y: usize) -> u32 {
        self.trees_visible_above(x, y)
            * self.trees_visible_below(x, y)
            * self.trees_visible_left(x, y)
            * self.trees_visible_right(x, y)
    }

    pub fn max_scenic_score(&self) -> u32 {
        let mut max = 0;
        let n = self.side_len();
        for x in 0..n {
            for y in 0..n {
                let curr = self.scenic_score(x, y);
                if curr > max {
                    max = curr
                }
            }
        }
        max
    }

    fn trees_visible_above(&self, x: usize, y: usize) -> u32 {
        if y == 0 {
            return 0;
        }
        let mut visible = 0;
        for iy in (0..y).rev() {
            visible += 1;
            if self.get(x, iy) >= self.get(x, y) {
                break;
            }
        }
        visible
    }

    fn trees_visible_below(&self, x: usize, y: usize) -> u32 {
        if y == self.side_len() - 1 {
            return 0;
        }
        let mut visible = 0;
        for iy in y + 1..self.side_len() {
            visible += 1;
            if self.get(x, iy) >= self.get(x, y) {
                break;
            }
        }
        visible
    }

    fn trees_visible_left(&self, x: usize, y: usize) -> u32 {
        if x == 0 {
            return 0;
        }
        let mut visible = 0;
        for ix in (0..x).rev() {
            visible += 1;
            if self.get(ix, y) >= self.get(x, y) {
                break;
            }
        }
        visible
    }

    fn trees_visible_right(&self, x: usize, y: usize) -> u32 {
        if x == self.side_len() - 1 {
            return 0;
        }
        let mut visible = 0;
        for ix in x + 1..self.side_len() {
            visible += 1;
            if self.get(ix, y) >= self.get(x, y) {
                break;
            }
        }
        visible
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_scores_example() {
        let input = include_str!("../example");
        let trees: Grid<u8> = Grid::parse(input);
        assert_eq!(trees.max_scenic_score(), 8);
    }

    #[test]
    fn test_scores_real() {
        let input = include_str!("../input");
        let trees: Grid<u8> = Grid::parse(input);
        assert_eq!(trees.max_scenic_score(), 368368);
    }
}
