// Since the format is a little wonky, let's normalize it first - records are separated with a \n\n right now - we'll turn that into a special char
// and then convert all the remaining \n to spaces.  We can then split on the special char, and then deal with all the key:value<space>key:value...

use std::collections::BTreeMap;

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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
