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

/// Gets the contents of the two "rucksack compartments"
fn partition_line(line: &str) -> (HashSet<char>, HashSet<char>) {
    let mid_point = line.len() / 2;
    let (first, second) = line.split_at(mid_point);
    (first.chars().collect(), second.chars().collect())
}

pub fn part2(input: &str) -> Result<String> {
    let mut lines = input.lines();

    let mut sum: usize = 0;
    loop {
        // Get the 3 lines for the group of elves
        let first = lines.next();
        let second = lines.next();
        let third = lines.next();
        if first.is_none() || second.is_none() || third.is_none() {
            break;
        }
        // Split lines to chars
        let first: HashSet<_> = first.unwrap().chars().collect();
        let second: HashSet<_> = second.unwrap().chars().collect();
        let third: HashSet<_> = third.unwrap().chars().collect();

        // Find the character that is in all three lines
        let in_both1: HashSet<_> = first.intersection(&second).map(|c| c.to_owned()).collect();
        let in_all: Vec<_> = third.intersection(&in_both1).collect();
        if in_all.len() != 1 {
            panic!("got three lines that did not share a common character");
        }
        sum += priority(*in_all[0]);
    }

    Ok(format!("{}", sum))
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

    #[test]
    fn test_part2_gives_correct_answer() {
        assert_eq!(part2(INPUT).unwrap(), "70".to_string());
    }
}
