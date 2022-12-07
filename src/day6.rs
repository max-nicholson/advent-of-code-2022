use std::{collections::HashSet, ops::Index};

use anyhow::{anyhow, Result};
use aoc_runner_derive::aoc;

fn parse_datastream(input: &str) -> Result<&str> {
    input
        .lines()
        .next()
        .ok_or_else(|| anyhow!("empty datastream"))
}

fn find_start_of_message_marker(datastream: &str, length_of_unique_marker: usize) -> Result<usize> {
    let mut marker = None;
    for (i, window) in datastream
        .as_bytes()
        .windows(length_of_unique_marker)
        .enumerate()
    {
        let unique: HashSet<&u8> = HashSet::from_iter(window);
        if unique.len() == length_of_unique_marker {
            marker = Some(i + length_of_unique_marker);
            break;
        }
    }

    marker.ok_or_else(|| anyhow!("no start-of-packet marker found"))
}

#[aoc(day6, part1)]
pub fn part_1(input: &str) -> Result<usize> {
    let datastream = parse_datastream(input)?;

    let length_of_unique_marker = 4;
    find_start_of_message_marker(datastream, length_of_unique_marker)
}

#[aoc(day6, part2)]
pub fn part_2(input: &str) -> Result<usize> {
    let datastream = parse_datastream(input)?;

    let length_of_unique_marker = 14;
    find_start_of_message_marker(datastream, length_of_unique_marker)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE).ok(), Some(7));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE).ok(), Some(19));
    }
}
