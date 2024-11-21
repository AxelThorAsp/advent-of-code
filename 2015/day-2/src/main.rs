use itertools::Itertools;
use std::fs::read_to_string;
use std::io::Result;

fn process_line(directions: &str) -> u32 {
    let sides = directions
        .split('x')
        .map(|substring| substring.parse::<u32>().unwrap_or_default());
    2 * (sides.clone().sum::<u32>() - sides.clone().max().unwrap_or_default())
        + sides.product::<u32>()
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let total: u32 = input.lines().map(|line| process_line(line)).sum();

    println!("{total}");

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read input file: {e}");
        std::process::exit(1);
    })
}