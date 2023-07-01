pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| {
                let fb: Vec<char> = s
                    .chars()
                    .take_while(|c| c == &'F' || c == &'B')
                    .by_ref()
                    .collect();
                let lr: Vec<char> = s.chars().skip_while(|c| c != &'L' && c != &'R').collect();

                let row = fb.iter().fold(0..=127, |acc, c| {
                    let mid = (acc.start() + acc.end()) / 2;
                    match c {
                        'F' => *acc.start()..=mid,
                        'B' => mid + 1..=*acc.end(),
                        _ => unreachable!(),
                    }
                });

                let col = lr.iter().fold(0..=7, |acc, c| {
                    let mid = (acc.start() + acc.end()) / 2;
                    match c {
                        'L' => *acc.start()..=mid,
                        'R' => mid + 1..=*acc.end(),
                        _ => unreachable!(),
                    }
                });
                (*row.start() * 8) + col.start()
            })
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(820));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_example1() {
        let input = "FBFBBFFRLR";
        assert_eq!(part_one(&input), Some(357));
    }

    #[test]
    fn test_example2() {
        let input = "BFFFBBFRRR";
        assert_eq!(part_one(&input), Some(567));
    }

    #[test]
    fn test_example3() {
        let input = "FFFBBBFRRR";
        assert_eq!(part_one(&input), Some(119));
    }

    #[test]
    fn test_example4() {
        let input = "BBFFBBFRLL";
        assert_eq!(part_one(&input), Some(820));
    }
}
