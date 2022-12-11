use std::collections::HashSet;

fn main() {
    let moves = parse(include_str!("../input"));
    let a1 = simulate(&moves);
    println!("Q1: {a1}");
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn move_by(self, Move { dir, n }: &Move) -> Self {
        let Self { mut x, mut y } = self;
        match dir {
            Dir::U => y += n,
            Dir::D => y -= n,
            Dir::L => x += n,
            Dir::R => x -= n,
        }
        Self { x, y }
    }

    // Move this point, the tail, towards the given head.
    fn move_towards(self, head: Self) -> Self {}
}

/// Returns number of positions the tail of the rope visits at least once
fn simulate(moves: &[Move]) -> usize {
    let mut points_visited = HashSet::new();
    let mut head = Point::default();
    let mut tail = Point::default();
    for mv in moves {
        points_visited.insert(tail);
        head = head.move_by(mv);
        tail = tail.move_towards(head);
    }
    points_visited.len()
}

#[derive(Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Clone, Copy)]
struct Move {
    dir: Dir,
    n: isize,
}

fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, distance)| Move {
            n: distance.parse::<u8>().unwrap() as _,
            dir: match dir {
                "D" => Dir::D,
                "U" => Dir::U,
                "R" => Dir::R,
                "L" => Dir::L,
                _ => unreachable!(),
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../input");
        assert_eq!(2000, parse(input).len());
    }
}
