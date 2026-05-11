use std::fs::File;
use std::io::{BufReader, Bytes, Read};

fn find_marker(
    bytes: Bytes<BufReader<File>>,
    marker_length: usize,
) -> Result<usize, std::io::Error> {
    let mut byte_buffer = Vec::with_capacity(marker_length);
    let mut buffer_index = 0;

    'outer: for (i, byte) in bytes.enumerate() {
        if byte_buffer.len() < marker_length {
            byte_buffer.push(byte?);
        } else {
            byte_buffer[buffer_index] = byte?;
            buffer_index = (buffer_index + 1) % marker_length;
        }

        // This block needs to be separate to prevent an edge case where pushing a byte to the
        // byte_buffer vector changes its length to equal marker_length. If this block of code
        // was in the else statement it would then end up skipping the first potential marker
        if byte_buffer.len() == marker_length {
            for (j, byte_1) in byte_buffer.iter().enumerate() {
                for (k, byte_2) in byte_buffer.iter().enumerate() {
                    if j != k && byte_1 == byte_2 {
                        continue 'outer;
                    }
                }
            }
            return Ok(i + 1);
        }
    }
    return Result::Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No marker found",
    ))?;
}

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let bytes = BufReader::new(file).bytes();
    return Ok(find_marker(bytes, 4)?);
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let bytes = BufReader::new(file).bytes();
    return Ok(find_marker(bytes, 14)?);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - First marker found at character {part_1}");
    println!("Part 2 - First message found at character {part_2}");
    return Ok(());
}
