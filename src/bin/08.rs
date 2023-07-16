pub fn part_one(input: &str) -> Option<u32> {
    let mut instr = parse(input);
    let mut acc = 0;
    let mut i = 0;
    while i < instr.len() {
        // dbg!(&i, &instr[i], &acc);
        if instr[i].acc.is_some() {
            return Some(acc);
        }
        match instr[i].op.as_str() {
            "nop" => {
                instr[i].acc = Some(acc.clone());
                i += 1;
            }
            "acc" => {
                acc = match instr[i].amount {
                    ..=0 => acc.wrapping_sub(instr[i].amount.abs() as u32),
                    1.. => acc + instr[i].amount as u32,
                };
                instr[i].acc = Some(acc.clone());
                i += 1;
            }
            "jmp" => {
                instr[i].acc = Some(acc.clone());
                i = match instr[i].amount {
                    ..=0 => i.wrapping_sub(instr[i].amount.abs() as usize),
                    1.. => i + instr[i].amount as usize,
                }
            }
            _ => unreachable!(),
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let instr = parse(input);
    // just in case the instructions are fine as-is, this guarantees we've tried the base case so we can just mutate in our loop below
    let orig = execute_to_completion(&instr);
    if orig.is_some() {
        return orig;
    }

    // run through the instructions, every time we encounter a jmp or nop, try a variation that switches that instruction
    for i in 1..instr.len() {
        match instr[i].op.as_str() {
            "jmp" => {
                let mut new_vec = instr.to_vec();
                new_vec[i].op = "nop".to_owned();
                match execute_to_completion(&new_vec) {
                    Some(solution) => return Some(solution),
                    None => continue,
                }
            }
            "nop" => {
                let mut new_vec = instr.to_vec();
                new_vec[i].op = "jmp".to_owned();
                match execute_to_completion(&instr) {
                    Some(solution) => return Some(solution),
                    None => continue,
                }
            }
            _ => continue,
        }
    }
    None
}

fn execute_to_completion(instr: &Vec<Instruction>) -> Option<u32> {
    let mut acc = 0;
    let mut i = 0;
    let mut instr = instr.to_vec();
    while i < instr.len() {
        if instr[i].acc.is_some() {
            return None; // if we looped, this isn't viable
        }
        match instr[i].op.as_str() {
            "nop" => {
                instr[i].acc = Some(acc.clone());
                i += 1;
            }
            "acc" => {
                acc = match instr[i].amount {
                    ..=0 => acc.wrapping_sub(instr[i].amount.abs() as u32),
                    1.. => acc + instr[i].amount as u32,
                };
                instr[i].acc = Some(acc.clone());
                i += 1;
            }
            "jmp" => {
                instr[i].acc = Some(acc.clone());
                i = match instr[i].amount {
                    ..=0 => i.wrapping_sub(instr[i].amount.abs() as usize),
                    1.. => i + instr[i].amount as usize,
                }
            }
            _ => unreachable!(),
        }
    }
    Some(acc)
}

#[derive(Debug, Clone)]
pub struct Instruction {
    op: String,
    amount: i16,
    acc: Option<u32>,
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (op, amt) = line.split_once(' ').unwrap();
            Instruction {
                op: op.to_owned(),
                amount: amt.trim().parse().unwrap(),
                acc: None,
            }
        })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
