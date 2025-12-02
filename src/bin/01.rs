advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let commands = input
        .split_whitespace()
        .map(|l| match &l[0..1] {
            "L" => -l[1..].parse::<i32>().unwrap(),
            "R" => l[1..].parse::<i32>().unwrap(),
            _ => panic!("Invalid input"),
        })
        .collect::<Box<_>>();

    let mut total = 50;
    let mut result = 0;

    for command in commands {
        total += command;

        if total % 100 == 0 {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let commands = input
        .split_whitespace()
        .map(|l| match &l[0..1] {
            "L" => -l[1..].parse::<i32>().unwrap(),
            "R" => l[1..].parse::<i32>().unwrap(),
            _ => panic!("Invalid input"),
        })
        .collect::<Box<_>>();

    let mut dial = 50;
    let mut result = 0;

    for command in commands {
        for _step in 0..command.abs() {
            if command.is_positive() {
                dial += 1;
            } else {
                dial -= 1;
            }

            if dial < 0 {
                dial += 100;
            } else if dial >= 100 {
                dial -= 100;
            }

            if dial == 0 {
                result += 1;
            }
        }
    }

    Some(result)
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
        assert_eq!(result, Some(6));
    }
}
