use std::{collections::HashMap, fmt::Debug};

use anyhow::{bail, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending, not_line_ending},
    combinator::{all_consuming, map, map_res, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

pub fn part1(input: &str) -> Result<usize> {
    let commands = parse_command_list(input)?;
    let state = State::from_commands(commands);

    let res = state.sizes.values().filter(|&size| *size <= 100000).sum();

    Ok(res)
}

pub fn part2(input: &str) -> Result<usize> {
    const TOTAL_SPACE: usize = 70000000;
    const NEED_FREE: usize = 30000000;

    let commands = parse_command_list(input)?;
    let state = State::from_commands(commands);

    let free = TOTAL_SPACE - state.used_space();
    let need = NEED_FREE - free;
    let mut candidates = state
        .sizes
        .values()
        .filter(|&size| *size >= need)
        .collect::<Vec<_>>();
    assert!(!candidates.is_empty());
    candidates.sort();

    Ok(*candidates[0])
}

#[derive(Debug)]
struct State {
    working_dir: WorkingDir,
    sizes: HashMap<String, usize>,
}

impl State {
    fn from_commands(cmds: Vec<Command>) -> Self {
        let mut state = Self {
            working_dir: WorkingDir::new(),
            sizes: HashMap::new(),
        };
        state.apply_all(cmds);
        state
    }

    fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::CdUp => self.working_dir.cd_up(),
            Command::CdInto(dir) => self.working_dir.cd_into(dir.as_ref()),
            Command::Ls(DirListing(entries)) => {
                let dir_size = entries
                    .iter()
                    .map(|entry| match entry {
                        DirEntry::File(_, size) => *size,
                        DirEntry::Dir(_) => 0,
                    })
                    .sum::<usize>();
                for path in self.working_dir.tree_paths() {
                    let size = self.sizes.entry(path).or_insert(0);
                    *size += dir_size;
                }
            }
        }
    }

    fn apply_all(&mut self, cmds: Vec<Command>) {
        for cmd in cmds {
            self.apply(cmd);
        }
    }

    fn used_space(&self) -> usize {
        *self.sizes.get("/".into()).unwrap_or(&0)
    }
}

struct WorkingDir(Vec<String>);

impl WorkingDir {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn cd_into(&mut self, dir: &str) {
        self.0.push(dir.to_owned());
    }

    fn cd_up(&mut self) {
        self.0.pop();
    }

    /// Gets current path, plus all of its "parent" paths
    fn tree_paths(&self) -> Vec<String> {
        let mut paths = Vec::with_capacity(self.0.len() + 1);
        paths.push("/".into());

        if self.0.is_empty() {
            return paths;
        }
        for i in 1..=self.0.len() {
            paths.push(self.0[0..i].join("/"));
        }
        paths
    }
}

impl Debug for WorkingDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "/");
        }
        let dirs = self.0.join("/");
        write!(f, "/{}", dirs)
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    CdUp,
    CdInto(String),
    Ls(DirListing),
}

#[derive(Debug, PartialEq)]
enum DirEntry {
    File(String, usize),
    Dir(String),
}

#[derive(Debug, PartialEq)]
struct DirListing(Vec<DirEntry>);

fn command(i: &str) -> IResult<&str, Command> {
    // "$ cd .."
    let cd_up = map(tag("$ cd .."), |_| Command::CdUp);
    // "$ cd fmfnpm"
    let cd_into = map(preceded(tag("$ cd "), alpha1), |name: &str| {
        Command::CdInto(name.into())
    });
    // "$ ls"
    let ls = map(
        preceded(tuple((tag("$ ls"), line_ending)), dir_listing),
        Command::Ls,
    );
    alt((cd_up, cd_into, ls))(i)
}

fn parse_command_list(i: &str) -> Result<Vec<Command>> {
    // Require that the list of commands always starts by changing into the root directory
    let commands = preceded(
        tuple((tag("$ cd /"), line_ending)),
        separated_list1(line_ending, command),
    );
    let Ok((_rest, commands)) = all_consuming(terminated(commands, opt(line_ending)))(i) else {
        bail!("Failed to parse input commands");
    };

    Ok(commands)
}

fn number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse::<usize>())(i)
}

fn dir_entry(i: &str) -> IResult<&str, DirEntry> {
    // "13445 b.txt"
    let file = map(
        separated_pair(number, tag(" "), not_line_ending),
        |(size, name)| DirEntry::File(name.to_owned(), size),
    );
    // "dir dassfsdf"
    let dir = map(preceded(tag("dir "), alpha1), |name: &str| {
        DirEntry::Dir(name.to_owned())
    });
    alt((file, dir))(i)
}

fn dir_listing(i: &str) -> IResult<&str, DirListing> {
    map(separated_list1(line_ending, dir_entry), |entries| {
        DirListing(entries)
    })(i)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_can_parse_dir_listing() {
        let input = "14848514 fwdvgnqp.txt\n8504156 c.dat\ndir fmfnpm";
        let expect = DirListing(vec![
            DirEntry::File("fwdvgnqp.txt".into(), 14848514),
            DirEntry::File("c.dat".into(), 8504156),
            DirEntry::Dir("fmfnpm".into()),
        ]);
        let (rest, listing) = dir_listing(input).unwrap();
        assert!(rest.is_empty());
        assert_eq!(listing, expect);
    }

    #[test]
    fn test_can_parse_input() {
        let expect = vec![
            Command::Ls(DirListing(vec![
                DirEntry::Dir("a".into()),
                DirEntry::File("b.txt".into(), 14848514),
                DirEntry::File("c.dat".into(), 8504156),
                DirEntry::Dir("d".into()),
            ])),
            Command::CdInto("a".into()),
            Command::Ls(DirListing(vec![
                DirEntry::Dir("e".into()),
                DirEntry::File("f".into(), 29116),
                DirEntry::File("g".into(), 2557),
                DirEntry::File("h.lst".into(), 62596),
            ])),
            Command::CdInto("e".into()),
            Command::Ls(DirListing(vec![DirEntry::File("i".into(), 584)])),
            Command::CdUp,
            Command::CdUp,
            Command::CdInto("d".into()),
            Command::Ls(DirListing(vec![
                DirEntry::File("j".into(), 4060174),
                DirEntry::File("d.log".into(), 8033020),
                DirEntry::File("d.ext".into(), 5626152),
                DirEntry::File("k".into(), 7214296),
            ])),
        ];
        let commands = parse_command_list(INPUT).unwrap();
        assert_eq!(commands, expect);
    }

    #[test]
    fn test_part_1_gives_correct_answer() {
        assert_eq!(part1(INPUT).unwrap(), 95437);
    }

    #[test]
    fn test_part_2_gives_correct_answer() {
        assert_eq!(part2(INPUT).unwrap(), 24933642);
    }
}
