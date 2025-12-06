use anyhow::Result;

pub fn solve(input: &str, part: u8) -> Result<String> {
    match part {
        1 => solve_part1(input),
        2 => solve_part2(input),
        _ => anyhow::bail!("Invalid part number"),
    }
}

fn solve_part1(input: &str) -> Result<String> {
    let banks = parse_input(input)?;
    let res = banks
        .iter()
        .map(|bank| find_max_joltage(bank, 2))
        .sum::<u64>();
    Ok(res.to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    let banks = parse_input(input)?;
    let res = banks
        .iter()
        .map(|bank| find_max_joltage(bank, 12))
        .sum::<u64>();
    Ok(res.to_string())
}

fn parse_input(input: &str) -> Result<Vec<&str>> {
    Ok(input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect())
}

fn find_max_joltage(bank: &str, k: usize) -> u64 {
    let digits: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let n = digits.len();

    if k > n {
        panic!(
            "k ({}) cannot be greater than the number of digits ({})",
            k, n
        );
    }

    let mut result = Vec::with_capacity(k);
    let mut start = 0;

    for i in 0..k {
        let remaining = k - i - 1;
        let search_end = n - remaining;

        let max_digit = digits[start..search_end].iter().max().unwrap();

        let max_idx = digits[start..search_end]
            .iter()
            .position(|d| d == max_digit)
            .unwrap();

        let actual_idx = start + max_idx;
        result.push(digits[actual_idx]);
        start = actual_idx + 1;
    }

    result.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_joltage_k2() {
        assert_eq!(find_max_joltage("987654321111111", 2), 98);
        assert_eq!(find_max_joltage("811111111111119", 2), 89);
        assert_eq!(find_max_joltage("234234234234278", 2), 78);
        assert_eq!(find_max_joltage("818181911112111", 2), 92);
    }

    #[test]
    fn test_find_max_joltage_k3() {
        assert_eq!(find_max_joltage("987654321111111", 3), 987);
        assert_eq!(find_max_joltage("12345", 3), 345);
    }

    #[test]
    fn test_part1() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(solve_part1(input).unwrap(), "357");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(find_max_joltage("99999", 2), 99);
        assert_eq!(find_max_joltage("12345", 2), 45);
        assert_eq!(find_max_joltage("54321", 2), 54);
    }
}
