use std::collections::HashSet;

use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let sum = input
        .lines()
        .map(|l| {
            let (first, second) = partition_line(l);
            let in_both: Vec<_> = first.intersection(&second).collect();
            // According to the instructions, there will always be one char common to both
            // rucksacks.
            if in_both.len() != 1 {
                panic!("line didn't have exactly one character that was in both rucksacks");
            }
            priority(*in_both[0])
        })
        .sum::<usize>();
    Ok(format!("{}", sum))
}

/// Gets the contents of the two "rucksacks"
fn partition_line(line: &str) -> (HashSet<char>, HashSet<char>) {
    let mid_point = line.len() / 2;
    let (first, second) = line.split_at(mid_point);
    (first.chars().collect(), second.chars().collect())
}

fn priority(c: char) -> usize {
    const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    CHARS
        .find(c)
        .expect("input contained something other than a letter")
        + 1
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1_gives_correct_answer() {
        assert_eq!(part1(INPUT).unwrap(), "157".to_string());
    }
}
