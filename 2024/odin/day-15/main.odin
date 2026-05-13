package main

import "core:fmt"
import "core:os"
import "core:slice"
import "core:strings"
import "core:time"

Tile :: enum(u8) {
	None,
	Wall,
	Box,
	Box_Left,
	Box_Right,
}

Warehouse :: struct {
	tiles: []Tile,
	width: int,
	height: int,
}

init_warehouse :: proc(input: string) -> (warehouse: Warehouse, robot_pos: [2]int) {
	width  := strings.index_rune(input, '\n')
	height := len(input) / (width + 1)
	tiles  := make([]Tile, width * height)

	for y in 0..<height {
		for x in 0..<width {
			tile       := &tiles[y *  width      + x]
			input_char :=  input[y * (width + 1) + x]

			switch input_char {
			case 'O':
				tile^ = .Box
			case '#':
				tile^ = .Wall
			case '@':
				robot_pos = {x, y}
				fallthrough
			case:
				tile^ = .None
			}
		}
	}

	warehouse = {tiles, width, height}

	return warehouse, robot_pos
}

init_warehouse_wide :: proc(input: string) -> (warehouse: Warehouse, robot_pos: [2]int) {
	width  := strings.index_rune(input, '\n')
	height := len(input) / (width + 1)
	width_wide := width * 2
	tiles  := make([]Tile, width_wide * height)

	for y in 0..<height {
		for x in 0..<width {
			tile_left  := &tiles[(y * width_wide) + (x * 2 + 0)]
			tile_right := &tiles[(y * width_wide) + (x * 2 + 1)]
			input_char :=  input[y * (width + 1) + x]

			switch input_char {
			case 'O':
				tile_left^  = .Box_Left
				tile_right^ = .Box_Right
			case '#':
				tile_left^  = .Wall
				tile_right^ = .Wall
			case '@':
				robot_pos = {x * 2, y}
				fallthrough
			case:
				tile_left^  = .None
				tile_right^ = .None
			}
		}
	}

	warehouse = {tiles, width_wide, height}

	return warehouse, robot_pos
}

delete_warehouse :: proc(warehouse: Warehouse) {
	delete(warehouse.tiles)
}

coord_to_index :: proc(warehouse: Warehouse, coord: [2]int) -> int {
	return coord.y * warehouse.width + coord.x
}

get_tile :: proc(warehouse: Warehouse, coord: [2]int) -> Tile {
	index := coord_to_index(warehouse, coord)
	tile  := warehouse.tiles[index]

	return tile
}

sum_gps_coords :: proc(warehouse: Warehouse) -> int {
	gps_sum := 0

	for y in 0..<warehouse.height {
		for x in 0..<warehouse.width {
			tile := get_tile(warehouse, {x, y})
			if tile == .Box || tile == .Box_Left {
				gps_sum += y * 100 + x
			}
		}
	}

	return gps_sum
}

load_input :: proc(path: string) -> string {
	input_bytes, ok := os.read_entire_file(path)
	if !ok {
		fmt.eprintln("couldn't read input file")
		os.exit(2)
	}

	return string(input_bytes)
}

print_solution :: proc(name, input: string, solution_proc: proc(string) -> int) {
	start_tk := time.tick_now()
	solution := solution_proc(input)
	duration := time.tick_since(start_tk)
	millis   := time.duration_milliseconds(duration)

	fmt.printfln("%v: %v, time: %.4fms", name, solution, millis)
}

part_1 :: proc(input: string) -> int {
	split     := strings.index(input, "\n\n") + 1
	movements := input[split + 1:]
	warehouse, robot_pos := init_warehouse(input[:split])

	defer delete_warehouse(warehouse)

	loop: for m in movements {
		// read the direction in which the robot should move
		dir: [2]int

		switch m {
		case '^': dir = {0, -1}
		case 'v': dir = {0, +1}
		case '>': dir = {+1, 0}
		case '<': dir = {-1, 0}
		case: continue
		}

		// find the number of steps, in the movement direction, to the nearest empty space (if any)
		steps := 1

		for {
			next_pos  := dir * steps + robot_pos
			next_tile := get_tile(warehouse, next_pos)

			if next_tile == .None {
				break
			} else if next_tile == .Wall {
				continue loop // no space to move, skip to next movement instruction
			}

			steps += 1
		}

		// move everything between the robot and the space one step in the movement direction
		for j in 0..<steps {
			pos_1 := dir * (steps - j) + robot_pos
			pos_2 := pos_1 - dir

			index_1 := coord_to_index(warehouse, pos_1)
			index_2 := coord_to_index(warehouse, pos_2)

			slice.swap(warehouse.tiles, index_1, index_2)
		}

		robot_pos += dir
	}

	return sum_gps_coords(warehouse)
}

part_2 :: proc(input: string) -> int {
	split     := strings.index(input, "\n\n") + 1
	movements := input[split + 1:]
	warehouse, robot_pos := init_warehouse_wide(input[:split])

	defer delete_warehouse(warehouse)

	loop: for m in movements {
		// read the direction in which the robot should move
		dir: [2]int

		switch m {
		case '^': dir = {0, -1}
		case 'v': dir = {0, +1}
		case '>': dir = {+1, 0}
		case '<': dir = {-1, 0}
		case: continue
		}

		// if the direction faces left or right, move boxes as in part 1

		// otherwise, use a breadth-first search to find all the boxes that touch each
		// other and shift them all up if all boxes have an empty space in front of them, i.e:

		// for each item in the current level of a stack (remember to record the
		// number of tiles in this level of the stack), look at the tile above/below it.

		// if the tile contains the left/right side of a box, set a bool to record that there
		// was an obstruction in this iteration and push the box tile to the stack. Then:

			// if the current tile is the robot (occurs on the first iteration), and the tile
			// above/below is the side of a box, push the other side of the box to the stack too.

			// otherwise:

				// if the current tile is the left side of a box and the next tile is the
				// right side of a box, push the left side of the next box to the stack too.

				// otherwise, if the current tile is the right side of a box and the next tile is
				// the left side of a box, push the right side of the next box to the stack too.

		// otherwise, if the tile contains a wall, skip this movement instruction

		// if the whole line of boxes has no obstruction, work backwards
		// through the stack, swapping each tile with the one above/below it.
	}

	// print warehouse
	for y in 0..<warehouse.height {
		for x in 0..<warehouse.width {
			pos := [2]int{x, y}

			if pos == robot_pos {
				fmt.print('@')
				continue
			}

			tile := get_tile(warehouse, pos)

			#partial switch tile {
			case .Box_Left:  fmt.print('[')
			case .Box_Right: fmt.print(']')
			case .Wall:      fmt.print('#')
			case .None:      fmt.print('.')
			}
		}

		fmt.println()
	}

	return sum_gps_coords(warehouse)
}

main :: proc() {
	// input := load_input("input.txt")
	// input := load_input("example_1.txt")
	input := load_input("example_2.txt")

	print_solution("part 1", input, part_1)
	print_solution("part 2", input, part_2)
}
