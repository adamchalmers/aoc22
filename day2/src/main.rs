use std::str::Chars;

fn main() {
    let input = include_str!("../input.txt");
    let (q1, q2) = input.lines().fold((0, 0), |(score_q1, score_q2), line| {
        (
            score_q1 + Moves::parse_q1(line).my_score(),
            score_q2 + Moves::parse_q2(line).my_score(),
        )
    });
    println!("Q1: {q1}");
    println!("Q2: {q2}");
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

struct Moves {
    me: Move,
    them: Move,
}

fn parse_them(chars: &mut Chars) -> Move {
    let them = match chars.next().unwrap() {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => unreachable!(),
    };
    chars.next();
    them
}

impl Moves {
    fn parse_q1(line: &str) -> Self {
        let mut chars = line.chars();
        let them = parse_them(&mut chars);

        // The second column, you reason, must be what you should play in response: X for Rock, Y
        // for Paper, and Z for Scissors.
        let me = match chars.next().unwrap() {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => unreachable!(),
        };
        Self { me, them }
    }

    fn parse_q2(line: &str) -> Self {
        let mut chars = line.chars();
        let them = parse_them(&mut chars);

        // the second column says how the round needs to end: X means you need to lose, Y means you
        // need to end the round in a draw, and Z means you need to win.
        let outcome = match chars.next().unwrap() {
            'X' => Winner::Them,
            'Y' => Winner::Draw,
            'Z' => Winner::Me,
            _ => unreachable!(),
        };
        let me = match (&them, outcome) {
            (Move::Rock, Winner::Me) => Move::Paper,
            (Move::Rock, Winner::Them) => Move::Scissors,
            (Move::Rock, Winner::Draw) => Move::Rock,
            (Move::Paper, Winner::Me) => Move::Scissors,
            (Move::Paper, Winner::Them) => Move::Rock,
            (Move::Paper, Winner::Draw) => Move::Paper,
            (Move::Scissors, Winner::Me) => Move::Rock,
            (Move::Scissors, Winner::Them) => Move::Paper,
            (Move::Scissors, Winner::Draw) => Move::Scissors,
        };

        Self { me, them }
    }

    fn winner(&self) -> Winner {
        match (&self.me, &self.them) {
            (Move::Rock, Move::Rock) => Winner::Draw,
            (Move::Rock, Move::Paper) => Winner::Them,
            (Move::Rock, Move::Scissors) => Winner::Me,
            (Move::Paper, Move::Rock) => Winner::Me,
            (Move::Paper, Move::Paper) => Winner::Draw,
            (Move::Paper, Move::Scissors) => Winner::Them,
            (Move::Scissors, Move::Rock) => Winner::Them,
            (Move::Scissors, Move::Paper) => Winner::Me,
            (Move::Scissors, Move::Scissors) => Winner::Draw,
        }
    }

    /// Your total score is the sum of your scores for each round. The score for a single round is
    /// the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the
    /// score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you
    /// won).
    fn my_score(&self) -> u64 {
        self.me.score()
            + match self.winner() {
                Winner::Me => 6,
                Winner::Them => 0,
                Winner::Draw => 3,
            }
    }
}

enum Winner {
    Me,
    Them,
    Draw,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q2() {
        let rounds = include_str!("../example.txt").lines().map(Moves::parse_q2);
        let total_score: u64 = rounds.map(|moves| moves.my_score()).sum();
        assert_eq!(total_score, 12);
    }

    #[test]
    fn test_q1() {
        let rounds = include_str!("../example.txt").lines().map(Moves::parse_q1);
        let total_score: u64 = rounds.map(|moves| moves.my_score()).sum();
        assert_eq!(total_score, 15);
    }
}
