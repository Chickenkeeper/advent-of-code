package main

import "core:fmt"
import "core:os"
import "core:strconv"
import "core:strings"

load_input :: proc(path: string) -> string {
	input_bytes, ok := os.read_entire_file(path)
	if !ok {
		fmt.eprintln("couldn't read input file")
		os.exit(int(os.ERROR_FILE_NOT_FOUND))
	}

	return string(input_bytes)
}

hash :: proc(n: int) -> int {
	n := n

	n ~= n << 6
	n &= 0xFFFFFF
	n ~= n >> 5
	n &= 0xFFFFFF
	n ~= n << 11
	n &= 0xFFFFFF

	return n
}

part_1 :: proc(input: string) -> int {
	input := input
	sum   := 0

	for line in strings.split_lines_iterator(&input) {
		secret_num, _ := strconv.parse_int(line)

		for _ in 0..<2000 {
			secret_num = hash(secret_num)
		}

		sum += secret_num
	}

	return sum
}

part_2 :: proc(input: string) -> int {
	return 0
}

main :: proc() {
	input := load_input("input.txt")
	// input := load_input("example.txt")

	fmt.printfln("part 1: %v", part_1(input))
	fmt.printfln("part 2: %v", part_2(input))
}
