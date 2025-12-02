use anyhow::Result;

pub fn solve(input: &str, part: u8) -> Result<String> {
    match part {
        1 => solve_part1(input),
        2 => solve_part2(input),
        _ => anyhow::bail!("Invalid part number"),
    }
}

fn solve_part1(input: &str) -> Result<String> {
    // TODO: Implement part 1
    Ok("Not yet implemented".to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    // TODO: Implement part 2
    Ok("Not yet implemented".to_string())
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
