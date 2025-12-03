use rayon::prelude::*;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let banks = input
        .split_whitespace()
        .map(|l| {
            l.chars()
                .map(|b| b.to_digit(10).unwrap() as u8)
                .collect::<Box<_>>()
        })
        .collect::<Box<_>>();

    let result = banks
        .into_par_iter()
        .map(|bank| {
            let tens = get_max(&bank[0..bank.len() - 1]);
            let units = get_max(&bank[(tens.0 + 1)..bank.len()]);

            (tens.1 * 10 + units.1) as u64
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = input
        .split_whitespace()
        .map(|l| {
            l.chars()
                .map(|b| b.to_digit(10).unwrap() as u8)
                .collect::<Box<_>>()
        })
        .collect::<Box<_>>();

    let result = banks
        .into_par_iter()
        .map(|bank| {
            let mut result = 0u64;
            let mut start_index = 0;

            for position in 0..12 {
                let (best_index, best_digit) =
                    get_max(&bank[start_index..bank.len() - (11 - position)]);

                result = result * 10 + best_digit as u64;
                start_index = start_index + best_index + 1;
            }

            result
        })
        .sum();

    Some(result)
}

fn get_max(bank: &[u8]) -> (usize, u8) {
    let mut max = (0, bank[0]);
    for current in bank.into_iter().enumerate() {
        if *current.1 == 9 {
            return (current.0, *current.1);
        } else if *current.1 > max.1 {
            max = (current.0, *current.1);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
