use anyhow::{bail, Result};
use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::{map_res, opt};
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::IResult;

pub fn part1(input: &str) -> Result<String> {
    if let Ok((_, plays)) = parse_plays(input) {
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

fn shape_from_char(input: char) -> Result<Shape, String> {
    match input {
        'A' | 'X' => Ok(Shape::Rock),
        'B' | 'Y' => Ok(Shape::Paper),
        'C' | 'Z' => Ok(Shape::Scissors),
        _ => Err(format!("cannot convert {} to a Rock/Paper/Scissors", input)),
    }
}

fn opponent_shape(input: &str) -> IResult<&str, Shape> {
    map_res(one_of("ABC"), shape_from_char)(input)
}

fn player_shape(input: &str) -> IResult<&str, Shape> {
    map_res(one_of("XYZ"), shape_from_char)(input)
}

fn parse_play(input: &str) -> IResult<&str, Play> {
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

fn parse_plays(input: &str) -> IResult<&str, Vec<Play>> {
    many0(parse_play)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_can_parse_a_single_line() {
        let input = "A Y\n";
        assert_eq!(
            parse_play(input),
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
    fn can_parse_multiple_lines_of_input() {
        assert_eq!(
            parse_plays(INPUT),
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
}
