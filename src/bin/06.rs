use std::collections::{HashSet, HashMap};

pub fn part_one(input: &str) -> Option<u32> {
    let groups: Vec<&str> = input.trim().split_terminator("\n\n").collect();
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
    let groups: Vec<&str> = input.trim().split_terminator("\n\n").collect();
    let mut total = 0u32; 
    for group in groups {
        let people = group.split("\n");
        let mut votes = HashMap::<char, u32>::new();
        let mut num_peeps = 0;
        for person in people {
            num_peeps += 1;
            for v in person.chars(){
                *votes.entry(v).or_insert(0) += 1;
            }
        }
        for (_,v) in votes {
            if v == num_peeps { 
                total += 1;
            }
        }
    }
    Some(total)
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
        assert_eq!(part_two(&input), Some(6));
    }
}
