use anyhow::{bail, Result};
use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::{map_res, opt};
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::IResult;

pub fn part1(input: &str) -> Result<String> {
    if let Ok((_, plays)) = parse_part1_plays(input) {
        let total_score = plays.iter().map(|play| play.score()).sum::<u32>();
        Ok(format!("{}", total_score))
    } else {
        bail!("could not parse input")
    }
}

pub fn part2(input: &str) -> Result<String> {
    if let Ok((_, plays)) = parse_part2_plays(input) {
        let total_score = plays.iter().map(|play| play.score()).sum::<u32>();
        Ok(format!("{}", total_score))
    } else {
        bail!("could not parse input")
    }
}

#[derive(Debug, PartialEq)]
struct Play {
    opponent_shape: Shape,
    response: Shape,
}

impl Play {
    fn score(&self) -> u32 {
        self.outcome_score() + self.response.score()
    }

    /// Score for just whether this was a win/lose/draw
    fn outcome_score(&self) -> u32 {
        // Draw
        if self.opponent_shape == self.response {
            return 3;
        }
        match (self.opponent_shape, self.response) {
            // Win
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Paper, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Rock) => 6,
            // Loss
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = String;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err(format!("cannot convert {} to a Rock/Paper/Scissors", input)),
        }
    }
}

enum DesiredResult {
    Lose,
    Draw,
    Win,
}

impl DesiredResult {
    fn get_response(&self, opponent_shape: Shape) -> Shape {
        match (self, opponent_shape) {
            (Self::Lose, Shape::Rock) => Shape::Scissors,
            (Self::Lose, Shape::Paper) => Shape::Rock,
            (Self::Lose, Shape::Scissors) => Shape::Paper,
            (Self::Draw, Shape::Rock) => Shape::Rock,
            (Self::Draw, Shape::Paper) => Shape::Paper,
            (Self::Draw, Shape::Scissors) => Shape::Scissors,
            (Self::Win, Shape::Rock) => Shape::Paper,
            (Self::Win, Shape::Paper) => Shape::Scissors,
            (Self::Win, Shape::Scissors) => Shape::Rock,
        }
    }
}

impl TryFrom<char> for DesiredResult {
    type Error = String;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            'X' => Ok(DesiredResult::Lose),
            'Y' => Ok(DesiredResult::Draw),
            'Z' => Ok(DesiredResult::Win),
            _ => Err(format!("cannot convert {} to a Win/Lose/Draw", input)),
        }
    }
}

fn opponent_shape(input: &str) -> IResult<&str, Shape> {
    map_res(one_of("ABC"), Shape::try_from)(input)
}

fn player_shape(input: &str) -> IResult<&str, Shape> {
    map_res(one_of("XYZ"), Shape::try_from)(input)
}

fn desired_result(input: &str) -> IResult<&str, DesiredResult> {
    map_res(one_of("XYZ"), DesiredResult::try_from)(input)
}

fn parse_part1_play(input: &str) -> IResult<&str, Play> {
    let (input, (opponent_shape, response)) =
        separated_pair(opponent_shape, space1, player_shape)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((
        input,
        Play {
            opponent_shape,
            response,
        },
    ))
}

fn parse_part1_plays(input: &str) -> IResult<&str, Vec<Play>> {
    many0(parse_part1_play)(input)
}

fn parse_part2_play(input: &str) -> IResult<&str, Play> {
    let (input, (opponent_shape, desired_result)) =
        separated_pair(opponent_shape, space1, desired_result)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((
        input,
        Play {
            opponent_shape,
            response: desired_result.get_response(opponent_shape),
        },
    ))
}

fn parse_part2_plays(input: &str) -> IResult<&str, Vec<Play>> {
    many0(parse_part2_play)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_can_parse_a_single_line_for_part1() {
        let input = "A Y\n";
        assert_eq!(
            parse_part1_play(input),
            Ok((
                "",
                Play {
                    opponent_shape: Shape::Rock,
                    response: Shape::Paper,
                }
            ))
        );
    }

    #[test]
    fn can_parse_multiple_lines_of_input_for_part1() {
        assert_eq!(
            parse_part1_plays(INPUT),
            Ok((
                "",
                vec![
                    Play {
                        opponent_shape: Shape::Rock,
                        response: Shape::Paper,
                    },
                    Play {
                        opponent_shape: Shape::Paper,
                        response: Shape::Rock,
                    },
                    Play {
                        opponent_shape: Shape::Scissors,
                        response: Shape::Scissors,
                    },
                ]
            ))
        );
    }

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, "15".to_string());
    }

    #[test]
    fn test_can_parse_a_single_line_for_part2() {
        let input = "A Y\n";
        assert_eq!(
            parse_part2_play(input),
            Ok((
                "",
                Play {
                    opponent_shape: Shape::Rock,
                    response: Shape::Rock,
                }
            ))
        );
    }

    #[test]
    fn can_parse_multiple_lines_of_input_for_part2() {
        assert_eq!(
            parse_part2_plays(INPUT),
            Ok((
                "",
                vec![
                    Play {
                        opponent_shape: Shape::Rock,
                        response: Shape::Rock,
                    },
                    Play {
                        opponent_shape: Shape::Paper,
                        response: Shape::Rock,
                    },
                    Play {
                        opponent_shape: Shape::Scissors,
                        response: Shape::Rock,
                    },
                ]
            ))
        );
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, "12".to_string());
    }
}
