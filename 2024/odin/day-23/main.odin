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

count_triples :: proc(branch []int, tree: [][dynamic]int, depth: int) -> int {
	for i in branch {

	}
}

part_1 :: proc(input: string) -> int {
	input_cpy := input
	lookup    := make(map[[2]byte]int, context.temp_allocator)
	counter   := 0

	// create lookup table
	for line in strings.split_lines_iterator(&input_cpy) {
		left  := [2]byte{line[0], line[1]}
		right := [2]byte{line[3], line[4]}

		if !(left in lookup) {
			lookup[left]  = left[0]  == 't' ? - counter : counter
			counter += 1
		}

		if !(right in lookup) {
			lookup[right] = right[0] == 't' ? - counter : counter
			counter += 1
		}
	}

	// create tree
	tree := make([][dynamic]int, len(lookup))

	for i in 0..<len(tree) {
		tree[i] = make([dynamic]int, context.temp_allocator)
	}

	input_cpy = input

	// fill tree
	for line in strings.split_lines_iterator(&input_cpy) {
		left  := [2]byte{line[0], line[1]}
		right := [2]byte{line[3], line[4]}

		left_index  := abs(lookup[left])
		right_index := abs(lookup[right])

		if !slice.contains(tree[left_index][:], right_index) {
			append(&tree[left_index], right_index)
		}

		if !slice.contains(tree[right_index][:], left_index) {
			append(&tree[right_index], left_index)
		}
	}

	// find triplets
	total := count_triples(&tree, 0)

	free_all(context.temp_allocator)
	delete(lookup)
	delete(tree)

	return total
}

part_2 :: proc(input: string) -> int {
	return 0
}

main :: proc() {
	input := load_input("example.txt")
	// input := load_input("input.txt")

	fmt.printfln("part 1: %v", part_1(input))
	fmt.printfln("part 2: %v", part_2(input))
}
