use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let groups: Vec<&str> = input.split_terminator("\n\n").collect();
    let mut total = 0u32; 
    for group in groups {
        let people = group.split("\n");
        let mut votes = HashSet::<char>::new();
        for person in people {
            for v in person.chars() {
                votes.insert(v);
            }
        }
        total += votes.len() as u32
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
