advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map = input
        .split_whitespace()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '@' => Tile::Roll,
                    '.' => Tile::Empty,
                    _ => unreachable!(),
                })
                .collect::<Box<_>>()
        })
        .collect::<Box<_>>();

    let mut result = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == Tile::Empty {
                continue;
            }

            let mut nearby = 0;
            for (other_row, other_col) in [
                (row.checked_sub(1), col.checked_sub(1)),
                (row.checked_sub(1), Some(col)),
                (row.checked_sub(1), Some(col + 1)),
                (Some(row), col.checked_sub(1)),
                (Some(row), Some(col + 1)),
                (Some(row + 1), col.checked_sub(1)),
                (Some(row + 1), Some(col)),
                (Some(row + 1), Some(col + 1)),
            ] {
                let Some(other_row) = other_row else {
                    continue;
                };
                let Some(other_col) = other_col else {
                    continue;
                };
                if other_row >= map.len() || other_col >= map[other_row].len() {
                    continue;
                }

                if map[other_row][other_col] == Tile::Roll {
                    nearby += 1;
                }
            }

            if nearby < 4 {
                result += 1;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = input
        .split_whitespace()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '@' => Tile::Roll,
                    '.' => Tile::Empty,
                    _ => unreachable!(),
                })
                .collect::<Box<_>>()
        })
        .collect::<Box<_>>();

    let mut result = 0;
    let mut to_remove = Vec::new();
    loop {
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                if map[row][col] == Tile::Empty {
                    continue;
                }

                let mut nearby = 0;
                for (other_row, other_col) in [
                    (row.checked_sub(1), col.checked_sub(1)),
                    (row.checked_sub(1), Some(col)),
                    (row.checked_sub(1), Some(col + 1)),
                    (Some(row), col.checked_sub(1)),
                    (Some(row), Some(col + 1)),
                    (Some(row + 1), col.checked_sub(1)),
                    (Some(row + 1), Some(col)),
                    (Some(row + 1), Some(col + 1)),
                ] {
                    let Some(other_row) = other_row else {
                        continue;
                    };
                    let Some(other_col) = other_col else {
                        continue;
                    };
                    if other_row >= map.len() || other_col >= map[other_row].len() {
                        continue;
                    }

                    if map[other_row][other_col] == Tile::Roll {
                        nearby += 1;
                    }
                }

                if nearby < 4 {
                    to_remove.push((row, col));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        } else {
            for (row, col) in to_remove.drain(..) {
                map[row][col] = Tile::Empty;
                result += 1;
            }
        }
    }

    Some(result)
}

#[derive(PartialEq)]
enum Tile {
    Roll,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
