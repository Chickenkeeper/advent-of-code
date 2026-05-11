use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        return Self { x, y };
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Rope {
    knots: Vec<Point>,
    visited_tail_positions: HashSet<Point>,
}

impl Rope {
    fn new(knots: usize) -> Self {
        return Self {
            knots: vec![Point::new(0, 0); knots],
            visited_tail_positions: HashSet::<Point>::new(),
        };
    }

    fn move_head(&mut self, movement: (Point, usize)) {
        for _ in 0..movement.1 {
            let mut prev = self.knots[0];
            prev += movement.0;
            self.knots[0] = prev;

            for s in 1..self.knots.len() {
                let mut curr = self.knots[s];

                if prev.x.abs_diff(curr.x) >= 2 || prev.y.abs_diff(curr.y) >= 2 {
                    curr.x += (prev.x - curr.x).clamp(-1, 1);
                    curr.y += (prev.y - curr.y).clamp(-1, 1);
                }

                self.knots[s] = curr;
                prev = curr;
            }

            self.visited_tail_positions.insert(prev);
        }
    }
}

fn line_to_movement(line: &str) -> Result<(Point, usize), Box<dyn std::error::Error>> {
    let mut tokens = line.split_whitespace();
    let direction = match tokens.next() {
        Some("U") => Point::new(0, 1),
        Some("D") => Point::new(0, -1),
        Some("L") => Point::new(-1, 0),
        Some("R") => Point::new(1, 0),
        _ => Result::Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Directions must be U, D, L, or R",
        ))?,
    };
    let distance = match tokens.next() {
        Some(n) => n.parse::<usize>()?,
        None => Result::Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Distance must be ASCII digits",
        ))?,
    };

    return Ok((direction, distance));
}

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e}"))?;
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(3);
    let mut rope = Rope::new(2);

    while reader.read_line(&mut line)? != 0 {
        let movement = line_to_movement(line.as_str())?;
        rope.move_head(movement);
        line.clear();
    }

    return Ok(rope.visited_tail_positions.len());
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e}"))?;
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(3);
    let mut rope = Rope::new(10);

    while reader.read_line(&mut line)? != 0 {
        let movement = line_to_movement(line.as_str())?;
        rope.move_head(movement);
        line.clear();
    }

    return Ok(rope.visited_tail_positions.len());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Total Unique Tail Positions: {part_1}");
    println!("Part 2 - Total Unique Tail Positions: {part_2}");
    return Ok(());
}
