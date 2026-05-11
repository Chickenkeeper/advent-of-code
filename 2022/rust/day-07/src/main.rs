use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

struct Directory {
    parent: Option<Rc<RefCell<Directory>>>,
    sub_dirs: HashMap<String, Rc<RefCell<Directory>>>,
    files: HashMap<String, usize>,
}

impl Directory {
    fn from_file(path: &Path) -> Result<Rc<RefCell<Directory>>, Box<dyn std::error::Error>> {
        let file = File::open(path).map_err(|e| format!("Error opening {path:?}: {e:?}"))?;
        let mut reader = BufReader::new(file);
        let mut line = String::with_capacity(8);
        let root = Rc::new(RefCell::new(Directory {
            parent: None,
            sub_dirs: HashMap::new(),
            files: HashMap::new(),
        }));
        let mut current_dir = root.clone();

        while reader.read_line(&mut line)? != 0 {
            if line.starts_with("$ ") {
                let command = line.trim_start_matches("$ ").trim();

                if command.starts_with("cd ") {
                    let dir_name = command.trim_start_matches("cd ");

                    if dir_name == ".." {
                        current_dir = match current_dir.clone().borrow().parent.clone() {
                            Some(d) => Ok(d.clone()),
                            None => Err(std::io::Error::new(
                                std::io::ErrorKind::NotFound,
                                format!("Directory not found"),
                            )),
                        }?;
                    } else if dir_name == "/" {
                        current_dir = root.clone();
                    } else {
                        current_dir = match current_dir.clone().borrow().sub_dirs.get(dir_name) {
                            Some(d) => Ok(d.clone()),
                            None => Err(std::io::Error::new(
                                std::io::ErrorKind::NotFound,
                                format!("Directory not found"),
                            )),
                        }?;
                    }
                } else if command != "ls" {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Command not recognised"),
                    ))?;
                }
            } else if line.starts_with("dir ") {
                let dir_name = line.trim_start_matches("dir ").trim();

                if !current_dir.borrow().sub_dirs.contains_key(dir_name) {
                    let new_dir = Rc::new(RefCell::new(Directory {
                        parent: Some(current_dir.clone()),
                        sub_dirs: HashMap::new(),
                        files: HashMap::new(),
                    }));
                    current_dir
                        .borrow_mut()
                        .sub_dirs
                        .insert(dir_name.to_string(), new_dir);
                }
            } else {
                let mut file_line = line.split_whitespace();
                let size = file_line
                    .next()
                    .ok_or_else(|| "Couldn't parse file")?
                    .parse::<usize>()?;
                let name = file_line.next().ok_or_else(|| "Couldn't parse file")?;

                current_dir
                    .borrow_mut()
                    .files
                    .insert(name.to_string(), size);
            }

            line.clear();
        }

        return Ok(root.clone());
    }

    fn get_size(&self) -> usize {
        let mut size = 0;

        for file in self.files.iter() {
            size += file.1;
        }

        return size;
    }

    fn get_total_size(&self) -> usize {
        let mut size = self.get_size();

        for dir in self.sub_dirs.iter() {
            size += dir.1.borrow().get_total_size();
        }

        return size;
    }
}

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    fn tally_sub_dir_sizes(root_dir: Rc<RefCell<Directory>>) -> usize {
        let mut size = 0;

        let root_size = root_dir.borrow().get_total_size();
        if root_size <= 100000 {
            size += root_size;
        }

        for dir in root_dir.borrow().sub_dirs.iter() {
            size += tally_sub_dir_sizes(dir.1.clone());
        }

        return size;
    }

    let file_system = Directory::from_file(Path::new("input.txt"))?;
    let size = tally_sub_dir_sizes(file_system.clone());

    return Ok(size);
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    fn find_smallest_deletable_directory(
        root_dir: Rc<RefCell<Directory>>,
        min_size: usize,
        size: Rc<RefCell<usize>>,
    ) -> usize {
        let root_size = root_dir.borrow().get_total_size();

        if root_size <= *size.borrow() && root_size >= min_size {
            *size.clone().borrow_mut() = root_size;
        }

        for dir in root_dir.borrow().sub_dirs.iter() {
            find_smallest_deletable_directory(dir.1.clone(), min_size, size.clone());
        }

        return *size.borrow();
    }

    let file_system = Directory::from_file(Path::new("input.txt"))?;
    let total_used_space = file_system.borrow().get_total_size();
    let min_size = 30000000 - (70000000 - total_used_space);
    let size = Rc::new(RefCell::new(70000000));
    let size = find_smallest_deletable_directory(file_system.clone(), min_size, size.clone());

    return Ok(size);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Total directory size sum: {part_1}");
    println!("Part 2 - Smallest deletable directory size: {part_2}");
    return Ok(());
}
