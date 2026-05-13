package main

import "core:fmt"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"

Instruction :: struct {
	opcode:  uint,
	operand: uint,
}

load_input :: proc(path: string) -> string {
	input_bytes, ok := os.read_entire_file(path)
	if !ok {
		fmt.eprintln("couldn't read input file")
		os.exit(int(os.ERROR_FILE_NOT_FOUND))
	}

	return string(input_bytes)
}

parse_registers :: proc(input: string) -> (a, b, c: uint) {
	input := input
	i := 0

	for line in strings.split_lines_iterator(&input) {
		num, _ := strconv.parse_uint(line[12:])

		switch i {
		case 0: a = num
		case 1: b = num
		case:   c = num
		}

		i += 1
	}

	return a, b, c
}

part_1 :: proc(input: string) -> string {
	input := input
	split := strings.index(input, "\n\n")

	register_str := input[:split]
	program_str  := input[split + 11:]

	a, b, c := parse_registers(register_str)
	program := make([dynamic]Instruction)
	result: strings.Builder

	strings.builder_init(&result)

	for len(program_str) > 0 {
		opcode_str , _ := strings.split_iterator(&program_str, ",")
		operand_str, _ := strings.split_iterator(&program_str, ",")

		opcode,  _ := strconv.parse_uint(opcode_str)
		operand, _ := strconv.parse_uint(operand_str)

		append(&program, Instruction{opcode, operand})
	}

	counter := uint(0)
	for counter < len(program) {
		opcode  := program[counter].opcode
		operand := program[counter].operand

		if opcode == 0 || opcode == 2 || opcode == 5 || opcode == 6 || opcode == 7 {
			switch operand {
			case 4: operand = a
			case 5: operand = b
			case 6: operand = c
			case:
			}
		}

		switch opcode {
		case 0: // adv
			a >>= operand
		case 1: // bxl
			b ~= operand
		case 2: // bst
			b = operand % 8
		case 3: // jnz
			if a != 0 {
				counter = operand
				continue
			}
		case 4: // bxc
			b ~= c
		case 5: // out
			strings.write_rune(&result, rune(operand % 8) + '0')
			strings.write_rune(&result, ',')
		case 6: // bdv
			b = a >> operand
		case 7: // cdv
			c = a >> operand
		case:
		}

		counter += 1
	}


	return strings.to_string(result)
}

part_2 :: proc(input: string) -> int {
	input := input
	split := strings.index(input, "\n\n")
	program_str := input[split + 11:]

	buff_len :: 16
	program: [buff_len]uint
	result:  [buff_len]uint

	for i in 0..<buff_len {
		val := uint(program_str[i * 2] - '0')
		program[i] = val
	}

	result_i := 0

	for i in 0..<max(int) {
		counter := uint(0)
		a := uint(i)
		b := uint(0)
		c := uint(0)

		loop: for counter < buff_len {
			opcode  := program[counter]
			operand := program[counter + 1]

			if opcode == 0 || opcode == 2 || opcode == 5 || opcode == 6 || opcode == 7 {
				switch operand {
				case 4: operand = a
				case 5: operand = b
				case 6: operand = c
				case:
				}
			}

			switch opcode {
			case 0: // adv
				a >>= operand
			case 1: // bxl
				b ~= operand
			case 2: // bst
				b = operand % 8
			case 3: // jnz
				if a != 0 {
					counter = operand
					continue loop
				}
			case 4: // bxc
				b ~= c
			case 5: // out
				val := operand % 8

				if val != program[result_i] {
					break loop
				} else {
					result[result_i] = val
					result_i += 1

					if result_i == buff_len {
						break loop
					}
				}
			case 6: // bdv
				b = a >> operand
			case 7: // cdv
				c = a >> operand
			case:
			}

			counter += 2
		}

		if i % 100_000_000 == 0 {
			fmt.println(i)
		}

		if result == program {
			return i
		}

		result = 0
		result_i = 0
	}

	return 0
}

main :: proc() {
	input := load_input("input.txt")
	// input := load_input("example_1.txt")
	// input := load_input("example_2.txt")

	fmt.printfln("part 1: %v", part_1(input))
	fmt.printfln("part 2: %v", part_2(input))
}
