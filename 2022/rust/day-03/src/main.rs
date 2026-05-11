use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_item_priority(item: char) -> Result<usize, &'static str> {
    if item.is_ascii_lowercase() {
        return Ok((item as usize) - (b'a' as usize) + 1);
    } else if item.is_ascii_uppercase() {
        return Ok((item as usize) - (b'A' as usize) + 27);
    } else {
        return Err("Items must be uppercase or lowercase ASCII characters");
    }
}

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(8);
    let mut priority_sum = 0;

    'outer: while reader.read_line(&mut line)? != 0 {
        let l = line.trim();
        let compartments = l.split_at(l.len() / 2);

        for c0 in compartments.0.chars() {
            for c1 in compartments.1.chars() {
                if c0 == c1 {
                    priority_sum += get_item_priority(c1)?;
                    line.clear();
                    continue 'outer;
                }
            }
        }
        return Err("No common item found")?;
    }
    return Ok(priority_sum);
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let mut lines = BufReader::new(file).lines().peekable();
    let mut priority_sum = 0;

    'outer: while lines.peek().is_some() {
        let mut rucksacks: [String; 3] = Default::default();

        for rucksack in rucksacks.iter_mut() {
            if let Some(line) = lines.next() {
                *rucksack = line?;
            } else {
                return Result::Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Not enough items in rucksack",
                ))?;
            }
        }

        for item_1 in rucksacks[0].chars() {
            for item_2 in rucksacks[1].chars() {
                for item_3 in rucksacks[2].chars() {
                    if item_1 == item_2 && item_1 == item_3 {
                        priority_sum += get_item_priority(item_1)?;
                        continue 'outer;
                    }
                }
            }
        }
        return Err("No common item found")?;
    }
    return Ok(priority_sum);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Duplicate item priority sum: {part_1}");
    println!("Part 2 - Badge priority sum: {part_2}");
    return Ok(());
}
