use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("/home/kanishk/advent-of-code-2023/inputs/day1-part1-input.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        // println!("{}", line?);

        let mut first = '0';
        let mut second = '0';
        let mut character_seen = false;
        for c in line?.chars() {
            if c.is_numeric() {
                if character_seen == false {
                    first = c;
                    character_seen = true;
                }
                second = c;
            }
        }

        total = total + (10 * first.to_digit(10).unwrap_or(0) + second.to_digit(10).unwrap_or(0));
        println!("First: {} Second: {}", first.to_digit(10).unwrap_or(0), second.to_digit(10).unwrap_or(0));
    }

    println!("Total: {}", total);

    Ok(())
}
