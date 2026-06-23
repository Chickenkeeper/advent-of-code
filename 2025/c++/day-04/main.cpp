#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

struct diagram {
	const vector<bool> rolls;
	const int          width;
	const int          height;
};

diagram load_input() {
	vector<bool> rolls;
	int          width  = 0;
	int          height = 0;

	ifstream file("input");
	char     ch;

	while (file.get(ch)) {
		if (ch == '\n') {
			height++;
			continue;
		}

		if (height == 0) {
			width++;
		}

		rolls.push_back(ch == '@');
	}

	return diagram{rolls, width, height};
}

/**
 * Checks whether a roll in a grid is accessible by looping over its 8 neighbours and
 * counting how many are also rolls. If fewer than 4 are rolls, it returns true.
 */
bool is_accessible(const diagram &grid, const int posX, const int posY) {
	int num_adjacent_rolls = 0;

	// loop over each neighbour of this roll
	for (int offsetY = -1; offsetY <= 1; offsetY++) {
		for (int offsetX = -1; offsetX <= 1; offsetX++) {

			// skip the position we're currently at
			if (offsetX == 0 && offsetY == 0) {
				continue;
			}

			const int neighbourX = posX + offsetX;
			const int neighbourY = posY + offsetY;

			// skip if the neighbour's outside the grid
			if (neighbourX < 0 || neighbourX >= grid.width || neighbourY < 0 || neighbourY >= grid.height) {
				continue;
			}

			// check if the neighbour's a roll
			if (grid.rolls[neighbourY * grid.width + neighbourX]) {
				num_adjacent_rolls++;
			}
		}
	}

	return num_adjacent_rolls < 4;
}

/**
 * For each roll of paper within the bounds of the grid, simply count how many are accessible.
 */
int part_1(const diagram &input) {
	int num_rolls = 0;

	// loop over all positions in the grid
	for (int posY = 0; posY < input.height; posY++) {
		for (int posX = 0; posX < input.width; posX++) {

			// skip if this position isn't a roll
			if (!input.rolls[posY * input.width + posX]) {
				continue;
			}

			if (is_accessible(input, posX, posY)) {
				num_rolls++;
			}
		}
	}

	return num_rolls;
}

/**
 * Same as part 1, but if any accessible rolls are found then make
 * a copy of the grid excluding the accessible rolls, swap to the
 * copy, and continue counting until no more rolls are accessible.
 */
int part_2(const diagram &input) {
	vector<bool> rolls_curr = input.rolls;
	vector<bool> rolls_next = input.rolls;

	bool rolls_accessible = true;
	int  num_rolls        = 0;

	while (rolls_accessible) {
		rolls_accessible = false;

		// loop over all positions in the grid
		for (int posY = 0; posY < input.height; posY++) {
			for (int posX = 0; posX < input.width; posX++) {
				const int curr_index = posY * input.width + posX;

				// skip if this position isn't a roll
				if (!rolls_curr[curr_index]) {
					rolls_next[curr_index] = false;
					continue;
				}

				if (is_accessible({rolls_curr, input.width, input.height}, posX, posY)) {
					rolls_next[curr_index] = false; // remove the accessible roll
					rolls_accessible       = true;
					num_rolls++;
				} else {
					rolls_next[curr_index] = true;
				}
			}
		}

		swap(rolls_curr, rolls_next);
	}

	return num_rolls;
}

int main() {
	const auto input = load_input();

	cout << "part 1: " << part_1(input) << endl;
	cout << "part 2: " << part_2(input) << endl;

	return 0;
}
