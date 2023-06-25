use std::str::FromStr;

use nom::{
    character::complete::{alpha1, char, digit1},
    combinator::map_res,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct PasswordValidation {
    min: usize,
    max: usize,
    letter: char,
    input: String,
}
fn parse_numbers(input: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(input)
}
impl PasswordValidation {
    // 1-3 a: abcde
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (min, max)) =
            separated_pair(parse_numbers, char('-'), parse_numbers)(input.trim())?;
        let (input, (letter, target)) =
            separated_pair(alpha1, char(':'), preceded(char(' '), alpha1))(input.trim())?;
        Ok((
            input,
            Self {
                min: min,
                max: max,
                letter: letter.chars().next().unwrap(),
                input: target.trim().to_string(),
            },
        ))
    }

    fn is_valid_for_part_1(self) -> bool {
        let count = &self.input.chars().filter(|c| *c == self.letter).count();
        let result = (self.min..=self.max).contains(count);
        // dbg!(&self, count, result);
        result
    }

    fn is_valid_for_part_2(self) -> bool {
        let c = self.input.as_bytes();
        let target = self.letter;
        match (
            c[self.min - 1] as char == target,
            c[self.max - 1] as char == target,
        ) {
            (true, false) | (false, true) => true,
            _ => false,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                PasswordValidation::parse(line)
                    .unwrap()
                    .1
                    .is_valid_for_part_1()
            })
            .filter(|v| *v == true)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                match PasswordValidation::parse(line)
                    .unwrap()
                    .1
                    .is_valid_for_part_2()
                {
                    true => 1,
                    false => 0,
                }
            })
            .sum(),
    )
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
