#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

vector<vector<char>> load_input() {
	vector<vector<char>> input;

	ifstream file("input");
	int      bank_len = 0;

	// find battery bank size
	while (file.get() != '\n') {
		bank_len++;
	}

	// rewind to start of file
	file.seekg(0);

	// load battery banks
	while (!file.eof()) {
		vector<char> battery_bank; battery_bank.reserve(bank_len);
		char         ch;

		while (file.get(ch) && ch != '\n') {
			battery_bank.push_back(ch - '0');
		}

		input.push_back(battery_bank);
	}

	return input;
}

/**
 * We know that the number of digits in the total joltage must equal the specified number
 * of batteries to enable in the bank, and we know intuitively that each digit in the
 * highest total joltage must be less than/equal to the one before it.
 *
 * Therefore, we can search for each battery to enable in turn; if 'remaining_batteries'
 * is the number of batteries that have yet to be enabled, then the next battery to enable
 * must be the leftmost one with the highest joltage in the range between the battery to
 * the right of the one previously enabled (or the first if none have been enabled yet)
 * and the one remaining_batteries to the left of the end of the battery bank (to leave
 * room for any other batteries that need to be enabled), which can be found with a
 * simple linear search.
 */
long find_max_joltage(const vector<char> &battery_bank, const int num_enabled_batteries) {
	int enabled_battery_joltages[num_enabled_batteries];
	int search_start_index = 0;
	int search_end_index   = battery_bank.size() - num_enabled_batteries;

	// look for each battery in turn, left to right
	for (int i = 0; i < num_enabled_batteries; i++) {
		int max_battery_joltage = 0;

		// search within the specified range, breaking early
		// if we find the highest possible battery joltage (9)
		for (int j = search_start_index; j <= search_end_index && max_battery_joltage < 9; j++) {
			const int battery_joltage = battery_bank[j];

			// keep track of the index and joltage of the best battery so far
			if (battery_joltage > max_battery_joltage) {
				max_battery_joltage = battery_joltage;
				search_start_index  = j;
			}
		}

		// store the joltage of the best battery in the search
		// range, and prepare the search range for the next one
		enabled_battery_joltages[i] = max_battery_joltage;
		search_start_index++;
		search_end_index++;
	}

	// convert the individual joltage digits to a single value, and return it
	long total_joltage = 0;

	for (const auto joltage : enabled_battery_joltages) {
		total_joltage = total_joltage * 10 + joltage;
	}

	return total_joltage;
}

/**
 * Both parts of this puzzle can be solved with different parameters of the same solution.
 */
int main() {
	const auto input = load_input();
	long total_joltage_1 = 0;
	long total_joltage_2 = 0;

	for (const auto &battery_bank : input) {
		total_joltage_1 += find_max_joltage(battery_bank, 2);
		total_joltage_2 += find_max_joltage(battery_bank, 12);
	}

	cout << "part 1: " << total_joltage_1 << endl;
	cout << "part 2: " << total_joltage_2 << endl;

	return 0;
}
