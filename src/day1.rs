use anyhow::Result;

pub fn part1(input: &str) -> Result<usize> {
    let elf_calories = elf_calories(input);
    let highest_calories = elf_calories.iter().max().unwrap();

    Ok(*highest_calories)
}

pub fn part2(input: &str) -> Result<usize> {
    let mut elf_calories = elf_calories(input);
    elf_calories.sort();
    elf_calories.reverse();
    let top3_elves_sum = elf_calories.iter().take(3).sum::<usize>();

    Ok(top3_elves_sum)
}

fn elf_calories(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|lines| {
            let elf_calories = lines
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum();
            elf_calories
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1_gives_correct_answer() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 24000);
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 45000);
    }
}
