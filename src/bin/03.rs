use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let input_number = input.parse::<i64>();
    match input_number {
        Err(_) => None,
        Ok(address) => {
            let iteration = (((address as f64).sqrt() - 1_f64) / 2_f64).ceil() as i64;
            if iteration == 0 {
                return Some(0);
            }
            let offset = address - (2 * iteration - 1).pow(2) - 1;
            if (offset / iteration) % 2 == 0 {
                Some((2 * iteration - offset % iteration - 1) as u64)
            } else {
                Some((iteration + offset % iteration + 1) as u64)
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_number = input.parse::<u64>();
    match input_number {
        Err(_) => None,
        Ok(address) => {
            let neighbours: Vec<(i64, i64)> = vec![
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
                (-1, 1),
                (1, -1),
                (-1, -1),
                (1, 1),
            ];
            let directions: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
            let mut table_results: HashMap<(i64, i64), u64> = HashMap::new();
            let mut location: (i64, i64) = (0, 0);
            let mut inc: i64 = 1;
            table_results.insert((0, 0), 1);
            let mut number_changes: u64 = 0;
            loop {
                for direction in directions.clone() {
                    for _ in 0..inc {
                        location = (location.0 + direction.0, location.1 + direction.1);
                        let last_cell_value: u64 = neighbours
                            .iter()
                            .map(|(x_offset, y_offset)| {
                                *table_results
                                    .entry((location.0 + x_offset, location.1 + y_offset))
                                    .or_insert(0)
                            })
                            .sum();
                        if last_cell_value > address {
                            return Some(last_cell_value);
                        }
                        table_results.insert(location, last_cell_value);
                    }
                    number_changes += 1;
                    if number_changes.is_multiple_of(2) {
                        inc += 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1"), Some(0));
        assert_eq!(part_one("12"), Some(3));
        assert_eq!(part_one("23"), Some(2));
        assert_eq!(part_one("1024"), Some(31))
    }
}
