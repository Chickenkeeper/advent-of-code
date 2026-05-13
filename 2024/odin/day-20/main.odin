package main

import "core:fmt"
import "core:os"
import "core:slice"
import "core:strings"

load_input :: proc(path: string) -> string {
	input_bytes, ok := os.read_entire_file(path)
	if !ok {
		fmt.eprintln("couldn't read input file")
		os.exit(int(os.ERROR_FILE_NOT_FOUND))
	}

	return string(input_bytes)
}

solve_cheats :: proc(input: string, cheat_duration, min_picosecs: int) -> int {
	width  := strings.index_rune(input, '\n') + 1
	height := len(input) / width
	dists  := make([]int, width * height)
	path   := make([dynamic][2]int)

	defer {
		delete(dists)
		delete(path)
	}

	slice.fill(dists, max(int))

	// find start and end positions
	start, end: [2]int

	outer: for y in 0..<height {
		for x in 0..<width {
			switch input[y * width + x] {
			case 'S': start = {x, y}
			case 'E': end   = {x, y}
			case:
				if start != {} && end != {} {
					break outer
				}
			}
		}
	}

	// find the path and record the distance from the end at each point
	pos := end

	for dist in 0..<width * height {
		@(static, rodata)
		directions := [4][2]int{
			{0, -1}, // north
			{0, +1}, // south
			{+1, 0}, // east
			{-1, 0}, // west
		}

		dists[pos.y * width + pos.x] = dist
		append(&path, pos)

		if pos == start {
			break
		}

		for dir in directions {
			next_pos := pos + dir

			if next_pos.x <= 0 ||
			   next_pos.y <= 0 ||
			   next_pos.x >= width - 2 ||
			   next_pos.y >= height - 1 {
				continue
			}

			index := next_pos.y * width + next_pos.x

			if input[index] == '#' ||
			   dists[index] != max(int) {
				continue
			}

			pos = next_pos
			break
		}
	}

	// walk along the path and search for shortcuts
	total_cheats := 0
	path_len     := len(path) - 1

	for dist_from_start in 0..<path_len {
		pos = path[path_len - dist_from_start]

		for y in -cheat_duration..=cheat_duration {
			for x in -cheat_duration..=cheat_duration {
				cheat_dist := abs(x) + abs(y)

				if cheat_dist == 0 ||
				   cheat_dist > cheat_duration {
					continue
				}

				cheat_pos := pos + {x, y}

				if cheat_pos.x <= 0 ||
				   cheat_pos.y <= 0 ||
				   cheat_pos.x >= width - 2 ||
				   cheat_pos.y >= height - 1 {
					continue
			 	}

				dist_from_end := dists[cheat_pos.y * width + cheat_pos.x]

				if dist_from_end == max(int) {
					continue
				}

				total_dist := dist_from_start + cheat_dist + dist_from_end
				time_saved := path_len - total_dist

				if time_saved >= min_picosecs {
					total_cheats += 1
				}
			}
		}
	}

	return total_cheats
}

part_1 :: proc(input: string, min_picosecs: int) -> int {
	return solve_cheats(input, 2, min_picosecs)
}

part_2 :: proc(input: string, min_picosecs: int) -> int {
	return solve_cheats(input, 20, min_picosecs)
}

main :: proc() {
	input := load_input("input.txt")

	fmt.printfln("part 1: %v", part_1(input, 100))
	fmt.printfln("part 2: %v", part_2(input, 100))
}
