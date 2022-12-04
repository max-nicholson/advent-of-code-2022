use anyhow::{anyhow, Result};
use std::{collections::HashSet, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

struct Item {}

impl Item {
    fn priority(item: char) -> Result<u32> {
        let ord: u32 = item.try_into()?;
        match item {
            'a'..='z' => Ok(ord - 96),
            'A'..='Z' => Ok(ord - 38),
            _ => Err(anyhow!(anyhow!("expected a-z/A-Z, got {}", item))),
        }
    }
}

#[derive(Debug, Clone)]
struct Compartment {
    items: HashSet<char>,
}

impl From<&[char]> for Compartment {
    fn from(chars: &[char]) -> Self {
        Compartment {
            items: chars.iter().cloned().collect::<_>(),
        }
    }
}

fn find_single_common_item<'a>(mut intersection: impl Iterator<Item = &'a char>) -> Result<char> {
    match (intersection.next(), intersection.next()) {
        (Some(&item), None) => Ok(item),
        (None, _) => Err(anyhow!("no common item")),
        (Some(item), Some(other_item)) => Err(anyhow!(
            "got more than one common item, {} and {}",
            item,
            other_item
        )),
    }
}

impl Compartment {
    fn common_item(&self, other: &Self) -> Result<char> {
        let intersection = self.items.intersection(&other.items);
        find_single_common_item(intersection)
    }
}

#[derive(Debug, Clone)]
pub struct Rucksack {
    compartments: [Compartment; 2],
}

impl FromStr for Rucksack {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let chars: Vec<_> = s.chars().collect();
        let (first_half, second_half) = chars.split_at(chars.len() / 2);
        Ok(Rucksack {
            compartments: [
                Compartment::from(first_half),
                Compartment::from(second_half),
            ],
        })
    }
}

impl Rucksack {
    fn common_item(&self) -> Result<char> {
        self.compartments[0].common_item(&self.compartments[1])
    }

    fn items(&self) -> HashSet<char> {
        self.compartments[0]
            .items
            .union(&self.compartments[1].items)
            .cloned()
            .collect()
    }
}

struct Group {
    rucksacks: Vec<Rucksack>,
}

impl Group {
    const SIZE: usize = 3;

    fn common_item(&self) -> Result<char> {
        let (first, rest) = self.rucksacks.split_at(1);
        let intersection = &mut first[0].items();
        for rucksack in rest {
            let items = rucksack.items();
            intersection.retain(|e| items.contains(e));
        }

        find_single_common_item(intersection.iter())
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| line.parse::<Rucksack>().unwrap())
        .collect()
}

#[aoc(day3, part1)]
pub fn part_1(rucksacks: &[Rucksack]) -> Result<u32> {
    let common_items: Result<Vec<char>> = rucksacks
        .iter()
        .map(|rucksack| rucksack.common_item())
        .collect();

    match common_items {
        Ok(common_items) => Ok(common_items
            .into_iter()
            .map(Item::priority)
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .sum()),
        Err(e) => Err(e),
    }
}

#[aoc(day3, part2)]
pub fn part_2(rucksacks: &[Rucksack]) -> Result<u32> {
    let priority_by_group = rucksacks
        .chunks(Group::SIZE)
        .map(|slice| {
            let group = Group {
                rucksacks: slice.to_vec(),
            };
            let common_item = group.common_item()?;
            Item::priority(common_item)
        })
        .collect::<Result<Vec<u32>>>()?;
    Ok(priority_by_group.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE)).ok(), Some(157));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE)).ok(), Some(70));
    }
}
