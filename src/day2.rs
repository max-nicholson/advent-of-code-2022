use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Player {
    You,
    Opponent,
}

pub struct Round {
    opponent: char,
    you: char,
}

impl Round {
    fn winner(you: Choice, opponent: Choice) -> Option<Player> {
        if you == opponent {
            return None
        }

        match (you, opponent) {
            (Choice::Paper, Choice::Rock) => Some(Player::You),
            (Choice::Scissors, Choice::Rock) => Some(Player::Opponent),
            (Choice::Rock, Choice::Paper) => Some(Player::Opponent),
            (Choice::Scissors, Choice::Paper) => Some(Player::You),
            (Choice::Rock, Choice::Scissors) => Some(Player::You),
            (Choice::Paper, Choice::Scissors) => Some(Player::Opponent),
            _ => unreachable!(),
        }
    }

    fn score(you: Choice, opponent: Choice) -> usize {
        let score = match Round::winner(you, opponent) {
            Some(Player::Opponent) => 0,
            None => 3,
            Some(Player::You) => 6,
        } + match you {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };
        score
    }
}

lazy_static! {
    static ref MAP_OPPONENT_CHOICE: HashMap<char, Choice> = {
        let mut m = HashMap::new();
        m.insert('A', Choice::Rock);
        m.insert('B', Choice::Paper);
        m.insert('C', Choice::Scissors);
        m
    };
}

fn lose_to(choice: Choice) -> Choice {
    match choice {
        Choice::Rock => Choice::Scissors,
        Choice::Paper => Choice::Rock,
        Choice::Scissors => Choice::Paper,
    }
}

fn draw_with(choice: Choice) -> Choice {
    choice
}

fn beat(choice: Choice) -> Choice {
    match choice {
        Choice::Rock => Choice::Paper,
        Choice::Paper => Choice::Scissors,
        Choice::Scissors => Choice::Rock,
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Round> {
    input.lines().map(|line| {
        let mut parts = line.split(' ');
        let opponent = parts.next().and_then(|x| x.chars().next()).unwrap();
        let you = parts.next().and_then(|x| x.chars().next()).unwrap();
        Round {
            opponent,
            you,
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn part_1(rounds: &[Round]) -> Option<usize> {
    Some(rounds.iter().map(|round| {
        Round::score(
            match round.you {
                'X' => Choice::Rock,
                'Y' => Choice::Paper,
                'Z' => Choice::Scissors,
                _ => unreachable!(),
            },
            *MAP_OPPONENT_CHOICE.get(&round.opponent).unwrap(),
        )
    }).sum())
}

#[aoc(day2, part2)]
pub fn part_2(rounds: &[Round]) -> Option<usize> {
    Some(rounds.iter().map(|round| {
        let opponent_choice = *MAP_OPPONENT_CHOICE.get(&round.opponent).unwrap();
        Round::score(
            match round.you {
                'X' => lose_to(opponent_choice),
                'Y' => draw_with(opponent_choice),
                'Z' => beat(opponent_choice),
                _ => unreachable!(),
            },
            opponent_choice,
        )
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE)), Some(15));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE)), Some(12));
    }
}
