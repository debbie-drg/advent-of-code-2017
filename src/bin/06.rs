advent_of_code::solution!(6);

use std::collections::HashMap;

fn argmax(input_vector: &[u64]) -> Option<usize> {
    let mut max: u64 = 0;
    let mut argmax = 0;
    for (index, element) in input_vector.iter().enumerate() {
        if *element > max {
            max = *element;
            argmax = index;
        }
    }
    Some(argmax)
}

struct MemoryBank {
    banks: Vec<u64>,
    memory: HashMap<Vec<u64>, u64>,
    number_iters: u64,
}

impl From<&str> for MemoryBank {
    fn from(input: &str) -> Self {
        let banks: Vec<u64> = input
            .split_whitespace()
            .map(|element| element.parse::<u64>().unwrap())
            .collect();
        let mut memory = HashMap::new();
        memory.insert(banks.clone(), 0);
        MemoryBank {
            banks,
            memory,
            number_iters: 0,
        }
    }
}

impl MemoryBank {
    fn get_next_bank(&mut self) -> Option<Vec<u64>> {
        let max_index = argmax(&self.banks);
        match max_index {
            Some(max_index) => {
                let to_redistribute = self.banks[max_index];
                self.banks[max_index] = 0;
                let length = self.banks.len() as u64;
                let to_add = to_redistribute / length;
                let reminder = to_redistribute % length;
                Some(
                    self.banks
                        .iter()
                        .enumerate()
                        .map(|(index, element)| {
                            let offset =
                                (index as i64 - max_index as i64).rem_euclid(length as i64);
                            element
                                + to_add
                                + ((0 < offset && offset <= ((reminder) as i64)) as u64)
                        })
                        .collect(),
                )
            }
            None => None,
        }
    }

    fn find_loop(&mut self) -> Option<(u64, u64)> {
        loop {
            self.number_iters += 1;
            let next_bank = self.get_next_bank();
            if let Some(bank) = next_bank {
                if self.memory.contains_key(&bank) {
                    return Some((self.number_iters, self.number_iters - self.memory[&bank]));
                }
                self.banks = bank.clone();
                self.memory.insert(bank, self.number_iters);
            } else {
                return None;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut memory_bank = MemoryBank::from(input);
    if let Some(result) = memory_bank.find_loop() {
        return Some(result.0);
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut memory_bank = MemoryBank::from(input);
    if let Some(result) = memory_bank.find_loop() {
        return Some(result.1);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("2 4 1 2"), Some(5));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2 4 1 2"), Some(4))
    }
}
