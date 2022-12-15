use std::collections::HashMap;

use anyhow::Result;

pub fn part1(input: &str) -> Result<usize> {
    let mut visibilities: HashMap<(usize, usize), bool> = HashMap::new();
    let mut heights = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .map(|c| c.to_digit(10))
                .flatten()
                .enumerate()
                .map(|(x, height)| {
                    visibilities.insert((x, y), false);
                    ((x, y), height as usize)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for row in heights.iter() {
        process_row(row.iter(), &mut visibilities);
        process_row(row.iter().rev(), &mut visibilities);
    }

    heights.reverse();
    let heights = transpose(heights);

    for row in heights.iter() {
        process_row(row.iter(), &mut visibilities);
        process_row(row.iter().rev(), &mut visibilities);
    }

    let visible_count = visibilities.values().filter(|v| *v == &true).count();

    Ok(visible_count)
}

fn process_row<'a, I>(row: I, visibilities: &mut HashMap<(usize, usize), bool>)
where
    I: Iterator<Item = &'a ((usize, usize), usize)>,
{
    let mut tallest = None;

    for ((x, y), height) in row {
        if tallest.is_none() {
            visibilities.insert((*x, *y), true);
        }
        if let Some(tallest) = tallest {
            if *height > tallest {
                visibilities.insert((*x, *y), true);
            }
        }

        tallest = tallest.map_or(Some(*height), |t| {
            if t < *height {
                Some(*height)
            } else {
                Some(t)
            }
        });
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1_gives_correct_answer() {
        let visible_trees = part1(INPUT).unwrap();
        assert_eq!(visible_trees, 21);
    }
}
