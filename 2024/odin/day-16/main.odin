package main

import "core:fmt"
import "core:os"
import "core:strings"

load_input :: proc(path: string) -> string {
	input_bytes, ok := os.read_entire_file(path)
	if !ok {
		fmt.eprintln("couldn't read input file")
		os.exit(int(os.ERROR_FILE_NOT_FOUND))
	}

	return string(input_bytes)
}

Direction :: enum {
	North,
	East,
	South,
	West,
}

Visit :: struct {
	pos: [2]int,
	dir: Direction,
}

Trail :: struct {
	pos:   [2]int,
	dir:   Direction,
	score: int,
}

part_1 :: proc(input: string) -> int {
	width   := strings.index_rune(input, '\n') + 1 // width includes newline
	height  := len(input) / width
	stack   := make([dynamic]Trail)
	visited := make(map[Visit]int)

	start     := [2]int{1, height - 2}
	min_score := max(int)

	append(&stack, Trail{start, .East, 0})

	for len(stack) > 0 {
		@(static, rodata)
		directions := [Direction][2]int {
			.North = {0, -1},
			.South = {0, +1},
			.East  = {+1, 0},
			.West  = {-1, 0},
		}

		curr_trail := pop(&stack)

		if curr_trail.score >= min_score {
			continue
		}

		v, ok := visited[{curr_trail.pos, curr_trail.dir}]
		if ok && v <= curr_trail.score {
			continue
		}

		visited[{curr_trail.pos, curr_trail.dir}] = curr_trail.score

		dir_vec := directions[curr_trail.dir]

		for {
			curr_tile := input[curr_trail.pos.y * width + curr_trail.pos.x]

			if curr_tile == '#' {
				break
			} else if curr_tile == 'E' {
				min_score = min(min_score, curr_trail.score)
				break
			}

			// split the path if there's space either side
			left_turn_dir:  Direction
			right_turn_dir: Direction

			switch curr_trail.dir {
			case .North:
				left_turn_dir = .West
				right_turn_dir = .East
			case .South:
				left_turn_dir = .East
				right_turn_dir = .West
			case .East:
				left_turn_dir = .North
				right_turn_dir = .South
			case .West:
				left_turn_dir = .South
				right_turn_dir = .North
			}

			left_pos  := curr_trail.pos + directions[left_turn_dir]
			right_pos := curr_trail.pos + directions[right_turn_dir]

			if input[left_pos.y * width + left_pos.x] != '#' {
				append(&stack, Trail{left_pos, left_turn_dir, curr_trail.score + 1001})
			}

			if input[right_pos.y * width + right_pos.x] != '#' {
				append(&stack, Trail{right_pos, right_turn_dir, curr_trail.score + 1001})
			}

			// continue walking along the path until a wall is found
			curr_trail.pos   += dir_vec
			curr_trail.score += 1
		}
	}

	return min_score
}

part_2 :: proc(input: string) -> int {
	best_path_len := 37
	width      := strings.index_rune(input, '\n') + 1 // width includes newline
	height     := len(input) / width
	visited    := make([][Direction]bool, width * height)
	stack      := make([dynamic]Visit, 0, best_path_len)
	best_tiles := make([]bool, width * height)

	start := [2]int{1, height - 2}
	append(&stack, Visit{start, .East})

	for len(stack) > 0 {
		@(static, rodata)
		directions := [Direction][2]int {
			.North = {0, -1},
			.East  = {+1, 0},
			.South = {0, +1},
			.West  = {-1, 0},
		}

		// fmt.println(len(stack))

		curr       := stack[len(stack) - 1]
		curr_index := curr.pos.y * width + curr.pos.x

		visited[curr_index][curr.dir] = true

		if len(stack) == best_path_len {
			if input[curr_index] == 'E' {
				for t in stack {
					best_tiles[t.pos.y * width + t.pos.x] = true
				}
			}

			pop(&stack)
			continue
		}

		// west
		west_dir   := Direction((uint(curr.dir) - 1) % 4)
		west_pos   := curr.pos + directions[west_dir]
		west_index := west_pos.y * width + west_pos.x

		if input[west_index] != '#' && !visited[west_index][west_dir] {
			append(&stack, Visit{west_pos, west_dir})
			continue
		}

		// north
		north_dir   := curr.dir
		north_pos   := curr.pos + directions[north_dir]
		north_index := north_pos.y * width + north_pos.x

		if input[north_index] != '#' && !visited[north_index][north_dir] {
			append(&stack, Visit{north_pos, north_dir})
			continue
		}

		// east
		east_dir   := Direction((uint(curr.dir) + 1) % 4)
		east_pos   := curr.pos + directions[east_dir]
		east_index := east_pos.y * width + east_pos.x

		if input[east_index] != '#' && !visited[east_index][east_dir] {
			append(&stack, Visit{east_pos, east_dir})
			continue
		}

		pop(&stack)
	}

	num_tiles := 0

	for t in best_tiles {
		if t {
			num_tiles += 1
		}
	}

	return num_tiles
}

main :: proc() {
	// input := load_input("input.txt")
	input := load_input("example_1.txt")
	// input := load_input("example_2.txt")

	fmt.printfln("part 1: %v", part_1(input))
	fmt.printfln("part 2: %v", part_2(input))
}
