use std::collections::HashSet;

fn main() {
    let input: Vec<char> = include_str!("../input").chars().collect();
    println!("Q1: {}", solve(&input, 4));
    println!("Q2: {}", solve(&input, 14));
}

fn solve(datastream: &[char], n: usize) -> usize {
    (n..=datastream.len())
        .find(|i| {
            let num_unique_chars = datastream[i - n..*i].iter().collect::<HashSet<_>>().len();
            num_unique_chars == n
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let input: Vec<char> = include_str!("../input").chars().collect();
        assert_eq!(1802, solve(&input, 4));
    }

    #[test]
    fn test_q2() {
        let input: Vec<char> = include_str!("../input").chars().collect();
        assert_eq!(3551, solve(&input, 14));
    }
}
