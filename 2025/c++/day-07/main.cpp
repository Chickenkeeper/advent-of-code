#include <fstream>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

struct beam {
	int left_child_index;
	int right_child_index;

	bool isLeaf() const {
		return left_child_index == -1 && right_child_index == -1;
	}
};

/**
 * Loads and translates the input file into a graph of beams. The first
 * element in the vector is the root node.
 *
 * To do this, it begins by scanning the first line of the input for the
 * column in which the first beam starts, and stores that column in a map
 * of columns which contain beams to the index of that beam in the graph.
 *
 * Then for every following line, if a splitter is found the map is
 * checked to see if a beam is travelling down that column. If so, we
 * know that the beam will hit the splitter and be split into two other
 * beams to the left and right of that column, so we calculate the
 * columns that the new beams will travel down and remove the beam to be
 * split from the map. Then we check whether there are already beams
 * travelling down the left and right columns, creating new beams if not,
 * and set the left and right beams as the children of the beam that was
 * just split.
 */
vector<beam> load_input() {
	vector<beam> beams;

	unordered_map<int, int> beam_cols;
	ifstream                file("input");
	char                    ch;
	int                     col = 0;

	// find start pos
	while (file.get() != 'S') {
		col++;
	}

	// initialize the graph
	beams.push_back(beam{-1, -1});
	beam_cols.emplace(col, 0);
	col++;

	while (file.get(ch)) {

		// reset the column counter at the end of each line
		if (ch == '\n') {
			col = 0;
			continue;
		}

		// if a splitter is found and a beam is travelling down the same column, split the beam
		if (ch == '^' && beam_cols.find(col) != beam_cols.end()) {
			// calculate the columns that the new beams will travel down
			const auto left_beam_col   = col - 1;
			const auto right_beam_col  = col + 1;

			// extract the index of the beam currently travelling down this column
			const auto prev_beam_index = beam_cols.extract(col).mapped();

			// if a beam already exists along the left column, set it as the current beam's left
			// child, otherwise create a new beam and set it as the current beam's left child
			if (beam_cols.find(left_beam_col) != beam_cols.end()) {
				beams[prev_beam_index].left_child_index = beam_cols[left_beam_col];
			} else {
				const auto left_beam_index = beams.size();

				beams.push_back(beam{-1, -1});
				beams[prev_beam_index].left_child_index = left_beam_index;
				beam_cols.emplace(left_beam_col, left_beam_index);
			}

			// same as above, but for the right side
			if (beam_cols.find(right_beam_col) != beam_cols.end()) {
				beams[prev_beam_index].right_child_index = beam_cols[right_beam_col];
			} else {
				const auto right_beam_index = beams.size();

				beams.push_back(beam{-1, -1});
				beams[prev_beam_index].right_child_index = right_beam_index;
				beam_cols.emplace(right_beam_col, right_beam_index);
			}
		}

		col++;
	}

	return beams;
}

/**
 * Since the input is stored as a graph of beam paths,
 * we can deduce how many times the beam splits by simply
 * counting the number of beam nodes that have children.
 */
long part_1(const vector<beam> &input) {
	long num_splits = 0;

	for (const auto beam : input) {
		if (!beam.isLeaf()) {
			num_splits++;
		}
	}

	return num_splits;
}

/**
 * Since the input is stored as a graph of beam paths we can count
 * the number of 'timelines', or unique paths, using a depth-first
 * search, using memoization to accelerate the process.
 */
long part_2(const vector<beam> &input) {
	vector<int>  beam_index_stack = {0}; // initialize with the root node
	vector<long> memo(input.size(), -1); // mark all paths as unexplored

	// depth-first search for timelines
	while (!beam_index_stack.empty()) {
		const auto beam_index = beam_index_stack.back();
		const auto beam       = input[beam_index];

		// skip leaf nodes
		if (beam.isLeaf()) {
			memo[beam_index] = 1; // leaf nodes only contribute one extra timeline
			beam_index_stack.pop_back();
			continue;
		}

		// check whether either path has already been memoized
		const auto num_timelines_left  = memo[beam.left_child_index];
		const auto num_timelines_right = memo[beam.right_child_index];

		if (num_timelines_left == -1) {
			// left child unexplored, explore left path
			beam_index_stack.push_back(beam.left_child_index);
		} else if (num_timelines_right == -1) {
			// right child unexplored, explore right path
			beam_index_stack.push_back(beam.right_child_index);
		} else {
			// both children explored, store the sum of the children's timelines
			memo[beam_index] = num_timelines_left + num_timelines_right;
			beam_index_stack.pop_back();
		}
	}

	return memo[0];
}

int main() {
	const auto input = load_input();

	cout << "part 1: " << part_1(input) << endl;
	cout << "part 2: " << part_2(input) << endl;

	return 0;
}
