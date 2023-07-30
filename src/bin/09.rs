use std::collections::BTreeMap;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    // hack, so that I don't have to change the macro for AoC
    let newlines = input.chars().filter(|c| *c=='\n').count();
    let preamble_len = if newlines < 20 { 5 } else { 25 };

    let values = input
        .lines()
        .map(|val| str::parse::<u64>(val).unwrap()).collect_vec();

    // if we have dups in the preamble_len previous values, this won't work and we'll have to do a map w/ count
    // really, we want some sort of circular queue

    for (idx,target) in values.iter().enumerate().skip(preamble_len){
        // set up our lookback as a hash to make faster(?)
        let mut lookback: BTreeMap<&u64, u8> = BTreeMap::new();
        for i in idx-preamble_len..=idx {
            lookback.entry(&values[i]).and_modify(|v| {*v+=1} ).or_insert(1);
        }
        let val = values[idx];
        let mut found = false;
        for i in idx-preamble_len..=idx {
            if values[i] > val {
                // impossible to match as we only have positive numbers
                continue;
            }
            let target = val - values[i];
            if lookback.contains_key(&target) {
                // double check that we don't search for 24 and find 12 once, but count it as 2x
                // if target == values[i] && lookback[&target] != 2 {
                //     dbg!(target, lookback, values[i]);
                //     panic!("dups?");
                //     // continue;
                // }
                found = true;
                break;
            }
        }
        if found {
            continue;
        }

        return Some(*target as u32);
    }
    
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(127));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
