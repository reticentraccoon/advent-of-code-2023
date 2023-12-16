use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("/home/kanishk/advent-of-code-2023/inputs/day2-part1-input.txt")?;
    let reader = BufReader::new(file);
    let all_lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();

    let mut sum = 0;
    let mut power_sum = 0;
    all_lines.into_iter()
        .for_each(|s| {
            let parts = s.split(": ").collect::<Vec<_>>();
            let game_number = &parts[0].split(" ").collect::<Vec<_>>()[1].parse::<i32>().unwrap();

            let attempts = &parts[1].split("; ").collect::<Vec<_>>();
            let mut is_cool = true;
            let mut red_max = 0;
            let mut blue_max = 0;
            let mut green_max = 0;
            for attempt in attempts {
                let colors = attempt.split(", ").collect::<Vec<_>>();

                let mut red_sum = 0;
                let mut blue_sum = 0;
                let mut green_sum = 0;
                for color in colors {
                    let num_and_color = color.split(" ").collect::<Vec<_>>();
                    let num = &num_and_color[0].parse::<i32>().unwrap();
                    let color_for_num = &num_and_color[1].to_string();

                    match color_for_num.as_str() {
                        "red" => {
                            red_sum += num;
                            if red_max < *num { red_max = *num; }
                        },
                        "blue" => {
                            blue_sum += num;
                            if blue_max < *num { blue_max = *num; }
                        },
                        "green" => {
                            green_sum += num;
                            if green_max < *num { green_max = *num; }
                        },
                        _ => println!("Skip."),
                    }
                }

                if red_sum > 12 || green_sum > 13 || blue_sum > 14 {
                    is_cool = false;
                }
            }
            let power = red_max * blue_max * green_max;
            power_sum += power;
            if is_cool { sum += game_number; }
        });

    println!("Sum-12-13-14: {} Power sum: {}", sum, power_sum);

    Ok(())
}