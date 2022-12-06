/*
A Y --> Rock < Paper
B X --> Paper > Rock
C Z --> Scissors = Scissors

Single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

*/

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<char> for Move {
    fn from(move_id: char) -> Self {
        match move_id {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => panic!("Invalid move: {move_id}"),
        }
    }
}

impl Move {
    fn wins(self, other: Move) -> bool {
        match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => true,
            _ => false,
        }
    }

    fn outcome(self, opponent: Move) -> Outcome {
        if self.wins(opponent) {
            Outcome::Win
        } else if opponent.wins(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opponent: Move,
    mine: Move,
}

impl FromStr for Round {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(opponent), Some(' '), Some(mine), None) = (chars.next(), chars.next(), chars.next(), chars.next())
        else {
            return Err(String::from("Could not parse round."));
        };
        Ok(Self {
            opponent: opponent.into(),
            mine: mine.into(),
        })
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.mine.outcome(self.opponent)
    }

    fn my_score(self) -> u64 {
        self.mine as u64 + self.outcome() as u64
    }
}

struct Round2 {
    opponent: Move,
    desired_outcome: Outcome,
}

impl FromStr for Round2 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(opponent), Some(' '), Some(desired_outcome), None) = (chars.next(), chars.next(), chars.next(), chars.next())
        else {
            return Err(String::from("Could not parse round."));
        };
        Ok(Self {
            opponent: opponent.into(),
            desired_outcome: desired_outcome.into(),
        })
    }
}

impl Round2 {
    fn my_move(&self) -> Move {
        match (self.opponent, self.desired_outcome) {
            (Move::Rock, Loss) => Move::Scissors,
            (Move::Rock, Win) => Move::Paper,
            (Move::Paper, Loss) => Move::Rock,
            (Move::Paper, Win) => Move::Scissors,
            (Move::Scissors, Loss) => Move::Paper,
            (Move::Scissors, Win) => Move::Rock,
            (_, Draw) => self.opponent,
        }
    }

    fn my_score(&self) -> u64 {
        self.my_move() as u64 + self.desired_outcome as u64
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

use Outcome::*;
impl From<char> for Outcome {
    fn from(move_id: char) -> Self {
        match move_id {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Invalid move: {move_id}"),
        }
    }
}

fn main() {
    let score: u64 = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Round>().unwrap())
        .map(|round| round.my_score())
        .sum();
    println!("My total score is {score}!");

    let score2: u64 = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Round2>().unwrap())
        .map(|round| round.my_score())
        .sum();
    println!("My new total score is {score2}!");
}
