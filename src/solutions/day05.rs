use anyhow::Result;

pub fn solve(input: &str, part: u8) -> Result<String> {
    match part {
        1 => solve_part1(input),
        2 => solve_part2(input),
        _ => anyhow::bail!("Invalid part number"),
    }
}

fn solve_part1(input: &str) -> Result<String> {
    let (range_tree, ingredients) = parse_input(input)?;

    Ok(ingredients
        .into_iter()
        .filter(|&ing| range_tree.contains(ing))
        .count()
        .to_string())
}

fn solve_part2(input: &str) -> Result<String> {
    let (range_tree, ingredients) = parse_input(input)?;
    Ok(range_tree
        .intervals
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum::<i64>()
        .to_string())
}

fn parse_input(input: &str) -> Result<(FreshRangeIntervalTree, Vec<i64>)> {
    let (range_str, ingredients_str) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow::anyhow!("Invalid input format"))?;
    println!("Range Str:\n{}", range_str);
    println!("Ingredients Str:\n{}", ingredients_str);

    let ranges: Vec<(i64, i64)> = range_str
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.trim().split('-').collect();
            let start = parts[0].parse::<i64>().unwrap();
            let end = parts[1].parse::<i64>().unwrap();
            (start, end)
        })
        .collect();

    let ingredients: Vec<i64> = ingredients_str
        .lines()
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect();

    Ok((FreshRangeIntervalTree::new(ranges), ingredients))
}

struct FreshRangeIntervalTree {
    intervals: Vec<(i64, i64)>,
}

impl FreshRangeIntervalTree {
    fn new(ranges: Vec<(i64, i64)>) -> Self {
        let intervals = Self::merge_overlapping_ranges(ranges);
        Self { intervals }
    }

    fn contains(&self, value: i64) -> bool {
        let idx = self.intervals.binary_search_by(|(start, _)| {
            if *start <= value {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        let idx = match idx {
            Ok(i) => i,
            Err(i) => {
                if i == 0 {
                    return false;
                }
                i - 1
            }
        };

        let (start, end) = self.intervals[idx];
        value >= start && value <= end
    }

    fn merge_overlapping_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        if ranges.is_empty() {
            return ranges;
        }

        ranges.sort_by_key(|k| k.0);

        let mut merged = Vec::new();
        let mut current = ranges[0];

        for &(start, end) in ranges.iter().skip(1) {
            if start <= current.1 + 1 {
                current.1 = current.1.max(end);
            } else {
                merged.push(current);
                current = (start, end);
            }
        }

        merged.push(current);

        merged
    }
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
