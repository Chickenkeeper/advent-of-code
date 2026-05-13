package main

import "core:testing"

@(test)
test_part_1_small :: proc(t: ^testing.T) {
	input := load_input("example_1.txt")
	defer delete(input)

	testing.expect_value(t, part_1(input), 2028)
}

@(test)
test_part_1_large :: proc(t: ^testing.T) {
	input := load_input("example_2.txt")
	defer delete(input)

	testing.expect_value(t, part_1(input), 10092)
}

@(test)
test_part_2 :: proc(t: ^testing.T) {
	input := load_input("example_1.txt")
	defer delete(input)

	testing.expect_value(t, part_2(input), 9021)
}
