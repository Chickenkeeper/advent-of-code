use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

struct TreeGrid {
    heights: Vec<u8>,
    width: usize,
    height: usize,
}

impl TreeGrid {
    fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path).map_err(|e| format!("Error opening {path:?}: {e}"))?;
        let reader = BufReader::new(file);
        let mut heights = Vec::new();
        let mut width = 0;
        let mut width_counter = 0;

        for b in reader.bytes() {
            let byte = b?;

            if byte == b'\n' {
                if width == 0 {
                    width = width_counter;
                } else if width != width_counter {
                    return Result::Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "All lines must be the same length",
                    ))?;
                }
                width_counter = 0;
            } else if byte >= b'0' && byte <= b'9' {
                heights.push(byte - b'0');
                width_counter += 1;
            } else {
                return Result::Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Characters must either be ASCII digits or newline",
                ))?;
            }
        }

        let height = heights.len() / width;

        return Ok(TreeGrid {
            heights,
            width,
            height,
        });
    }
}

fn solution_part_1() -> Result<usize, Box<dyn std::error::Error>> {
    let tree_grid = TreeGrid::from_file(Path::new("input.txt"))?;
    let mut visible_trees = 0;

    for y in 0..tree_grid.height {
        for x in 0..tree_grid.width {
            let current_tree_height = tree_grid.heights[y * tree_grid.width + x];

            // Left to right
            let mut is_visible_right = true;
            for i in (x + 1)..tree_grid.width {
                if tree_grid.heights[y * tree_grid.width + i] >= current_tree_height {
                    is_visible_right = false;
                    break;
                }
            }

            // Right to left
            let mut is_visible_left = true;
            for i in (0..x).rev() {
                if tree_grid.heights[y * tree_grid.width + i] >= current_tree_height {
                    is_visible_left = false;
                    break;
                }
            }

            // Top to bottom
            let mut is_visible_bottom = true;
            for i in (y + 1)..tree_grid.height {
                if tree_grid.heights[i * tree_grid.width + x] >= current_tree_height {
                    is_visible_bottom = false;
                    break;
                }
            }

            // Bottom to top
            let mut is_visible_top = true;
            for i in (0..y).rev() {
                if tree_grid.heights[i * tree_grid.width + x] >= current_tree_height {
                    is_visible_top = false;
                    break;
                }
            }

            if is_visible_right || is_visible_left || is_visible_bottom || is_visible_top {
                visible_trees += 1;
            }
        }
    }

    return Ok(visible_trees);
}

fn solution_part_2() -> Result<usize, Box<dyn std::error::Error>> {
    let tree_grid = TreeGrid::from_file(Path::new("input.txt"))?;
    let mut max_scenic_score = 0;

    for y in 0..tree_grid.height {
        for x in 0..tree_grid.width {
            let current_tree_height = tree_grid.heights[y * tree_grid.width + x];

            // Left to right
            let mut view_dist_right = 0;
            for i in (x + 1)..tree_grid.width {
                view_dist_right += 1;

                if tree_grid.heights[y * tree_grid.width + i] >= current_tree_height {
                    break;
                }
            }

            // Right to left
            let mut view_dist_left = 0;
            for i in (0..x).rev() {
                view_dist_left += 1;

                if tree_grid.heights[y * tree_grid.width + i] >= current_tree_height {
                    break;
                }
            }

            // Top to bottom
            let mut view_dist_bottom = 0;
            for i in (y + 1)..tree_grid.height {
                view_dist_bottom += 1;

                if tree_grid.heights[i * tree_grid.width + x] >= current_tree_height {
                    break;
                }
            }

            // Bottom to top
            let mut view_dist_top = 0;
            for i in (0..y).rev() {
                view_dist_top += 1;

                if tree_grid.heights[i * tree_grid.width + x] >= current_tree_height {
                    break;
                }
            }

            let tree_scenic_score =
                view_dist_right * view_dist_left * view_dist_bottom * view_dist_top;
            if tree_scenic_score > max_scenic_score {
                max_scenic_score = tree_scenic_score;
            }
        }
    }

    return Ok(max_scenic_score);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = solution_part_1()?;
    let part_2 = solution_part_2()?;

    println!("Part 1 - Total Visible Trees: {part_1}");
    println!("Part 2 - Highest Scenic Score: {part_2}");
    return Ok(());
}
