use std::str;

use nom::{
    bytes::complete::take_while,
    character::{
        complete::{char, line_ending},
        is_digit,
    },
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_bytes!("../input.txt");
    let (q1, q2) = solve(input);
    println!("q1: {q1}");
    println!("q2: {q2}");
}

/// Parse the input, find how many ranges overlap
/// (returning q1 and q2's definition of 'overlap' respectively)
fn solve(input: &[u8]) -> (usize, usize) {
    let ranges = parse_input(input);
    let q1 = ranges
        .iter()
        .filter(|q| matches!(q, Overlap::Complete))
        .count();
    let q2 = ranges
        .iter()
        .filter(|q| matches!(q, Overlap::Partial | Overlap::Complete))
        .count();
    (q1, q2)
}

/// One elf's selection assignment -- an inclusive range of section IDs.
type Range = (usize, usize);

/// Do the two ranges overlap each other at all?
enum Overlap {
    /// e.g. 2-4, 6-8
    None,
    /// e.g. 5-7, 7-9 overlaps in a single section, 7.
    Partial,
    /// e.g. 2-8, 3-7 overlaps all of the sections 3 through 7.
    Complete,
}

/// You can calculate the kind of overlap that two ranges have.
impl From<(Range, Range)> for Overlap {
    fn from(((a, b), (c, d)): (Range, Range)) -> Self {
        if (a >= c && b <= d) || (c >= a && d <= b) {
            return Self::Complete;
        }
        if (b >= c && b <= d) || (d >= a && d <= b) {
            return Self::Partial;
        }
        Self::None
    }
}

/// For each line in the input, parse the two ranges, and calculate their overlap.
fn parse_input(input: &[u8]) -> Vec<Overlap> {
    fn parse_num(i: &[u8]) -> IResult<&[u8], usize> {
        map_res(take_while(is_digit), |digits: &[u8]| {
            str::from_utf8(digits).unwrap().parse()
        })(i)
    }

    fn parse_range(i: &[u8]) -> IResult<&[u8], Range> {
        separated_pair(parse_num, char('-'), parse_num)(i)
    }

    fn parse_line(i: &[u8]) -> IResult<&[u8], Overlap> {
        map(
            separated_pair(parse_range, char(','), parse_range),
            Overlap::from,
        )(i)
    }

    let mut parser = separated_list1(line_ending, parse_line);
    let (_remaining_input, range) = parser(input).expect("could not parse the problem input");
    range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_bytes!("../example.txt");
        let (q1, q2) = solve(input);
        assert_eq!(q1, 2);
        assert_eq!(q2, 4);
    }
}
