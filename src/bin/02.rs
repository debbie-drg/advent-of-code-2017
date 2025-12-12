advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .split("\n")
        .map(|row| {
            row.split_whitespace()
                .filter_map(|element| element.parse::<u64>().ok())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .into_iter()
            .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .into_iter()
            .map(|row| {
                for element_1 in row.iter() {
                    for element_2 in row.iter() {
                        if element_1 != element_2 && element_1 % element_2 == 0 {
                            return element_1 / element_2;
                        }
                    }
                }
                0
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let part_one_input = "5 1 9 5
7 5 3
2 4 6 8";
        assert_eq!(part_one(part_one_input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let part_two_input = "5 9 2 8
9 4 7 3
3 8 6 5";
        assert_eq!(part_two(part_two_input), Some(9));
    }
}
