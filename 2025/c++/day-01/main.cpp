#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

struct rotation {
	const int direction;
	const int value;
};

vector<rotation> load_input() {
	vector<rotation> input;

	ifstream file("input");
	char     dir_char;
	int      value;

	while (file >> dir_char >> value) {
		const int direction = dir_char == 'L' ? -1 : 1;

		input.push_back(rotation {direction, value});
	}

	return input;
}

/**
 * For each rotation, simply add or subtract the value of the rotation
 * (depending on its direction) to the dial, wrapping the result with
 * a modulus. If the dial now points to 0, increment the password.
 */
int part_1(const vector<rotation> &input) {
	int dial     = 50;
	int password = 0;

	for (const auto &rotation : input) {
		const int direction = rotation.direction;
		const int value     = rotation.value;

		// value is also wrapped to avoid taking the mod of a negative number
		dial = (dial + ((value % 100) * direction) + 100) % 100;

		if (dial == 0) {
			password++;
		}
	}

	return password;
}

/**
 * For each rotation, start by dividing its value by 100 to find the
 * number of 0s hit during the rotation. This value is then multiplied
 * by 100 and subtracted from the original value to compute the remainder,
 * which is used to set the new position of the dial and to check if the
 * dial hits or passes 0 a final time at the end of the rotation.
 */
int part_2(const vector<rotation> &input) {
	int dial     = 50;
	int password = 0;

	for (const auto &rotation : input) {
		const int direction  = rotation.direction;
		const int value      = rotation.value;
		const int full_turns = value / 100;
		const int remaining  = value - (full_turns * 100);
		const int to_zero    = direction == 1 ? 100 - dial : dial;

		// record any zeros during the rotation
		password += full_turns;

		// check if the dial turns to/past zero at the end of the rotation
		if (dial > 0 && remaining >= to_zero) {
			password++;
		}

		dial = (dial + (remaining * direction) + 100) % 100;
	}

	return password;
}

int main() {
	const auto input = load_input();

	cout << "part 1: " << part_1(input) << endl;
	cout << "part 2: " << part_2(input) << endl;

	return 0;
}
