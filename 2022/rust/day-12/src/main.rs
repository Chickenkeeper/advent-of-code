use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

struct Pathfinder {
    elevations: Vec<u8>,
    connectivity: HashMap<Point, Point>,
    path_buffer: Vec<Point>,
    goal: Point,
    width: isize,
    height: isize,
}

impl Pathfinder {
    fn from_file(path: &Path) -> Result<(Self, Point, Point), Box<dyn std::error::Error>> {
        let file = File::open(path).map_err(|e| format!("Error opening input.txt: {e:?}"))?;
        let mut reader = BufReader::new(file);
        let mut line_buffer = String::with_capacity(64);
        let mut elevations = Vec::new();
        let mut start_pos = Point::new(0, 0);
        let mut end_pos = Point::new(0, 0);
        let mut width = 0;
        let mut height = 0;

        while reader.read_line(&mut line_buffer)? != 0 {
            if width == 0 {
                width = (line_buffer.len() - 1) as isize;
            }

            for (x, byte) in line_buffer.bytes().enumerate() {
                if byte >= b'a' && byte <= b'z' {
                    elevations.push(byte - b'a');
                } else if byte == b'S' {
                    elevations.push(0);
                    start_pos = Point::new(x as isize, height);
                } else if byte == b'E' {
                    elevations.push(25);
                    end_pos = Point::new(x as isize, height);
                }
            }

            height += 1;
            line_buffer.clear();
        }

        return Ok((
            Pathfinder {
                elevations,
                connectivity: HashMap::<Point, Point>::new(),
                path_buffer: Vec::<Point>::new(),
                goal: Point::new(0, 0),
                width,
                height,
            },
            start_pos,
            end_pos,
        ));
    }

    fn get_elevation(&self, pos: Point) -> Option<u8> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            return Some(self.elevations[(pos.y * self.width + pos.x) as usize]);
        } else {
            return None;
        }
    }

    fn print(&self, screen: &mut String) {
        for (y, r) in self
            .elevations
            .chunks_exact(self.width as usize)
            .enumerate()
        {
            for (x, b) in r.iter().enumerate() {
                let curr = Point::new(x as isize, y as isize);

                if self.path_buffer.contains(&curr) || curr == self.goal {
                    screen.push_str("\u{1b}[7m"); // invert colours
                } else {
                    screen.push_str("\u{1b}[27m"); // uninvert colours
                }

                if curr == self.goal {
                    screen.push('E');
                } else if let Some(from) = self.connectivity.get(&curr) {
                    if from.y == curr.y - 1 {
                        screen.push('^');
                    } else if from.x == curr.x + 1 {
                        screen.push('>');
                    } else if from.y == curr.y + 1 {
                        screen.push('v');
                    } else {
                        screen.push('<');
                    }
                } else {
                    screen.push((*b + b'a') as char);
                }
            }
            screen.push('\n');
        }

        print!("\u{1b}[0;0H{screen}\u{1b}[27m"); // move cursor to 0, 0, print data, and uninvert colours
        screen.clear();
        // std::thread::sleep(std::time::Duration::from_millis(100));
    }

    fn set_goal(&mut self, goal: Point) {
        let mut frontier = VecDeque::new();
        let mut screen = String::new();

        self.goal = goal;
        frontier.push_back(goal);

        while let Some(curr_pos) = frontier.pop_front() {
            let max_elev = self.get_elevation(curr_pos).unwrap().saturating_sub(1);

            let north = Point::new(curr_pos.x, curr_pos.y - 1);
            if let Some(next_elev) = self.get_elevation(north) {
                if !self.connectivity.contains_key(&north) && next_elev >= max_elev {
                    frontier.push_back(north);
                    self.connectivity.insert(north, curr_pos);
                }
            }

            let east = Point::new(curr_pos.x + 1, curr_pos.y);
            if let Some(next_elev) = self.get_elevation(east) {
                if !self.connectivity.contains_key(&east) && next_elev >= max_elev {
                    frontier.push_back(east);
                    self.connectivity.insert(east, curr_pos);
                }
            }

            let south = Point::new(curr_pos.x, curr_pos.y + 1);
            if let Some(next_elev) = self.get_elevation(south) {
                if !self.connectivity.contains_key(&south) && next_elev >= max_elev {
                    frontier.push_back(south);
                    self.connectivity.insert(south, curr_pos);
                }
            }

            let west = Point::new(curr_pos.x - 1, curr_pos.y);
            if let Some(next_elev) = self.get_elevation(west) {
                if !self.connectivity.contains_key(&west) && next_elev >= max_elev {
                    frontier.push_back(west);
                    self.connectivity.insert(west, curr_pos);
                }
            }

            self.print(&mut screen);
            println!("\nNodes: {}\n", self.connectivity.len());
        }
    }

    fn get_length_to_goal(&mut self, start_pos: Point) -> Option<usize> {
        if self.connectivity.is_empty() {
            return None;
        }

        let mut curr_pos = start_pos;
        let mut screen = String::new();

        self.path_buffer.clear();
        while curr_pos != self.goal {
            self.path_buffer.push(curr_pos);

            if let Some(from_pos) = self.connectivity.get(&curr_pos) {
                curr_pos = *from_pos;
            } else {
                return None;
            }

            self.print(&mut screen);
            println!("\nPath Length: {}\n", self.path_buffer.len());
        }

        return Some(self.path_buffer.len());
    }
}

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    print!("\u{1b}[2J"); // clear screen
    print!("\u{1b}[?25l"); // hide cursor

    let (mut heightmap, start_pos, end_pos) = Pathfinder::from_file(Path::new("input.txt"))?;
    heightmap.set_goal(end_pos);
    let path_length = heightmap.get_length_to_goal(start_pos).unwrap();

    print!("\u{1b}[?25h"); // show cursor
    return Ok(path_length);
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    print!("\u{1b}[2J"); // clear screen
    print!("\u{1b}[?25l"); // hide cursor

    let mut min_length = usize::MAX;
    let (mut heightmap, _, end_pos) = Pathfinder::from_file(Path::new("input.txt"))?;
    heightmap.set_goal(end_pos);

    for y in 0..(heightmap.height as usize) {
        for x in 0..(heightmap.width as usize) {
            let curr_pos = Point::new(x as isize, y as isize);

            if let Some(height) = heightmap.get_elevation(curr_pos) {
                if height == 0 {
                    if let Some(length) = heightmap.get_length_to_goal(curr_pos) {
                        min_length = min_length.min(length);
                    }
                }
            }
        }
    }

    print!("\u{1b}[?25h"); // show cursor
    return Ok(min_length);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    // std::thread::sleep(std::time::Duration::from_millis(2000));
    let part_2 = solution_part_2()?;

    println!("Part 1 - Shortest Path Length: {part_1}");
    println!("Part 2 - Shortest Path Length: {part_2}");
    return Ok(());
}
