use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let elf_calories = elf_calories(input);
    let highest_calories = elf_calories.iter().max().unwrap();

    Ok(format!("{}", highest_calories))
}

pub fn part2(input: &str) -> Result<String> {
    let mut elf_calories = elf_calories(input);
    elf_calories.sort();
    elf_calories.reverse();
    let top3_elves_sum = elf_calories.iter().take(3).sum::<u32>();

    Ok(format!("{}", top3_elves_sum))
}

fn elf_calories(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|lines| {
            let elf_calories: u32 = lines.lines().map(|line| line.parse::<u32>().unwrap()).sum();
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
        assert_eq!(res, "24000".to_string());
    }

    #[test]
    fn test_part2_gives_correct_answer() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, "45000".to_string());
    }
}
