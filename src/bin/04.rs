use itertools::Itertools;
use std::collections::HashSet;
advent_of_code::solution!(4);

fn is_valid(input: Vec<&str>, count_anagrams: bool) -> bool {
    let vector_length = input.len();
    let mut words_map = HashSet::new();
    for word in input {
        if count_anagrams {
            let to_add = word.chars().sorted().collect::<String>();
            words_map.insert(to_add);
        } else {
            words_map.insert(word.to_string());
        }
    }
    vector_length == words_map.len()
}

fn is_valid_batch(input: &str, count_anagrams: bool) -> u64 {
    input
        .split("\n")
        .filter(|line| is_valid(line.split_whitespace().collect(), count_anagrams))
        .count() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(is_valid_batch(input, false))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(is_valid_batch(input, true))
}
