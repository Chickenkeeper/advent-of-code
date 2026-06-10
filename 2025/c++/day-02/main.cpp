#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

struct range {
	const long start;
	const long end;
};

vector<range> load_input() {
	vector<range> input;

	ifstream file("input");
	long     id_start;
	long     id_end;

	do {
		file >> id_start;
		file.get(); // skip hyphen
		file >> id_end;

		input.push_back(range {id_start, id_end});
	} while (file.get() != '\n');

	return input;
}

/**
 * For each id in each range, simply convert it to a string and then, if the string can be
 * split into two equal halves, compare both halves. If both halves match, the id's invalid.
 */
long part_1(const vector<range> &input) {
	long sum = 0;

	for (const auto &range : input) {
		for (long id = range.start; id <= range.end; id++) {
			const auto id_str = to_string(id);

			// if the id string can be split into two halves of equal length, do so
			if (id_str.size() % 2 == 0) {
				const long midpoint = id_str.size() / 2;

				// check whether both halves are equivalent
				if (id_str.substr(0, midpoint) == id_str.substr(midpoint)) {
					sum += id;
				}
			}
		}
	}

	return sum;
}

/**
 * For each id in each range, convert it to a string. Then, instead of just comparing both
 * halves of the string, compare contiguous substrings of increasing length from 1 to half
 * the length of the string, skipping any length that the string's length isn't a multiple
 * of (e.g. for an ID of 123123, first compare each digit individually, then 12, 31 and 23,
 * then 123 and 123). If the string's found to be comprised of any repeated substring, then
 * the id's invalid.
 */
long part_2(const vector<range> &input) {
	long sum = 0;

	for (const auto &range : input) {
		for (long id = range.start; id <= range.end; id++) {
			const auto id_str      = to_string(id);
			const long max_seq_len = id_str.size() / 2;

			// for each length > 1 up to half the length of the string
			for (int l = 1; l <= max_seq_len; l++) {
				// skip if the string's length isn't a multiple of the substring length
				if (id_str.size() % l != 0) {
					continue;
				}

				// create a substring from the start of the id string to that length
				const long max_seqs = id_str.size() / l;
				const auto seq      = id_str.substr(0, l);

				bool invalid = true;

				// then check whether the id string consists of repeated copies of that substring
				for (int p = 1; p < max_seqs; p++) {
					const auto curr_seq = id_str.substr(p * l, l);

					// early-out if substrings don't match
					if (curr_seq != seq) {
						invalid = false;
						break;
					}
				}

				if (invalid) {
					sum += id;
					break;
				}
			}
		}
	}

	return sum;
}

int main() {
	const auto input = load_input();

	cout << "part 1: " << part_1(input) << endl;
	cout << "part 2: " << part_2(input) << endl;

	return 0;
}
