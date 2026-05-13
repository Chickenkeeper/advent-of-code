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

get_combos :: proc(design: string, patterns: []string, memo: ^map[string]int) -> int {
	if design in memo {
		return memo[design]
	} else {
		memo[design] = 0
	}

	for pattern in patterns {
		if design == pattern {
			memo[design] += 1
		} else if strings.starts_with(design, pattern) {
			memo[design] += get_combos(design[len(pattern):], patterns, memo)
		}
	}

	return memo[design]
}

part_1 :: proc(input: string) -> int {
	split := strings.index_rune(input, '\n')
	patterns_str := input[:split]
	designs_str  := input[split + 2:]
	patterns := make([dynamic]string)

	for pattern in strings.split_iterator(&patterns_str, ", ") {
		append(&patterns, pattern)
	}

	possible_designs := 0
	memo := make(map[string]int)

	loop: for design in strings.split_lines_iterator(&designs_str) {
		if get_combos(design, patterns[:], &memo) > 0 {
			possible_designs += 1
		}
	}

	return possible_designs
}

part_2 :: proc(input: string) -> int {
	split := strings.index_rune(input, '\n')
	patterns_str := input[:split]
	designs_str  := input[split + 2:]
	patterns := make([dynamic]string)

	for pattern in strings.split_iterator(&patterns_str, ", ") {
		append(&patterns, pattern)
	}

	combos := 0
	memo := make(map[string]int)

	loop: for design in strings.split_lines_iterator(&designs_str) {
		combos += get_combos(design, patterns[:], &memo)
	}

	return combos
}

main :: proc() {
	input := load_input("input.txt")

	fmt.printfln("part 1: %v", part_1(input))
	fmt.printfln("part 2: %v", part_2(input))
}
