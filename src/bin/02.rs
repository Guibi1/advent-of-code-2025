use rayon::prelude::*;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input
        .split_whitespace()
        .flat_map(|l| {
            l.split(",").map(|r| {
                let (min, max) = r.split_once("-").unwrap();
                min.parse::<u64>().unwrap()..=max.parse::<u64>().unwrap()
            })
        })
        .collect::<Box<_>>();

    let count = ranges
        .into_par_iter()
        .flatten()
        .map(|id: u64| {
            let digits = (id as f64).log10().floor() as u32 + 1;
            let divisor = 10u64.pow(digits / 2);
            let left = id / divisor;
            let right = id % divisor;
            if left == right { id } else { 0 }
        })
        .sum();

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input
        .split_whitespace()
        .flat_map(|l| {
            l.split(",").map(|r| {
                let (min, max) = r.split_once("-").unwrap();
                min.parse::<u64>().unwrap()..=max.parse::<u64>().unwrap()
            })
        })
        .collect::<Box<_>>();

    let count = ranges
        .into_par_iter()
        .flatten()
        .map(|id| {
            let digits = (id as f64).log10().floor() as u32 + 1;
            let divisor = 10u64.pow(digits / 2);
            let left = id / divisor;
            let right = id % divisor;
            if left == right {
                return id;
            }

            let s = id.to_string();
            for size in 1..=(s.len().div_ceil(2) - 1) {
                if s.len() % size != 0 {
                    continue;
                }
                if (1..(s.len() / size)).all(|i| s[..size] == s[i * size..(i + 1) * size]) {
                    return id;
                }
            }
            0
        })
        .sum();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
