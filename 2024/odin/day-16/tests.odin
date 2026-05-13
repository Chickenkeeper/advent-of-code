package main

import "core:testing"

@(test)
test_part_1_small :: proc(t: ^testing.T) {
	input := load_input("example_1.txt")
	testing.expect_value(t, part_1(input), 7036)
}

@(test)
test_part_1_large :: proc(t: ^testing.T) {
	input := load_input("example_1.txt")
	testing.expect_value(t, part_1(input), 11048)
}

@(test)
test_part_2_small :: proc(t: ^testing.T) {
	input := load_input("example_1.txt")
	testing.expect_value(t, part_1(input), 45)
}

@(test)
test_part_2_large :: proc(t: ^testing.T) {
	input := load_input("example_1.txt")
	testing.expect_value(t, part_1(input), 64)
}
