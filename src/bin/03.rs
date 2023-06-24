pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);
    let width = map.first().unwrap().len();
    // move right 3 down 1, using infinitely repeating map to the right
    let mut real_x = 0;
    let mut tree_count = 0;
    for y in 0..map.len() {
        let x = real_x % width;
        if map[y][x] {
            tree_count += 1;
        }
        real_x += 3;
    }
    Some(tree_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_map(input);
    let width = map.first().unwrap().len();
    // move right 3 down 1, using infinitely repeating map to the right
    let rounds = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees = 1;
    for (dx, dy) in rounds.iter() {
        let mut real_x = 0;
        let mut tree_count = 0;
        let mut y = 0;
        while y < map.len() {
            let x = real_x % width;
            if map[y][x] {
                tree_count += 1;
            }
            real_x += dx;
            y += dy;
        }
        // dbg!(dx, dy, tree_count);
        trees *= tree_count;
    }
    Some(trees)
}

fn parse_map(input: &str) -> Vec<Vec<bool>> {
    input
    .lines()
    .map(|line| {
        line.chars()
            .into_iter()
            .map(|c| c.to_string() == "#")
            .collect()
    })
    .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(336));
    }
}
