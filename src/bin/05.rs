use std::ops::RangeInclusive;

use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh, ingredient) = input.split_once("\n\n")?;

    let fresh = fresh
        .par_lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(min, max)| min.parse::<u64>().unwrap()..=max.parse().unwrap())
        .collect::<Box<_>>();

    Some(
        ingredient
            .par_lines()
            .map(|l| l.parse().unwrap())
            .filter(|i| fresh.iter().any(|r| r.contains(i)))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let fresh = input
        .split_once("\n\n")?
        .0
        .par_lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(min, max)| min.parse::<u64>().unwrap()..=max.parse().unwrap())
        .collect::<Box<_>>();

    let fresh = fresh
        .into_iter()
        .fold(Vec::<RangeInclusive<u64>>::new(), |mut ranges, r| {
            let mut merged = r;
            ranges.retain(|existing| {
                if merged.end() >= existing.start() && merged.start() <= existing.end() {
                    merged =
                        *merged.start().min(existing.start())..=*merged.end().max(existing.end());
                    false
                } else {
                    true
                }
            });
            ranges.push(merged);
            ranges
        });

    Some(fresh.iter().map(|r| r.end() - r.start() + 1).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
