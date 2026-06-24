#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

struct range {
	size_t start;
	size_t end;

	long length() {
		return end - start + 1;
	}

	static bool compare(range a, range b) {
		if (a.start == b.start) {
			return b.end < a.end;
		} else {
			return a.start < b.start;
		}
	}
};

struct database {
	vector<range>  fresh_id_ranges;
	vector<size_t> available_ids;
};

database load_input() {
	vector<range>  fresh_id_ranges;
	vector<size_t> available_ids;

	ifstream file("input");
	size_t   id_start;
	size_t   id_end;

	// load id ranges
	while (file.peek() != '\n') {
		file >> id_start;
		file.get(); // ignore dash
		file >> id_end;
		file.get(); // ignore newline

		fresh_id_ranges.push_back(range{id_start, id_end});
	}

	// load available ids
	while (file >> id_start) {
		available_ids.push_back(id_start);
	}

	// sort both vectors so the solutions can run
	// in linear time instead of quadratic time
	sort(fresh_id_ranges.begin(), fresh_id_ranges.end(), range::compare);
	sort(available_ids.begin(), available_ids.end(), less<size_t>());

	return database{fresh_id_ranges, available_ids};
}

/**
 * Since the input for this puzzle isn't too large, it's possible to
 * solve this part by simply checking every available id against every
 * id range.
 *
 * However, a more scalable and potentially faster solution is to loop
 * over the (sorted) available ids and id ranges once; advancing the
 * id range if the end of the range is smaller than the currently
 * selected available id and advancing to the next available id if
 * it's contained within the currently selected id range, while
 * incrementing the total number of fresh ids if the available id is
 * contained within the id range.
 */
size_t part_1(const database &input) {
	size_t num_fresh = 0;

	size_t fresh_id_range_index = 0;
	size_t available_id_index   = 0;

	// search until we run out of available ids or id ranges
	while (
		fresh_id_range_index < input.fresh_id_ranges.size() &&
		available_id_index   < input.available_ids.size()
	) {
		const range  fresh_id_range = input.fresh_id_ranges[fresh_id_range_index];
		const size_t available_id   = input.available_ids[available_id_index];

		// this condition handles both overlapping and nested ranges
		if (available_id > fresh_id_range.end) {
			fresh_id_range_index++;
		} else {
			available_id_index++;

			// check whether the available id is contained within the range
			if (available_id >= fresh_id_range.start) {
				num_fresh++;
			}
		}
	}

	return num_fresh;
}

/**
 * All that's needed for this part is to loop over each (sorted) id range
 * and check whether it overlaps with the next one; if it does then merge
 * the ranges together, otherwise add its length to the total number of
 * fresh ids and advance to the next range.
 */
size_t part_2(database &input) {
	size_t num_fresh  = 0;
	range  curr_range = input.fresh_id_ranges[0];

	// loop over each id range
	for (size_t i = 1; i < input.fresh_id_ranges.size(); i++) {
		const range next_range = input.fresh_id_ranges[i];

		// check whether the current and next range overlap
		if (next_range.start <= curr_range.end) {

			// make sure that the end of the next range is actually greater than
			// the end of the current one, otherwise the current range won't end up
			// encompassing the values from both ranges after changing its endpoint
			if (next_range.end > curr_range.end) {
				curr_range.end = next_range.end;
			}
		} else {
			// add the length of the current range to
			// the total and advance to the next one
			num_fresh += curr_range.length();
			curr_range = next_range;
		}
	}

	// include the length of the very last range
	num_fresh += curr_range.length();

	return num_fresh;
}

int main() {
	auto input = load_input();

	cout << "part 1: " << part_1(input) << endl;
	cout << "part 2: " << part_2(input) << endl;

	return 0;
}
