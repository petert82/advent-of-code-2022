use std::ops::RangeInclusive;

use anyhow::{bail, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map_res, opt},
    multi::many0,
    sequence::{preceded, separated_pair},
    IResult,
};

pub fn part1(input: &str) -> Result<String> {
    let input_ranges = parse_lines(input)?;
    let count = input_ranges
        .iter()
        .filter(|(r1, r2)| {
            (r1.contains(r2.start()) && r1.contains(r2.end()))
                || (r2.contains(r1.start()) && r2.contains(r1.end()))
        })
        .count();
    Ok(format!("{}", count))
}

pub fn part2(input: &str) -> Result<String> {
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
    Ok(format!("{}", count))
}

fn number(digits: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse::<usize>())(digits)
}

fn range_inclusive(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (rest, (start, end)) = separated_pair(number, tag("-"), number)(input)?;
    Ok((rest, (start..=end)))
}

fn parse_line(line: &str) -> IResult<&str, (RangeInclusive<usize>, RangeInclusive<usize>)> {
    let (rest, range1) = range_inclusive(line)?;
    let (rest, range2) = preceded(tag(","), range_inclusive)(rest)?;
    let (rest, _) = opt(line_ending)(rest)?;

    Ok((rest, (range1, range2)))
}

fn parse_lines(lines: &str) -> Result<Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>> {
    let Ok((rest, parsed)) = many0(parse_line)(lines) else {
        bail!("Failed to parse input");
    };
    if !rest.is_empty() {
        bail!("Input contained unexpected extra content: {:?}", rest);
    }
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
        assert_eq!(part1(INPUT).unwrap(), "2".to_string());
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        assert_eq!(part2(INPUT).unwrap(), "4".to_string());
    }
}
