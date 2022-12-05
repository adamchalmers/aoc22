mod parse;

fn main() {
    let (a1, a2) = solve(include_bytes!("../input"));
    println!("Q1: {a1}");
    println!("Q2: {a2}");
}

/// Returns answer to q1 and q2, given the problem input.
fn solve(input: &[u8]) -> (String, String) {
    let (row, rearrangements) = parse::entire_input(input);

    // Solve q1
    let mut row_q1 = row.clone();
    for rearrangement in &rearrangements {
        row_q1.move_crates(rearrangement, false);
    }
    let answer1 = row_q1.top_of_each_column();

    // Solve q2
    let mut row_q2 = row;
    for rearrangement in &rearrangements {
        row_q2.move_crates(rearrangement, true);
    }
    let answer2 = row_q2.top_of_each_column();

    (answer1, answer2)
}

/// Each element of the vec is a column of crates. Columns may be empty.
#[derive(Debug, Clone)]
pub struct Row(Vec<Vec<char>>);

impl Row {
    /// Put `other` on top of `self`.
    fn stack(mut self, Self(other): Self) -> Self {
        for (i, column) in other.iter().enumerate() {
            self.0[i].extend(column);
        }
        self
    }

    /// Make a string from the character labelling the top crate of every column.
    fn top_of_each_column(&self) -> String {
        self.0
            .iter()
            .map(|column| match column.last() {
                Some(char) => String::from(*char),
                None => Default::default(),
            })
            .collect()
    }

    /// Use the crane to rearrange crates.
    fn move_crates(&mut self, rearrangement: &Rearrangement, reverse: bool) {
        let mut buf = Vec::new();
        for _ in 0..rearrangement.qty {
            let to_move = self.0[rearrangement.src]
                .pop()
                .expect("not enough crates in column");
            buf.push(to_move);
        }
        if reverse {
            buf.reverse();
        }
        self.0[rearrangement.dst].extend(buf);
    }
}

/// Move a certain quantity of crates from column number 'src' to column number 'dst'.
/// Uses 0-based indices for 'src' and 'dst'.
pub struct Rearrangement {
    qty: usize,
    src: usize,
    dst: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_bytes!("../example");
        let (a1, a2) = solve(input);
        assert_eq!(a1, "CMZ");
        assert_eq!(a2, "MCD");
    }
}
