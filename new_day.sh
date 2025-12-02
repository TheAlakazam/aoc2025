#!/bin/bash

# Script to create a new day's solution file

if [ -z "$1" ]; then
    echo "Usage: ./new_day.sh <day_number>"
    echo "Example: ./new_day.sh 5"
    exit 1
fi

DAY=$1
DAY_PADDED=$(printf "%02d" $DAY)

# Create the solution file
cat > "src/solutions/day${DAY_PADDED}.rs" << 'EOF'
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
EOF

echo "✓ Created src/solutions/day${DAY_PADDED}.rs"

# Add to mod.rs if not already there
if ! grep -q "pub mod day${DAY_PADDED};" src/solutions/mod.rs; then
    echo "pub mod day${DAY_PADDED};" >> src/solutions/mod.rs
    echo "✓ Added day${DAY_PADDED} to src/solutions/mod.rs"
else
    echo "! day${DAY_PADDED} already in src/solutions/mod.rs"
fi

# Remind to update main.rs
echo ""
echo "Don't forget to add to src/main.rs in the run_solution function:"
echo "    ${DAY} => solutions::day${DAY_PADDED}::solve(&input, part),"
echo ""
echo "Then download input with:"
echo "    cargo run -- download ${DAY}"
