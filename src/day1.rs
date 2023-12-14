use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("/home/kanishk/advent-of-code-2023/inputs/day1-part1-input.txt")?;
    let reader = BufReader::new(file);
    let all_lines = reader.lines().map(|s| s.unwrap()).collect::<Vec<String>>();

    let total_sum = periphery_digit_sum(all_lines.clone());
    println!("[Part 1] Total: {}", total_sum);

    let conversion_map = HashMap::from([
        ("zero", "zero0zero"),
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ]);
    let converted_sum = periphery_digit_sum(
        all_lines
            .into_iter()
            .map(|line| {
                let mut line_clone = line.clone();
                conversion_map
                    .iter()
                    .for_each(|(k, v)| line_clone = line_clone.replace(k, v));
                return line_clone
            })
            .collect::<Vec<String>>()
            .clone()
    );
    println!("[Part 2] Converted sum: {}", converted_sum);

    Ok(())
}

fn periphery_digit_sum(lines: Vec<String>) -> i32 {
    return lines
        .into_iter()
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<String>())
        .map(|line| [line.chars().next().unwrap(), line.chars().last().unwrap()])
        .map(|char_list| char_list.iter().collect::<String>())
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>();
}
