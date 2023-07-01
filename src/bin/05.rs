use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| {
                let (fb, lr) = s
                    .chars()
                    .into_iter()
                    .fold((vec![], vec![]), |mut acc, c| match c {
                        'F' | 'B' => {
                            acc.0.push(c);
                            acc
                        }
                        'L' | 'R' => {
                            acc.1.push(c);
                            acc
                        }
                        _ => unreachable!(),
                    });
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
    let mut seats : Vec<u32> = input.lines().map(|s| {
        let (fb, lr) = s
            .chars()
            .into_iter()
            .fold((vec![], vec![]), |mut acc, c| match c {
                'F' | 'B' => {
                    acc.0.push(c);
                    acc
                }
                'L' | 'R' => {
                    acc.1.push(c);
                    acc
                }
                _ => unreachable!(),
            });
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
    }).collect();
    seats.sort();
    for (a, b) in seats.iter().tuple_windows() {
        if b.abs_diff(*a) == 2 {    // need a space between a and b so 2 not 1!
            dbg!(&a,&b);
            return Some(a+1)
        }
    }
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
