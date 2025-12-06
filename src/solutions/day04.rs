use anyhow::Result;

pub fn solve(input: &str, part: u8) -> Result<String> {
    match part {
        1 => solve_part1(input),
        2 => solve_part2(input),
        _ => anyhow::bail!("Invalid part number"),
    }
}

fn solve_part1(input: &str) -> Result<String> {
    let data = parse_input(input)?;
    let m = data.len();
    let n = data[0].len();
    let mut accessible_count = 0;
    for i in 0..m {
        for j in 0..n {
            if data[i][j] == '@' && is_roll_accessible(&data, i, j, m, n) {
                accessible_count += 1;
            }
        }
    }
    Ok(accessible_count.to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    let mut data = parse_input(input)?;
    let m = data.len();
    let n = data[0].len();
    let mut accessible_count = 0;
    for i in 0..m {
        for j in 0..n {
            if data[i][j] == '@' && is_roll_accessible(&data, i, j, m, n) {
                accessible_count += 1;
            }
        }
    }

    let mut removable_count = accessible_count;
    accessible_count = 0;

    while removable_count > 0 {
        removable_count = 0;
        for i in 0..m {
            for j in 0..n {
                if data[i][j] == '@' && is_roll_accessible(&data, i, j, m, n) {
                    accessible_count += 1;
                    data[i][j] = '.';
                    removable_count += 1;
                }
            }
        }
    }

    Ok(accessible_count.to_string())
}

fn parse_input(input: &str) -> Result<Vec<Vec<char>>> {
    Ok(input
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|line: &Vec<char>| !line.is_empty())
        .collect())
}

fn is_roll_accessible(data: &Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) -> bool {
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    for (dx, dy) in dirs.iter() {
        let new_x = i as isize + dx;
        let new_y = j as isize + dy;
        if new_x >= 0 && new_x < m as isize && new_y >= 0 && new_y < n as isize {
            count += (data[new_x as usize][new_y as usize] == '@') as usize;
        }
    }
    count < 4
}
