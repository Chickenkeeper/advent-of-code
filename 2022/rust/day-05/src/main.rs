use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
struct CrateMovement {
    num_crates: usize,
    from_stack: usize,
    to_stack: usize,
}

impl CrateMovement {
    fn from_string(movement: &str) -> Result<CrateMovement, Box<dyn std::error::Error>> {
        let par_err = "Invalid Movement Parameter";
        let mut tokens = movement.split_whitespace().skip(1).step_by(2);
        let num_crates = tokens.next().ok_or_else(|| par_err)?.parse::<usize>()?;
        let from_stack = tokens.next().ok_or_else(|| par_err)?.parse::<usize>()?;
        let to_stack = tokens.next().ok_or_else(|| par_err)?.parse::<usize>()?;

        if from_stack == 0 || to_stack == 0 {
            return Result::Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Stack numbers must start from 1",
            ))?;
        }

        return Ok(CrateMovement {
            num_crates,
            from_stack: from_stack - 1,
            to_stack: to_stack - 1,
        });
    }
}

struct CrateStacks {
    stacks: Vec<Vec<char>>,
}

impl CrateStacks {
    fn from_file(reader: &mut BufReader<File>) -> Result<CrateStacks, std::io::Error> {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let mut line = String::with_capacity(32);

        while reader.read_line(&mut line)? != 0 {
            let bytes = line.as_bytes();

            if (bytes[1] as char).is_ascii_digit() {
                break;
            }

            while stacks.len() < (bytes.len() + 1) / 4 {
                stacks.push(Vec::new());
            }

            for (i, b) in bytes.iter().skip(1).step_by(4).enumerate() {
                if *b != b' ' {
                    stacks[i].push(*b as char);
                }
            }
            line.clear();
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }

        return Ok(CrateStacks { stacks });
    }

    fn move_crates_sequential(&mut self, movement: CrateMovement) -> Result<(), &'static str> {
        let from_stack_len = self.stacks[movement.from_stack].len();
        if from_stack_len < movement.num_crates {
            return Err("Stack does not contain enough crates");
        }

        let start_index = from_stack_len - 1;
        for i in 0..movement.num_crates {
            let c = self.stacks[movement.from_stack][start_index - i];
            self.stacks[movement.to_stack].push(c);
        }

        self.stacks[movement.from_stack].truncate(start_index + 1 - movement.num_crates);
        return Ok(());
    }

    fn move_crates_grouped(&mut self, movement: CrateMovement) -> Result<(), &'static str> {
        let from_stack_len = self.stacks[movement.from_stack].len();
        if from_stack_len < movement.num_crates {
            return Err("Stack does not contain enough crates");
        }

        let start_index = from_stack_len - movement.num_crates;
        for i in 0..movement.num_crates {
            let c = self.stacks[movement.from_stack][start_index + i];
            self.stacks[movement.to_stack].push(c);
        }

        self.stacks[movement.from_stack].truncate(start_index);
        return Ok(());
    }

    fn get_top_crates(&mut self) -> String {
        let mut top_crates = String::new();
        for stack in self.stacks.iter() {
            let top_crate = stack[stack.len().saturating_sub(1)];
            top_crates.push(top_crate);
        }

        return top_crates;
    }
}

fn solution_part_1() -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(16);
    let mut crate_stacks = CrateStacks::from_file(&mut reader)?;

    while reader.read_line(&mut line)? != 0 {
        let l = line.trim();

        if !l.is_empty() {
            let crate_movement = CrateMovement::from_string(l)?;
            crate_stacks.move_crates_sequential(crate_movement)?;
            line.clear();
        }
    }

    return Ok(crate_stacks.get_top_crates());
}

fn solution_part_2() -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(16);
    let mut crate_stacks = CrateStacks::from_file(&mut reader)?;

    while reader.read_line(&mut line)? != 0 {
        let l = line.trim();

        if !l.is_empty() {
            let crate_movement = CrateMovement::from_string(l)?;
            crate_stacks.move_crates_grouped(crate_movement)?;
            line.clear();
        }
    }

    return Ok(crate_stacks.get_top_crates());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Top crates: {part_1}");
    println!("Part 2 - Top crates: {part_2}");
    return Ok(());
}
