advent_of_code::solution!(5);

fn parse_steps(input: &str) -> Vec<i64> {
    input
        .split("\n")
        .map(|element| element.parse::<i64>().unwrap())
        .collect()
}

fn jumps(input: &str, decrease: bool) -> u64 {
    let mut location: i64 = 0;
    let mut number_steps = 0;
    let mut offsets = parse_steps(input);
    while 0 <= location && location < offsets.len() as i64 {
        let previous_location = location as usize;
        location += offsets[location as usize];
        if decrease && offsets[previous_location] >= 3 {
            offsets[previous_location] -= 1;
        } else {
            offsets[previous_location] += 1;
        }
        number_steps += 1;
    }
    number_steps
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(jumps(input, false))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(jumps(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
