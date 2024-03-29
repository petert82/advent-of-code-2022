use std::collections::HashMap;

use anyhow::Result;

pub fn part1(input: &str) -> Result<usize> {
    let (mut visibilities, mut heights) = build_grid(input, false);

    for row in heights.iter() {
        process_row_part1(row.iter(), &mut visibilities);
        process_row_part1(row.iter().rev(), &mut visibilities);
    }

    heights.reverse();
    let heights = transpose(heights);

    for row in heights.iter() {
        process_row_part1(row.iter(), &mut visibilities);
        process_row_part1(row.iter().rev(), &mut visibilities);
    }

    let visible_count = visibilities.values().filter(|v| *v == &true).count();

    Ok(visible_count)
}

pub fn part2(input: &str) -> Result<usize> {
    let (mut visibilities, mut heights) = build_grid(input, 1 as usize);

    for row in heights.iter() {
        process_row_part2(row.iter(), &mut visibilities);
        process_row_part2(row.iter().rev(), &mut visibilities);
    }

    heights.reverse();
    let heights = transpose(heights);

    for row in heights.iter() {
        process_row_part2(row.iter(), &mut visibilities);
        process_row_part2(row.iter().rev(), &mut visibilities);
    }

    let max_scenic_score = *visibilities
        .values()
        .max()
        .expect("there should be at least one scenic score");

    Ok(max_scenic_score)
}

fn build_grid<T>(
    input: &str,
    default_visibility: T,
) -> (
    HashMap<(usize, usize), T>,
    Vec<Vec<((usize, usize), usize)>>,
)
where
    T: Copy,
{
    let mut visibilities: HashMap<(usize, usize), _> = HashMap::new();
    let heights = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .map(|c| c.to_digit(10))
                .flatten()
                .enumerate()
                .map(|(x, height)| {
                    visibilities.insert((x, y), default_visibility);
                    ((x, y), height as usize)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (visibilities, heights)
}

fn process_row_part1<'a, I>(row: I, visibilities: &mut HashMap<(usize, usize), bool>)
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

fn process_row_part2<'a, I>(row: I, visibilities: &mut HashMap<(usize, usize), usize>)
where
    I: Iterator<Item = &'a ((usize, usize), usize)>,
{
    let mut prev_heights: Vec<usize> = Vec::new();

    for (i, ((x, y), height)) in row.enumerate() {
        if prev_heights.is_empty() {
            visibilities.entry((*x, *y)).and_modify(|vis| *vis *= 0);
        } else {
            // Find the index of the last tree we saw that was this tall or taller
            let prev_idx = prev_heights
                .iter()
                .enumerate()
                .filter(|(_, &v)| v >= *height)
                .map(|(j, _)| j)
                .last();
            // Calculate the distance to that last tree
            let run_length = match prev_idx {
                None => prev_heights.len(),
                Some(j) => i - j,
            };
            visibilities
                .entry((*x, *y))
                .and_modify(|vis| *vis *= run_length);
        }
        prev_heights.push(*height);
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

    #[test]
    fn test_part2_gives_correct_answer() {
        let visible_trees = part2(INPUT).unwrap();
        assert_eq!(visible_trees, 8);
    }
}
