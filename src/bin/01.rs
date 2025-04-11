advent_of_code::solution!(1);

pub fn capcha(input: &str, skip: usize) -> Option<u64> {
    let as_vector = input
        .to_string()
        .chars()
        .filter_map(|digit| digit.to_digit(10))
        .map(|digit| digit as u64)
        .collect::<Vec<u64>>();

    Some(
        as_vector
            .iter()
            .enumerate()
            .filter_map(|(i, &val)| {
                if val == as_vector[(i + skip) % as_vector.len()] {
                    Some(val)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    capcha(input, 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    capcha(input, input.len() / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1122"), Some(3));
        assert_eq!(part_one("1111"), Some(4));
        assert_eq!(part_one("1234"), Some(0));
        assert_eq!(part_one("91212129"), Some(9));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("1212"), Some(6));
        assert_eq!(part_two("1221"), Some(0));
        assert_eq!(part_two("123425"), Some(4));
        assert_eq!(part_two("123123"), Some(12));
        assert_eq!(part_two("12131415"), Some(4));
    }
}
