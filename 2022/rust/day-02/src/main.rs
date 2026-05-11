use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn from_shape_and_outcome(shape: Shape, target_outcome: Outcome) -> Shape {
        match (shape, target_outcome) {
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
        }
    }
}

impl FromStr for Shape {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Shape character must only be A, B, C, X, Y, or Z",
            )),
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    fn from_shapes(player_shape: Shape, opponent_shape: Shape) -> Outcome {
        match (opponent_shape, player_shape) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Win,
            (Shape::Rock, Shape::Scissors) => Outcome::Lose,
            (Shape::Paper, Shape::Rock) => Outcome::Lose,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Win,
            (Shape::Scissors, Shape::Rock) => Outcome::Win,
            (Shape::Scissors, Shape::Paper) => Outcome::Lose,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }
}

impl FromStr for Outcome {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Outcome character must only by X, Y, or Z",
            )),
        }
    }
}

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(3);
    let mut score = 0;

    while reader.read_line(&mut line)? != 0 {
        let mut shapes = line.split_whitespace();
        let opponent_shape = Shape::from_str(shapes.next().unwrap_or_default())?;
        let player_shape = Shape::from_str(shapes.next().unwrap_or_default())?;
        let round_outcome = Outcome::from_shapes(player_shape, opponent_shape);

        score += player_shape as usize;
        score += round_outcome as usize;
        line.clear();
    }

    return Ok(score);
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e:?}"))?;
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(3);
    let mut score = 0;

    while reader.read_line(&mut line)? != 0 {
        let mut shapes = line.split_whitespace();
        let opponent_shape = Shape::from_str(shapes.next().unwrap_or_default())?;
        let target_outcome = Outcome::from_str(shapes.next().unwrap_or_default())?;
        let target_player_shape = Shape::from_shape_and_outcome(opponent_shape, target_outcome);

        score += target_outcome as usize;
        score += target_player_shape as usize;
        line.clear();
    }

    return Ok(score);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Total score: {part_1}");
    println!("Part 2 - Total score: {part_2}");
    return Ok(());
}
