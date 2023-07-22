use std::collections::BTreeSet;

pub fn part_one(input: &str) -> Option<u32> {
    let newlines = input.chars().filter(|c| *c=='\n').count();
    let preamble_len = if newlines < 20 { 5 } else { 25 };

    // spiffy, but I didn't read the instructions closely enough.
    let (preamble, values) = input
        .lines()
        .map(|val| str::parse::<u16>(val).unwrap())
        .enumerate()
        .fold((BTreeSet::new(), vec![]), |(mut preamble, mut values), (idx, val)|  {
            if idx < preamble_len {
                preamble.insert(val); {}
            } else {
                values.push(val)
            };
            (preamble, values)
        });

    // naive approach with horrible control logic lol
    let mut sorted = Vec::from_iter(preamble.clone());
    sorted.sort();

    
    for target in values.iter() {
        let mut found = None;
        for a in preamble.iter() {
            let complement : i32 = *target as i32 - *a as i32;
            dbg!(&target, &a, &complement, &preamble);
            if complement > 0 && preamble.contains(&(complement as u16)) {
                dbg!(&a, &complement, &target);
                found = Some(target);
                break
            }
        }
        if found.is_none() {
            dbg!(&found);
            return Some(*target as u32);
        }
    };
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
