use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

#[derive(Clone, Copy)]
struct Assignment {
    section_start: usize,
    section_end: usize,
}

impl Assignment {
    fn new(section_start: usize, section_end: usize) -> Self {
        return Assignment {
            section_start,
            section_end,
        };
    }

    fn from_line(line: &str) -> Result<(Assignment, Assignment), ParseIntError> {
        let mut sections = [0; 4];

        for (i, section) in line.split(&[',', '-']).enumerate() {
            sections[i] = section.parse::<usize>()?;
        }

        let elf_1_sections = Assignment::new(sections[0], sections[1]);
        let elf_2_sections = Assignment::new(sections[2], sections[3]);
        return Ok((elf_1_sections, elf_2_sections));
    }

    fn contains_section(&self, section: usize) -> bool {
        return section >= self.section_start && section <= self.section_end;
    }

    fn contains_assignment(&self, assignment: Assignment) -> bool {
        return self.section_start <= assignment.section_start
            && self.section_end >= assignment.section_end;
    }

    fn overlaps_assignment(&self, assignment: Assignment) -> bool {
        return self.contains_assignment(assignment)
            || assignment.contains_section(self.section_start)
            || assignment.contains_section(self.section_end);
    }
}

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(8);
    let mut total_containments = 0;

    while reader.read_line(&mut line)? != 0 {
        let (assignment_1, assignment_2) = Assignment::from_line(line.trim())?;

        if assignment_1.contains_assignment(assignment_2)
            || assignment_2.contains_assignment(assignment_1)
        {
            total_containments += 1;
        }
        line.clear();
    }
    return Ok(total_containments);
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(8);
    let mut total_overlaps = 0;

    while reader.read_line(&mut line)? != 0 {
        let (assignment_1, assignment_2) = Assignment::from_line(line.trim())?;

        if assignment_1.overlaps_assignment(assignment_2) {
            total_overlaps += 1;
        }
        line.clear();
    }
    return Ok(total_overlaps);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Fully contained assignments: {part_1}");
    println!("Part 2 - Overlapping assignments: {part_2}");
    return Ok(());
}
