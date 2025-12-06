use anyhow::Result;

pub fn solve(input: &str, part: u8) -> Result<String> {
    match part {
        1 => solve_part1(input),
        2 => solve_part2(input),
        _ => anyhow::bail!("Invalid part number"),
    }
}

fn solve_part1(input: &str) -> Result<String> {
    let (problems, operations) = parse_input(input)?;
    let n = problems.len();
    let m = operations.len();

    println!("Number of problems: {}, Number of operations: {}", m, n);

    let mut res_vec = vec![0i64; m];

    for i in 0..m {
        let op = operations[i];
        let mut start = match op {
            "*" => 1,
            "+" => 0,
            _ => anyhow::bail!("Invalid operation"),
        };
        for j in 0..n {
            let val = problems[j][i];
            match op {
                "*" => start *= val,
                "+" => start += val,
                _ => unreachable!(),
            }
        }
        res_vec[i] = start;
    }

    Ok(res_vec.iter().sum::<i64>().to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    let n_rows = lines.len();
    let n_cols = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut total = 0i64;
    let mut col = (n_cols - 1) as isize;

    while col >= 0 {
        let mut numbers: Vec<i64> = Vec::new();

        while col >= 0 {
            let column_chars: Vec<char> = lines
                .iter()
                .map(|line| {
                    if col < line.len() as isize {
                        line.chars().nth(col as usize).unwrap()
                    } else {
                        ' '
                    }
                })
                .collect();

            if column_chars.iter().all(|&c| c == ' ') {
                col -= 1;
                continue;
            }

            let mut number_str = String::new();
            for row in 0..(n_rows - 1) {
                let ch = column_chars[row];
                if ch.is_digit(10) {
                    number_str.push(ch);
                }
            }
            if !number_str.is_empty() {
                let number = number_str.parse::<i64>().unwrap();
                numbers.push(number);
            }

            let op = column_chars[n_rows - 1];
            if op == '+' || op == '*' {
                let result = match op {
                    '+' => numbers.iter().sum::<i64>(),
                    '*' => numbers.iter().product::<i64>(),
                    _ => unreachable!(),
                };

                total += result;
                col -= 1;
                break;
            }

            col -= 1;
        }
    }
    Ok(total.to_string())
}

fn parse_input(input: &str) -> Result<(Vec<Vec<i64>>, Vec<&str>)> {
    let lines = input.lines().collect::<Vec<&str>>();
    let n = lines.len();
    let operations = lines[n - 1].split_whitespace().collect::<Vec<&str>>();

    let problems = lines[0..n - 1]
        .iter()
        .map(|line| {
            line.split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    Ok((problems, operations))
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
