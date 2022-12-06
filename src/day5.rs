use anyhow::{anyhow, Result};
use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Move {
    from: usize,
    to: usize,
    quantity: usize,
}

fn parse_stacks(stacks: &str) -> Result<Vec<Vec<char>>> {
    let lines = stacks.lines().collect::<Vec<&str>>();
    let height = lines.len() - 1;
    let columns = (lines[lines.len() - 1].len() + 1) / 3;

    let mut stacks = Vec::with_capacity(columns);
    (0..columns).for_each(|_| {
        let arr = Vec::with_capacity(height);
        stacks.push(arr);
    });

    (1..=height).rev().for_each(|row| {
        let line = lines[row - 1].chars().collect::<Vec<char>>();
        for column in 0..columns {
            let index = column * 4 + 1;
            if index > (line.len() - 1) {
                break;
            }
            let maybe_crate = line[column * 4 + 1];
            if let 'A'..='Z' = maybe_crate {
                stacks[column].push(maybe_crate)
            }
        }
    });

    Ok(stacks)
}

fn parse_moves(moves: &str) -> Result<Vec<Move>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("regex to compile");
    }

    moves
        .lines()
        .map(|line| {
            let groups = RE
                .captures(line)
                .ok_or_else(|| anyhow!("couldn't parse step procedure, got {}", line))?;
            Ok(Move {
                quantity: str::parse(&groups[1])?,
                from: str::parse(&groups[2])?,
                to: str::parse(&groups[3])?,
            })
        })
        .collect::<Result<Vec<Move>>>()
}

pub fn parse_input(input: &str) -> Result<(Vec<Vec<char>>, Vec<Move>)> {
    let (state, moves) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("expected stacks and moves separated by empty line"))?;

    Ok((parse_stacks(state)?, parse_moves(moves)?))
}

fn top_crate_of_each_stack(mut stacks: Vec<Vec<char>>) -> String {
    stacks
        .iter_mut()
        .filter_map(|stack| stack.last())
        .collect::<String>()
}

#[aoc(day5, part1)]
pub fn part_1(input: &str) -> Result<String> {
    let (stacks, moves) = parse_input(input)?;

    let stacks = moves.into_iter().fold(stacks, |mut acc, _move| {
        for _ in 0.._move.quantity {
            let removed = acc[_move.from - 1].pop().expect("stack shouldn't be empty");
            acc[_move.to - 1].push(removed);
        }
        acc
    });

    Ok(top_crate_of_each_stack(stacks))
}

#[aoc(day5, part2)]
pub fn part_2(input: &str) -> Result<String> {
    let (stacks, moves) = parse_input(input)?;

    let stacks = moves.into_iter().fold(stacks, |mut acc, _move| {
        let mut queue: Vec<char> = Vec::with_capacity(_move.quantity);
        for _ in 0.._move.quantity {
            let removed = acc[_move.from - 1].pop().expect("stack shouldn't be empty");
            queue.push(removed);
        }
        for i in (0.._move.quantity).rev() {
            acc[_move.to - 1].push(queue[i])
        }
        acc
    });

    Ok(top_crate_of_each_stack(stacks))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE).ok(), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE).ok(), Some("MCD".to_string()));
    }
}
