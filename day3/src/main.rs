#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn main() {
    println!("q1: {}", q1(include_str!("../input.txt")));
    println!("q2: {}", q2(include_str!("../input.txt")));
}

fn q2(input: &str) -> u64 {
    input
        .lines()
        .array_chunks()
        .map(|[a, b, c]| {
            let intersection = &(&set(a) & &set(b)) & &set(c);
            let duplicate = intersection.iter().next().unwrap().to_owned();
            priority(duplicate)
        })
        .sum()
}

fn q1(input: &str) -> u64 {
    input
        .lines()
        .map(|rucksack| -> u64 {
            let (l, r) = rucksack.split_at(rucksack.len() / 2);
            assert_eq!(l.len(), r.len());
            let duplicate = set(l).intersection(&set(r)).next().unwrap().to_owned();
            priority(duplicate)
        })
        .sum()
}

fn set(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn priority(c: char) -> u64 {
    let a_upper: u64 = 'A'.into();
    let a_lower: u64 = 'a'.into();
    let uint: u64 = c.into();
    if c.is_uppercase() {
        uint - a_upper + 27
    } else {
        uint - a_lower + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
    }

    #[test]
    fn test_q1() {
        assert_eq!(q1(include_str!("../example.txt")), 157)
    }

    #[test]
    fn test_q2() {
        assert_eq!(q2(include_str!("../example.txt")), 70)
    }
}
