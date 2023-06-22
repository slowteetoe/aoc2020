// lol, this is some of the worst code I've written in a long time
pub fn part_one(input: &str) -> Option<u32> {
    let valid: u32 = input
        .lines()
        .map(|line| {
            if let Some((rule, input)) = line.split_once(":") {
                if let Some((count, target)) = rule.split_once(" ") {
                    if let Some((min, max)) = count.split_once("-").map(|(a, b)| {
                        (
                            str::parse::<usize>(a).unwrap(),
                            str::parse::<usize>(b).unwrap(),
                        )
                    }) {
                        let count = input.chars().filter(|c| c.to_string() == target).count();
                        if count >= min && count <= max {
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();
    Some(valid)
}

pub fn part_two(input: &str) -> Option<u32> {
    let valid: u32 = input
        .lines()
        .map(|line| {
            if let Some((rule, input)) = line.split_once(":") {
                if let Some((count, target)) = rule.split_once(" ") {
                    if let Some((pos_1, pos_2)) = count.split_once("-").map(|(a, b)| {
                        (
                            str::parse::<usize>(a).unwrap() - 1,
                            str::parse::<usize>(b).unwrap() - 1,
                        )
                    }) {
                        let c = input.trim().as_bytes();
                        let target = target.trim().as_bytes()[0] as char;
                        let valid = match (c[pos_1] as char == target, c[pos_2] as char == target) {
                            (true, false) | (false, true) => 1,
                            _ => 0,
                        };
                        // dbg!(valid);
                        valid
                    } else {
                        0
                    }
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();
    Some(valid)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(1));
    }
}
