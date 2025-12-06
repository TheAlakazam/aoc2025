use anyhow::Result;

pub fn solve(input: &str, part: u8) -> Result<String> {
    match part {
        1 => solve_part1(input),
        2 => solve_part2(input),
        _ => anyhow::bail!("Invalid part number"),
    }
}

fn solve_part1(input: &str) -> Result<String> {
    let mut result: u64 = 0;
    for parts in input.split(',') {
        let (start, end) = parse_range(parts.trim())?;
        for pid in start..=end {
            let pid_str = pid.to_string();
            if !is_pid_valid(&pid_str) {
                result += pid as u64;
            }
        }
    }
    Ok(result.to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    let mut result: u64 = 0;
    for parts in input.split(',') {
        let (start, end) = parse_range(parts.trim())?;
        for pid in start..=end {
            let pid_str = pid.to_string();
            if !is_pid_valid(&pid_str) {
                result += pid as u64;
            }
        }
    }
    Ok(result.to_string())
}

fn parse_range(range_str: &str) -> Result<(u64, u64)> {
    let parts: Vec<&str> = range_str.split('-').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid range format");
    }
    let start: u64 = parts[0].parse()?;
    let end: u64 = parts[1].parse()?;
    Ok((start, end))
}

fn is_pid_valid(pid: &str) -> bool {
    let len = pid.len();
    for d in 1..=(len / 2) {
        if len % d == 0 {
            let pattern = &pid[0..d];
            let num_repeats = len / d;

            if num_repeats >= 2 {
                let repeated = pattern.repeat(num_repeats);
                if repeated == pid {
                    return false;
                }
            }
        }
    }
    true
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
