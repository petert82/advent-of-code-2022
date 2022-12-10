use std::collections::{HashSet, VecDeque};

use anyhow::{bail, Result};

pub fn part1(input: &str) -> Result<usize> {
    solve(input, 4)
}

pub fn part2(input: &str) -> Result<usize> {
    solve(input, 14)
}

/// Find the (1-indexed) position of the end of the first sequence of `find_len` contiguous
/// distinct characters in `input`
fn solve(input: &str, find_len: usize) -> Result<usize> {
    let mut deq = VecDeque::with_capacity(find_len);
    let mut diff_chars = HashSet::with_capacity(find_len);

    for (i, c) in input.chars().enumerate() {
        deq.push_back(c);
        if deq.len() == find_len {
            // convert the characters from the Deque to a Set
            for recent_c in deq.iter() {
                diff_chars.insert(*recent_c);
            }
            // Check if we have expected number of distinct characters
            if diff_chars.len() == find_len {
                return Ok(i + 1);
            }
            // Didn't have right number of distinct letters
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

    #[test]
    pub fn test_part_2_gives_correct_answer() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 26);
    }
}
