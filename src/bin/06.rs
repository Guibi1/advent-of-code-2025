advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let problems = input
        .split_whitespace()
        .filter_map(|i| i.parse::<u64>().ok())
        .collect::<Box<_>>();
    let ops = input
        .lines()
        .last()?
        .split_whitespace()
        .map(|o| match o {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => unreachable!(),
        })
        .collect::<Box<_>>();
    let numbers_per_problem = input.lines().count() - 1;
    let ops_len = ops.len();

    Some(
        ops.iter()
            .enumerate()
            .map(|(i, op)| match op {
                Operation::Add => (0..numbers_per_problem)
                    .map(|j| problems[i + ops_len * j])
                    .sum::<u64>(),
                Operation::Mul => (0..numbers_per_problem)
                    .map(|j| problems[i + ops_len * j])
                    .product::<u64>(),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let ops = input
        .lines()
        .last()?
        .chars()
        .fold(Vec::<(Operation, i32)>::new(), |mut ops, o| {
            match o {
                '+' => {
                    if let Some(op) = ops.last_mut() {
                        op.1 -= 1;
                    }
                    ops.push((Operation::Add, 1));
                }
                '*' => {
                    if let Some(op) = ops.last_mut() {
                        op.1 -= 1;
                    }
                    ops.push((Operation::Mul, 1));
                }
                ' ' => {
                    if let Some(op) = ops.last_mut() {
                        op.1 += 1;
                    }
                }
                _ => unreachable!(),
            }
            ops
        });
    let numbers_per_problem = input.lines().count() - 1;

    let mut offset = 0;
    let mut result = 0;

    for (op, width) in ops {
        let numbers = (0..width)
            .map(|i| {
                input
                    .lines()
                    .take(numbers_per_problem)
                    .map(|line| {
                        line.chars()
                            .skip((offset + i) as usize)
                            .next()
                            .unwrap_or(' ')
                    })
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
                    .unwrap()
            })
            .collect::<Box<[_]>>();

        match op {
            Operation::Add => {
                result += numbers.into_iter().sum::<u64>();
            }
            Operation::Mul => {
                result += numbers.into_iter().product::<u64>();
            }
        }

        offset += width + 1;
    }

    Some(result)
}

enum Operation {
    Add,
    Mul,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
