use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let highest_calories = input
        .split("\n\n")
        .map(|lines| {
            let elf_calories: u32 = lines.lines().map(|line| line.parse::<u32>().unwrap()).sum();
            elf_calories
        })
        .max()
        .unwrap();

    Ok(format!("{}", highest_calories))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_gives_correct_answer() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let res = part1(input).unwrap();
        assert_eq!(res, "24000".to_string());
    }
}
