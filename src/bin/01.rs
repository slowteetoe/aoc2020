use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let nums: Vec<i32> = input
        .split_ascii_whitespace()
        .into_iter()
        .map(|n| str::parse::<i32>(n).unwrap())
        .sorted()
        .collect();
    for n in 0..nums.len() - 1 {
        let other = 2020 - nums[n];
        for idx in n..=nums.len() - 1 {
            if nums[idx] == other {
                return Some(nums[n] as u32 * other as u32);
            } else if nums[idx] > other {
                break;
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums: Vec<i32> = input
        .split_ascii_whitespace()
        .into_iter()
        .map(|n| str::parse::<i32>(n).unwrap())
        .sorted()
        .collect();
    // guess we can do the same as part_one, but with an extra loop.  since we know there's an answer, we'll be a little sloppy
    for n1 in 0..nums.len()-1 {
        let val1 = nums[n1];
        for n2 in 1..nums.len()-1 {
            let val2 = nums[n2];
            let target = 2020 - (val1+val2);
            for n3 in 2..nums.len()-1 {
                if nums[n3] == target {
                    // dbg!(val1 , val2, target);
                    return Some((val1 * val2 * target) as u32)    // watch out for overflow?
                } else if nums[n3] > target  {
                    break;
                }
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(514579));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
