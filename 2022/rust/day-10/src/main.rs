use std::fs::File;
use std::io::{BufRead, BufReader};

fn solution_part_1() -> Result<isize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e}"))?;
    let mut reader = BufReader::new(file);
    let mut line_buffer = String::with_capacity(8);
    let mut x_register = 1;
    let mut instruction_value = 0;
    let mut instruction_counter = 0;
    let mut cycle_counter = 1;
    let mut cycle_target = 20;
    let mut signal_strength_sum = 0;

    loop {
        if instruction_counter == 0 {
            x_register += instruction_value;

            if reader.read_line(&mut line_buffer)? != 0 {
                if line_buffer.starts_with("noop") {
                    instruction_value = 0;
                    instruction_counter = 1;
                } else if line_buffer.starts_with("addx") {
                    instruction_value = line_buffer
                        .trim_start_matches("addx")
                        .trim()
                        .parse::<isize>()?;
                    instruction_counter = 2;
                }
                line_buffer.clear();
            } else {
                return Result::Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Not enough instructions in input file",
                ))?;
            }
        }
        if cycle_counter == cycle_target {
            signal_strength_sum += cycle_counter * x_register;
            cycle_target += 40;
        }
        if cycle_counter == 220 {
            return Ok(signal_strength_sum);
        } else {
            instruction_counter -= 1;
            cycle_counter += 1;
        }
    }
}

fn solution_part_2() -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e}"))?;
    let mut reader = BufReader::new(file);
    let mut line_buffer = String::with_capacity(8);
    let mut image_buffer = String::with_capacity(246);
    let mut x_register = 1;
    let mut instruction_value = 0;
    let mut instruction_counter = 0;

    for _ in 0..6 {
        for x in 0..40 {
            if instruction_counter == 0 {
                x_register += instruction_value;

                if reader.read_line(&mut line_buffer)? != 0 {
                    if line_buffer.starts_with("noop") {
                        instruction_value = 0;
                        instruction_counter = 1;
                    } else if line_buffer.starts_with("addx") {
                        instruction_value = line_buffer
                            .trim_start_matches("addx")
                            .trim()
                            .parse::<isize>()?;
                        instruction_counter = 2;
                    }
                    line_buffer.clear();
                } else {
                    return Result::Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "Not enough instructions in input file",
                    ))?;
                }
            }

            if x >= x_register - 1 && x <= x_register + 1 {
                image_buffer.push('#');
            } else {
                image_buffer.push('.');
            }
            instruction_counter -= 1;
        }
        image_buffer.push('\n');
    }
    return Ok(image_buffer);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Signal Strength Sum: {part_1}");
    println!("Part 2 - Image:\n\n{part_2}");
    return Ok(());
}
