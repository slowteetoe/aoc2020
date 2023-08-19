pub fn part_one(input: &str) -> Option<u32> {
    let mut values = input
        .lines()
        .map(|line| str::parse::<u32>(line).unwrap())
        .collect::<Vec<u32>>();
    values.push(values.iter().max().unwrap() + 3);
    values.sort();

    let mut counts = (0, 0, 0);
    let mut prev = &0u32;
    values.iter().for_each(|n| {
        match n - prev {
            3 => counts.2 += 1,
            2 => counts.1 += 1,
            1 => counts.0 += 1,
            _ => unreachable!("should only be a diff of 1-3"),
        }
        prev = n;
    });
    Some(counts.0 * counts.2)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(220));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
