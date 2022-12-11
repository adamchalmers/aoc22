use std::collections::HashSet;

fn main() {
    let moves = parse(include_str!("../input"));
    println!("Q1: {}", simulate(&moves, false, 2));
    println!("Q2: {}", simulate(&moves, false, 10));
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    #[must_use]
    fn move_dir(mut self, dir: Dir) -> Self {
        match dir {
            Dir::U => self.y += 1,
            Dir::D => self.y -= 1,
            Dir::L => self.x -= 1,
            Dir::R => self.x += 1,
        }
        self
    }

    // Move this point, the tail, towards the given head.
    #[must_use]
    fn move_towards(self, head: &Self) -> Self {
        let mut tail = self;

        // If they're touching, no move is necessary.
        // Diagonally adjacent and even overlapping both count as touching
        if head.x.abs_diff(tail.x) <= 1 && head.y.abs_diff(tail.y) <= 1 {
            return tail;
        }

        // If the head is ever two steps directly up, down, left, or right from the tail, the tail
        // must also move one step in that direction so it remains close enough.
        if head.x == tail.x {
            if head.y == tail.y + 2 {
                tail.y += 1;
                return tail;
            }
            if head.y == tail.y - 2 {
                tail.y -= 1;
                return tail;
            }
        }
        if head.y == tail.y {
            if head.x == tail.x + 2 {
                tail.x += 1;
                return tail;
            }
            if head.x == tail.x - 2 {
                tail.x -= 1;
                return tail;
            }
        }

        // Otherwise, move diagonally towards the head.

        // Is tail's x too small?
        if head.x > tail.x {
            tail.x += 1;
        } else {
            tail.x -= 1;
        }
        // Is tail's y too small?
        if head.y > tail.y {
            tail.y += 1;
        } else {
            tail.y -= 1;
        }

        tail
    }
}

/// Returns number of positions the tail of the rope visits at least once.
/// Simulates `n` points.
fn simulate(moves: &[Move], print_steps: bool, knots: usize) -> usize {
    let mut points_visited = HashSet::new();
    // The head is #0, the tail is #n.
    let mut points = vec![Point::default(); knots];
    for mv in moves {
        if print_steps {
            mv.print();
        }
        for _step in 0..mv.steps {
            points[0] = points[0].move_dir(mv.dir);
            for i in 1..points.len() {
                let head = points[i - 1];
                let tail = points[i];
                points[i] = tail.move_towards(&head);
                if print_steps {
                    print_grid(points[i - 1], points[i]);
                }
            }
            points_visited.insert(points.last().unwrap().to_owned());
        }
    }
    points_visited.len()
}

fn print_grid(head: Point, tail: Point) {
    const WIDTH: usize = 6;
    for y in (0..WIDTH).rev() {
        for x in 0..WIDTH {
            let p = Point {
                x: x as _,
                y: y as _,
            };
            if p == Default::default() {
                print!("s");
            } else if p == head {
                print!("H");
            } else if p == tail {
                print!("T");
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!()
}

#[derive(Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Dir::U => "U",
            Dir::D => "D",
            Dir::L => "L",
            Dir::R => "R",
        };
        s.fmt(f)
    }
}

#[derive(Clone, Copy)]
struct Move {
    dir: Dir,
    steps: isize,
}

impl Move {
    fn print(&self) {
        println!("== {} {} ==", self.dir, self.steps);
    }
}

fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, distance)| Move {
            steps: distance.parse::<u8>().unwrap() as _,
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

    #[test]
    fn test_example_q1() {
        let input = include_str!("../example");
        let moves = parse(input);
        let q1 = simulate(&moves, true, 2);
        assert_eq!(q1, 13);
    }

    #[test]
    fn test_real_q1() {
        let input = include_str!("../input");
        let moves = parse(input);
        assert_eq!(simulate(&moves, false, 2), 6367);
        assert_eq!(simulate(&moves, false, 10), 2536);
    }
}
