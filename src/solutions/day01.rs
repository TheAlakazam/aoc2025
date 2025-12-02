use anyhow::Result;

pub fn solve(input: &str, part: u8) -> Result<String> {
    match part {
        1 => solve_part1(input),
        2 => solve_part2(input),
        _ => anyhow::bail!("Invalid part number"),
    }
}

fn solve_part1(input: &str) -> Result<String> {
    let mut position: i32 = 50;
    let mut result: i32 = 0;
    for line in input.lines() {
        let (instruction, step) = parse_instruction(line);
        position = rotate(&position, &instruction, &step);
        result = result + (if position == 0 { 1 } else { 0 });
    }
    Ok(result.to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    let mut position: i32 = 50;
    let mut result: i32 = 0;
    for line in input.lines() {
        let (instruction, step) = parse_instruction(line);
        let mut stepper: i32 = step;
        let amount = match instruction.as_str() {
            "L" => -1,
            "R" => 1,
            _ => 0,
        };
        while stepper > 0 {
            position = (position - amount).rem_euclid(100);
            if position == 0 {
                result += 1;
            }
            stepper -= 1;
        }
    }
    Ok(result.to_string())
}

fn rotate(current: &i32, direction: &str, step: &i32) -> i32 {
    match direction {
        "L" => ((current - step) % 100 + 100) % 100,
        "R" => (current + step) % 100,
        _ => *current,
    }
}

fn parse_instruction(instruction: &str) -> (String, i32) {
    let direction = instruction.chars().next().unwrap().to_string();
    let step = instruction[1..].parse::<i32>().unwrap();
    (direction, step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ""; // Add test input
        assert_eq!(solve_part1(input).unwrap(), "expected");
    }

    #[test]
    fn test_part2() {
        let input = ""; // Add test input
        assert_eq!(solve_part2(input).unwrap(), "expected");
    }
}
