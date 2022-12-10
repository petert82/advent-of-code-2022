use std::collections::{HashSet, VecDeque};

use anyhow::{bail, Result};

pub fn part1(input: &str) -> Result<usize> {
    let mut deq = VecDeque::with_capacity(4);
    let mut diff_chars = HashSet::with_capacity(4);

    for (i, c) in input.chars().enumerate() {
        deq.push_back(c);
        if deq.len() == 4 {
            // convert the 4 characters from the Deque to a Set
            for recent_c in deq.iter() {
                diff_chars.insert(*recent_c);
            }
            // Check if we have 4 distinct characters
            if diff_chars.len() == 4 {
                return Ok(i + 1);
            }
            // Didn't have 4 distinct letters
            diff_chars.clear();
            deq.pop_front();
        }
    }
    bail!("Did not find 4 different characters adjacent to each other");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_part_1_gives_correct_answer() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
    }
}
