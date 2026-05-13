package main

import "core:fmt"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"

Direction :: enum(u8) {
	None,
	North,
	South,
	East,
	West,
}

load_input :: proc(path: string) -> string {
	input_bytes, ok := os.read_entire_file(path)
	if !ok {
		fmt.eprintln("couldn't read input file")
		os.exit(int(os.ERROR_FILE_NOT_FOUND))
	}

	return string(input_bytes)
}

part_1 :: proc(input: string, width, height, num_bytes: int) -> int {
	input     := input
	visited   := make([]bool, width * height)
	graph     := make([]Direction, width * height)
	obstacles := make([]bool, width * height)
	stack     := make([dynamic][2]int)

	// map obstacles
	i := 0
	for line in strings.split_lines_iterator(&input) {
		split := strings.index_rune(line, ',')
		x, _  := strconv.parse_int(line[:split])
		y, _  := strconv.parse_int(line[split + 1:])

		obstacles[y * width + x] = true
		i += 1

		if i == num_bytes {
			break
		}
	}

	// create the pathfinding graph using a breath-first search
	append(&stack, [2]int{width - 1, height - 1})

	for len(stack) > 0 {
		@(static, rodata)
		directions := [4][2]int{
			{0, -1},
			{0, +1},
			{+1, 0},
			{-1, 0},
		}

		coord := stack[0]
		index := coord.y * width + coord.x

		ordered_remove(&stack, 0)

		if visited[index] {
			continue
		} else {
			visited[index] = true
		}

		for dir, i in directions {
			next := coord + dir
			next_index := next.y * width + next.x

			if next.x >= 0 &&
			   next.y >= 0 &&
			   next.x < width &&
			   next.y < height &&
			   !visited[next_index] &&
			   !obstacles[next_index] {
				switch i {
				case 0: graph[next_index] = .South
				case 1: graph[next_index] = .North
				case 2: graph[next_index] = .West
				case 3: graph[next_index] = .East
				case:
				}

				append(&stack, next)
			}
		}
	}

	// record the total length of a path to the exit
	start  := [2]int{0, 0}
	end    := [2]int{width - 1, height - 1}
	length := 0

	for start != end {
		switch graph[start.y * width + start.x] {
			case .North: start.y -= 1
			case .South: start.y += 1
			case .East:  start.x += 1
			case .West:  start.x -= 1
			case .None:
		}

		length += 1
	}

	return length
}

part_2 :: proc(input: string, width, height, start: int) -> [2]int {
	input     := input
	visited   := make([]bool, width * height)
	graph     := make([]Direction, width * height)
	obstacles := make([]bool, width * height)
	stack     := make([dynamic][2]int)

	// test obstacles
	i := 0
	for line in strings.split_lines_iterator(&input) {
		split := strings.index_rune(line, ',')
		x, _  := strconv.parse_int(line[:split])
		y, _  := strconv.parse_int(line[split + 1:])

		obstacles[y * width + x] = true
		i += 1

		// we already know the first 1024 obstacles still leave a path to the
		// exit, so only start searching for impossible paths beyond this point
		if i < start {
			continue
		}

		// create the pathfinding graph using a breath-first search
		append(&stack, [2]int{width - 1, height - 1})

		for len(stack) > 0 {
			@(static, rodata)
			directions := [4][2]int{
				{0, -1},
				{0, +1},
				{+1, 0},
				{-1, 0},
			}

			coord := stack[0]
			index := coord.y * width + coord.x

			ordered_remove(&stack, 0)

			if visited[index] {
				continue
			} else {
				visited[index] = true
			}

			for dir, i in directions {
				next := coord + dir
				next_index := next.y * width + next.x

				if next.x >= 0 &&
				   next.y >= 0 &&
				   next.x < width &&
				   next.y < height &&
				   !visited[next_index] &&
				   !obstacles[next_index] {
					switch i {
					case 0: graph[next_index] = .South
					case 1: graph[next_index] = .North
					case 2: graph[next_index] = .West
					case 3: graph[next_index] = .East
					case:
					}

					append(&stack, next)
				}
			}
		}

		// if the start position has no direction, there's no path to the end
		if graph[0] == .None {
			return {x, y}
		}

		slice.fill(visited, false)
		slice.fill(graph, Direction.None)
	}

	return {-1, -1}
}

main :: proc() {
	input := load_input("input.txt")

	fmt.printfln("part 1: %v", part_1(input, 71, 71, 1024))
	fmt.printfln("part 2: %v", part_2(input, 71, 71, 1024))
}
