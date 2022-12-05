use anyhow::{anyhow, Result};
use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::str::FromStr;

struct Assignment {
    sections: std::ops::RangeInclusive<u32>,
}

impl Assignment {
    fn fully_overlaps_with(&self, other: &Self) -> bool {
        self.sections.contains(other.sections.start())
            && self.sections.contains(other.sections.end())
            || other.sections.contains(self.sections.start())
                && other.sections.contains(self.sections.end())
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        self.sections.contains(other.sections.start())
            || self.sections.contains(other.sections.end())
            || other.sections.contains(self.sections.start())
            || other.sections.contains(self.sections.end())
    }
}

impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .ok_or_else(|| anyhow!("expected X-Y formatted range, got {}", s))?;

        let start: u32 = str::parse(start)?;
        let end: u32 = str::parse(end)?;

        Ok(Assignment {
            sections: start..=end,
        })
    }
}

#[aoc(day4, part1)]
pub fn part_1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| str::parse::<Assignment>(range).expect("numeric range"))
                .collect_tuple::<(Assignment, Assignment)>()
                .expect("pair of ranges")
        })
        .filter(|(a, b)| a.fully_overlaps_with(b))
        .count())
}

#[aoc(day4, part2)]
pub fn part_2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| str::parse::<Assignment>(range).expect("numeric range"))
                .collect_tuple::<(Assignment, Assignment)>()
                .expect("pair of ranges")
        })
        .filter(|(a, b)| a.overlaps_with(b))
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE).ok(), Some(2));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE).ok(), Some(4));
    }
}
