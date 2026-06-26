#include <algorithm>
#include <fstream>
#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

struct junc_box {
	int x;
	int y;
	int z;

	/**
	 * Returns the squared distance between two junction boxes.
	 */
	static long dist_sq(const junc_box &a, const junc_box &b) {
		const long vx = a.x - b.x;
		const long vy = a.y - b.y;
		const long vz = a.z - b.z;

		return vx * vx + vy * vy + vz * vz;
	}

	bool operator==(const junc_box& a) const {
		return x == a.x && y == a.y && z == a.z;
	}
};

template<>
struct std::hash<junc_box> {
	size_t operator()(const junc_box& box) const noexcept {
		const auto h1 = hash<int>{}(box.x);
		const auto h2 = hash<int>{}(box.y);
		const auto h3 = hash<int>{}(box.z);

		auto seed = h1;
		seed ^= h2 + 0x9e3779b9 + (seed << 6) + (seed >> 2); // old version of Boost's hash_combine
		seed ^= h3 + 0x9e3779b9 + (seed << 6) + (seed >> 2);

		return seed;
	}
};

vector<junc_box> load_input() {
	vector<junc_box> junction_boxes;

	ifstream file("input");
	char     comma;
	int      x;
	int      y;
	int      z;

	while (file >> x >> comma >> y >> comma >> z) {
		junction_boxes.push_back(junc_box{x, y, z});
	}

	return junction_boxes;
}

/**
 * Both parts use the same solution at different stages of computation.
 *
 * The process described in the problem statement is essentially a modified
 * version of Kruskal's algorithm for finding a minimum spanning forest of an
 * undirected edge-weighted graph, which this solution is an implementation of.
 */
int main() {
	const auto input = load_input();

	long part_1 = 0;
	long part_2 = 0;

	vector<pair<junc_box, junc_box>> box_pairs;       // every unique pair of junction boxes
	vector<unordered_set<junc_box>>  circuits;        // which junction boxes are in each circuit
	unordered_map<junc_box, size_t>  box_circuit_map; // the index of the circuit each box belongs to

	// initialize the data structures
	for (size_t i = 0; i < input.size(); i++) {
		const auto box = input[i];

		for (size_t j = i + 1; j < input.size(); j++) {
			box_pairs.push_back(pair{box, input[j]});
		}

		circuits.push_back({box}); // each box starts out in its own circuit
		box_circuit_map[box] = i;
	}

	// sort all pairs of junction boxes by the distance between them, in ascending order
	sort(box_pairs.begin(), box_pairs.end(), [](auto a, auto b) {
		return junc_box::dist_sq(a.first, a.second) < junc_box::dist_sq(b.first, b.second);
	});

	pair<junc_box, junc_box> prev_box_pair;

	// combine pairs of circuits until there's only one left
	for (size_t i = 0; circuits.size() > 1; i++) {

		// solve part 1
		if (i == 1000) {
			size_t max_circuit_size_1 = 0;
			size_t max_circuit_size_2 = 0;
			size_t max_circuit_size_3 = 0;

			// simple sort to find the 3 largest circuits
			for (const auto &circuit : circuits) {
				if (circuit.size() > max_circuit_size_3) {
					max_circuit_size_3 = circuit.size();

					if (circuit.size() > max_circuit_size_2) {
						swap(max_circuit_size_2, max_circuit_size_3);

						if (circuit.size() > max_circuit_size_1) {
							swap(max_circuit_size_1, max_circuit_size_2);
						}
					}
				}
			}

			part_1 = max_circuit_size_1 * max_circuit_size_2 * max_circuit_size_3;
		}

		// get the next smallest pair of junction boxes and the circuits they belong to
		const auto box_pair             = box_pairs[i];
		const auto first_circuit_index  = box_circuit_map[box_pair.first];
		const auto second_circuit_index = box_circuit_map[box_pair.second];

		// skip if both junction boxes are already part of the same circuit
		if (first_circuit_index == second_circuit_index) {
			continue;
		}

		prev_box_pair = box_pair;

		auto &first_circuit  = circuits[first_circuit_index];
		auto &second_circuit = circuits[second_circuit_index];

		// combine both circuits by moving all junction
		// boxes from the second circuit into the first
		while (!second_circuit.empty()) {
			const auto box = second_circuit.extract(second_circuit.begin()).value();

			box_circuit_map[box] = first_circuit_index;
			first_circuit.insert(box);
		}

		// remove the second circuit from the vector of circuits by replacing it
		// with the last circuit (unless the circuit to remove is the last one)
		if (second_circuit_index != circuits.size() - 1) {
			for (const auto box : circuits.back()) {
				box_circuit_map[box] = second_circuit_index;
			}

			swap(second_circuit, circuits.back());
		}

		circuits.pop_back();
	}

	// solve part 2
	part_2 = prev_box_pair.first.x * prev_box_pair.second.x;

	cout << "part 1: " << part_1 << endl;
	cout << "part 2: " << part_2 << endl;

	return 0;
}
