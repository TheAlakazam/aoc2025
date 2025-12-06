mod solutions;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aoc2025")]
#[command(about = "Advent of Code 2025 Solutions", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the solution for a specific day
    Run {
        /// Day number (1-25)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
        day: u8,

        /// Part number (1 or 2)
        #[arg(short, long, default_value = "1")]
        #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
        part: u8,
    },

    /// Download input for a specific day
    Download {
        /// Day number (1-25)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
        day: u8,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { day, part } => {
            run_solution(day, part)?;
        }
        Commands::Download { day } => {
            download_input(day)?;
        }
    }

    Ok(())
}

fn run_solution(day: u8, part: u8) -> Result<()> {
    let input_path = format!("inputs/day{:02}.txt", day);
    let input = fs::read_to_string(&input_path)
        .with_context(|| format!("Failed to read input file: {}", input_path))?;

    println!("Running Day {} - Part {}", day, part);
    println!("Input length: {} characters\n", input.len());

    let result = match day {
        1 => solutions::day01::solve(&input, part),
        2 => solutions::day02::solve(&input, part),
        3 => solutions::day03::solve(&input, part),
        4 => solutions::day04::solve(&input, part),
        5 => solutions::day05::solve(&input, part),
        // 6 => solutions::day06::solve(&input, part),
        // 7 => solutions::day07::solve(&input, part),
        // 8 => solutions::day08::solve(&input, part),
        // 9 => solutions::day09::solve(&input, part),
        // 10 => solutions::day10::solve(&input, part),
        // 11 => solutions::day11::solve(&input, part),
        // 12 => solutions::day12::solve(&input, part),
        _ => {
            anyhow::bail!("Day {} not yet implemented", day);
        }
    };

    match result {
        Ok(answer) => {
            println!("Answer: {}", answer);
        }
        Err(e) => {
            eprintln!("Error solving Day {} Part {}: {}", day, part, e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn download_input(day: u8) -> Result<()> {
    let session_token = get_session_token()?;
    let url = format!("https://adventofcode.com/2025/day/{}/input", day);
    let output_path = format!("inputs/day{:02}.txt", day);

    println!("Downloading input for Day {}...", day);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_token))
        .header(
            "User-Agent",
            "github.com/yourusername/aoc2025 by your@email.com",
        )
        .send()
        .context("Failed to send request")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download input: HTTP {} - Check your session token and ensure Day {} is available",
            response.status(),
            day
        );
    }

    let content = response.text().context("Failed to read response")?;

    fs::create_dir_all("inputs").context("Failed to create inputs directory")?;
    fs::write(&output_path, content).context("Failed to write input file")?;

    println!("Successfully saved to {}", output_path);
    Ok(())
}

fn get_session_token() -> Result<String> {
    // Try to read from .env file
    let env_path = PathBuf::from(".env");
    if env_path.exists() {
        let content = fs::read_to_string(&env_path)?;
        for line in content.lines() {
            if let Some(token) = line.strip_prefix("AOC_SESSION=") {
                return Ok(token.trim().to_string());
            }
        }
    }

    // Try environment variable
    if let Ok(token) = std::env::var("AOC_SESSION") {
        return Ok(token);
    }

    anyhow::bail!(
        "Session token not found. Please set AOC_SESSION in .env file or as environment variable.\n\
         Get your session token from https://adventofcode.com (check browser cookies)"
    );
}
