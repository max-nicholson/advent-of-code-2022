use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_calories(input: &str) -> Vec<Option<usize>> {
    input.lines().map(|n| str::parse(n).ok()).collect()
}

fn group_calories_by_elf(input: &Vec<Option<usize>>) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut group: Vec<usize> = Vec::new();
    for line in input {
        if let &Some(calorie) = line {
            group.push(calorie);
        } else {
            groups.push(group);
            group = Vec::new();
        }
    }

    if group.len() > 0 {
        groups.push(group)
    }

    groups
}

#[aoc(day1, part1)]
pub fn part_1(input: &Vec<Option<usize>>) -> Option<usize> {
    let groups = group_calories_by_elf(input);

    groups.iter().map(|group| group.iter().sum()).max()
}

#[aoc(day1, part2)]
pub fn part_2(input: &Vec<Option<usize>>) -> Option<usize> {
    let groups = group_calories_by_elf(input);

    let mut total_calories_by_elf = groups.iter().map(|group| group.iter().sum()).collect::<Vec<usize>>();
    total_calories_by_elf.sort_by(|a, b| b.cmp(a));

    Some(total_calories_by_elf.iter().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_calories(EXAMPLE)), Some(24_000));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_calories(EXAMPLE)), Some(45_000));
    }
}
