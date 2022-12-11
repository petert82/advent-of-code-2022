use std::ops::RangeInclusive;

use anyhow::{bail, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map, map_res, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn part1(input: &str) -> Result<usize> {
    let input_ranges = parse_lines(input)?;
    let count = input_ranges
        .iter()
        .filter(|(r1, r2)| {
            (r1.contains(r2.start()) && r1.contains(r2.end()))
                || (r2.contains(r1.start()) && r2.contains(r1.end()))
        })
        .count();
    Ok(count)
}

pub fn part2(input: &str) -> Result<usize> {
    let input_ranges = parse_lines(input)?;
    let count = input_ranges
        .iter()
        .filter(|(r1, r2)| {
            r1.contains(r2.start())
                || r1.contains(r2.end())
                || r2.contains(r1.start())
                || r2.contains(r1.end())
        })
        .count();
    Ok(count)
}

fn number(digits: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse::<usize>())(digits)
}

fn range_inclusive(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    map(separated_pair(number, tag("-"), number), |(start, end)| {
        start..=end
    })(input)
}

fn parse_line(line: &str) -> IResult<&str, (RangeInclusive<usize>, RangeInclusive<usize>)> {
    separated_pair(range_inclusive, tag(","), range_inclusive)(line)
}

fn parse_lines(lines: &str) -> Result<Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>> {
    let list = separated_list1(line_ending, parse_line);
    let Ok((_rest, parsed)) = all_consuming(terminated(list, opt(line_ending)))(lines) else {
        bail!("Failed to parse input");
    };
    Ok(parsed)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1_gives_correct_answer() {
        assert_eq!(part1(INPUT).unwrap(), 2);
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        assert_eq!(part2(INPUT).unwrap(), 4);
    }
}
