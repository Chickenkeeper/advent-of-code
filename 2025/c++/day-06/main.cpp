#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

struct homework {
	const vector<char> chars;
	const int          cols;
	const int          rows;
};

homework load_input() {
	vector<char> chars;
	int          cols = 0;
	int          rows = 0;

	ifstream file("input");
	char     ch;

	while (file.get(ch)) {
		chars.push_back(ch);

		if (rows == 0) {
			cols++;
		}

		if (ch == '\n') {
			rows++;
		}
	}

	return homework{chars, cols, rows};
}

/**
 * Iterate over each problem, first reading the problem's operator and then its
 * values, and performing the relevant calculations in the process. Since the
 * operators are commutative, the order in which values are read and problems
 * are solved isn't important.
 *
 * Since the homework input is stored as a grid of characters, and since the
 * homework is laid out as a table where the width of each column varies by the
 * minimum width needed to store any number in that column, we also need to keep
 * track of the maximum width of the numbers in each problem so we know where
 * the next problem column starts.
 */
long part_1(const homework &input) {
	int  col_end = 0; // the ending column of this problem
	long total   = 0;

	// for each problem
	while (col_end < input.cols) {
		// read the operation
		const char op = input.chars[(input.rows - 1) * input.cols + col_end];

		// remember which column of characters
		// marks the start of this problem column
		int  col_start = col_end;
		long problem   = 0;

		// iterate over each row of numbers, top to bottom
		for (int row = 0; row < input.rows - 1; row++) {
			int  col = col_start;
			long num = 0;

			// skip leading whitespace
			while (input.chars[row * input.cols + col] == ' ') {
				col++;
			}

			// read a number
			while (true) {
				const char ch = input.chars[row * input.cols + col];

				// stop when trailing whitespace is found
				if (ch < '0' || ch > '9') {
					break;
				}

				num = num * 10 + (ch - '0');
				col++;
			}

			// keep track of the start of the next problem column
			col_end = max(col_end, col + 1);

			// solve the next step of this problem
			if (problem == 0) {
				problem = num;
			} else if (op == '+') {
				problem += num;
			} else {
				problem *= num;
			}
		}

		total += problem;
	}

	return total;
}

/**
 * Similar to part 1, but since both the problems and their values are read
 * vertically then we don't need to iterate over problems and values separately.
 * Instead, we can just iterate over each column of characters, and when a
 * column which contains only whitespace is found then we know we've reached the
 * end of the current problem and can start the next one.
 */
long part_2(const homework &input) {
	int  col   = 0;
	long total = 0;

	// for each problem
	while (col < input.cols) {
		// read the operation
		const char op = input.chars[(input.rows - 1) * input.cols + col];

		long problem = 0;

		// for each column of digits in this problem
		while (true) {
			long num       = 0;
			bool num_found = false;

			// try to read a number
			for (int row = 0; row < input.rows - 1; row++) {
				const char ch = input.chars[row * input.cols + col];

				// skip any whitespace
				if (ch < '0' || ch > '9') {
					continue;
				}

				num       = num * 10 + (ch - '0');
				num_found = true;
			}

			col++;

			// if the column was empty then we've finished this problem
			if (!num_found) {
				break;
			}

			// solve the next step of this problem
			if (problem == 0) {
				problem = num;
			} else if (op == '+') {
				problem += num;
			} else {
				problem *= num;
			}
		}

		total += problem;
	}

	return total;
}

int main() {
	const auto input = load_input();

	cout << "part 1: " << part_1(input) << endl;
	cout << "part 2: " << part_2(input) << endl;

	return 0;
}
