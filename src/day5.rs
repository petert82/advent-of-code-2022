use anyhow::{bail, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map_res, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

pub fn part1(input: &str) -> Result<String> {
    let state = GameState::try_from(input)?;
    let mut stacks = state.execute_part1();
    let mut res = String::with_capacity(stacks.len());
    for stack in stacks.iter_mut() {
        res.push(stack.pop().unwrap());
    }
    Ok(res)
}

struct GameState {
    stacks: Stacks,
    moves: Vec<Move>,
}

type Stacks = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

impl GameState {
    fn execute_part1(mut self) -> Stacks {
        for Move { num, from, to } in self.moves.iter() {
            for _ in 0..*num {
                let val = self.stacks[from - 1].pop().unwrap();
                self.stacks[to - 1].push(val);
            }
        }
        self.stacks
    }
}

impl TryFrom<&str> for GameState {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((input_stacks, input_moves)) = value.split_once("\n\n") else {
        bail!("expected input to contain a blank line between stacks and moves")
        };
        Ok(GameState {
            stacks: parse_stacks(input_stacks)?,
            moves: parse_moves(input_moves)?,
        })
    }
}

fn parse_stacks(input: &str) -> Result<Stacks> {
    let num_stacks = (input
        .lines()
        .next()
        .expect("expect at least one line in stack config")
        .len()
        + 1)
        / 4;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                let stack_idx = ((i + 3) / 4) - 1;
                stacks[stack_idx].push(c);
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    Ok(stacks)
}

fn number(digits: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse::<usize>())(digits)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (rest, (_, num, _, from, _, to)) = tuple((
        tag("move "),
        number,
        tag(" from "),
        number,
        tag(" to "),
        number,
    ))(input)?;
    let (rest, _) = opt(line_ending)(rest)?;

    Ok((rest, Move { num, from, to }))
}

fn parse_moves(input: &str) -> Result<Vec<Move>> {
    let Ok((rest, moves)) = many0(parse_move)(input) else {
        bail!("Failed to parse moves");
    };
    if !rest.is_empty() {
        bail!("Input contained unexpected extra content: {:?}", rest);
    }
    Ok(moves)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_can_parse_initial_stack_config() {
        let (stacks_input, _) = INPUT.split_once("\n\n").unwrap();
        let stacks = parse_stacks(stacks_input).unwrap();
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0], vec!['Z', 'N']);
        assert_eq!(stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(stacks[2], vec!['P']);
    }

    #[test]
    fn test_can_parse_moves() {
        let (_, moves_input) = INPUT.split_once("\n\n").unwrap();
        let expect = vec![
            Move {
                num: 1,
                from: 2,
                to: 1,
            },
            Move {
                num: 3,
                from: 1,
                to: 3,
            },
            Move {
                num: 2,
                from: 2,
                to: 1,
            },
            Move {
                num: 1,
                from: 1,
                to: 2,
            },
        ];
        assert_eq!(parse_moves(moves_input).unwrap(), expect);
    }

    #[test]
    fn test_part1_gives_correct_answer() {
        assert_eq!(part1(INPUT).unwrap(), "CMZ".to_string());
    }
}
