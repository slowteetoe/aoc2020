// Since the format is a little wonky, let's normalize it first - records are separated with a \n\n right now - we'll turn that into a special char
// and then convert all the remaining \n to spaces.  We can then split on the special char, and then deal with all the key:value<space>key:value...

use std::collections::BTreeMap;

use itertools::Itertools;
use nom::AsChar;

pub fn parse(input: &str) -> Vec<BTreeMap<String, String>> {
    let input = input
        .replace("\n\n", "@")
        .replace("\n", " ")
        .replace("@", "\n");
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|kv| kv.split_at(kv.find(":").unwrap()))
                .map(|(k, v)| (k.to_owned(), v[1..].to_owned()))
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    // this is silly, "cid" doesn't actually have any bearing on part 1 even though it's mentioned
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let records = parse(input);
    Some(
        records
            .iter()
            .map(|r| {
                if required.iter().all(|f| r.contains_key(*f)) {
                    1
                } else {
                    0
                }
            })
            .sum(),
    )
}

struct Height {
    val: u16,
    unit: String,
}

fn parse_hgt(input: &str) -> Height {
    let val = input.chars().take_while(|c| c.is_numeric()).join("");
    let val = str::parse::<u16>(&val).unwrap();
    let unit = input
        .chars()
        .skip_while(|c| c.is_numeric())
        .take_while(|c| c.is_ascii())
        .join("");
    Height { val, unit }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut required: BTreeMap<&str, Box<dyn Fn(&str) -> bool>> = BTreeMap::new();
    required.insert(
        "byr",
        Box::new(|val| (1920..=2002).contains(&str::parse::<u32>(val).unwrap())),
    );
    required.insert(
        "iyr",
        Box::new(|val| (2010..=2020).contains(&str::parse::<u32>(val).unwrap())),
    );
    required.insert(
        "eyr",
        Box::new(|val| (2020..=2030).contains(&str::parse::<u32>(val).unwrap())),
    );
    required.insert(
        "hcl",
        Box::new(|val| {
            val.starts_with("#") && val.chars().into_iter().skip(1).all(|c| c.is_hex_digit())
        }),
    );
    required.insert(
        "hgt",
        Box::new(|val| {
            let h = parse_hgt(val);
            match h.unit.as_str() {
                "cm" => (150..=193).contains(&h.val),
                "in" => (59..=76).contains(&h.val),
                _ => false,
            }
        }),
    );
    required.insert("ecl", Box::new(|val| match val {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false
    }));
    required.insert(
        "pid",
        Box::new(|val| val.len() == 9 && val.chars().all(|c| c.is_digit(10))),
    );
    // !["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let records = parse(input);
    Some(
        records
            .iter()
            .map(|r| {
                if required
                    .iter()
                    .all(|(key, f)| r.get(*key).is_some_and(|val| f(val)))
                {
                    1
                } else {
                    0
                }
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(2));
    }
}
