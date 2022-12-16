use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

fn get_tunnels_and_rates(
) -> (HashMap<String, Vec<String>>, HashMap<String, i32>) {
	let mut tunnels = HashMap::new();
	let mut rates = HashMap::new();
	for line in read_lines("input/2022/16.txt") {
		let words: Vec<&str> = line
			.split(|c: char| !c.is_alphanumeric())
			.filter(|s| !s.is_empty())
			.collect();
		let valve = words[1].to_owned();
		let rate = words[5].parse::<i32>().unwrap();
		let exits = words[10..].iter().map(|&s| s.to_owned()).collect_vec();
		if rate > 0 {
			rates.insert(valve.clone(), rate);
		}
		tunnels.insert(valve, exits);
	}
	(tunnels, rates)
}

fn get_one_distances(
	tunnels: &HashMap<String, Vec<String>>,
	v: &str,
) -> HashMap<String, i32> {
	let mut distances = HashMap::from([(v.to_owned(), 0)]);
	let mut queue = VecDeque::from([v]);
	while let Some(current) = queue.pop_front() {
		let current_distance = distances[current];
		for neighbor in tunnels.get(current).unwrap() {
			distances.entry(neighbor.clone()).or_insert_with(|| {
				queue.push_back(neighbor);
				current_distance + 1
			});
		}
	}
	distances
}

fn get_all_distances(
	tunnels: &HashMap<String, Vec<String>>,
) -> HashMap<String, HashMap<String, i32>> {
	tunnels
		.keys()
		.map(|v| (v.clone(), get_one_distances(tunnels, v)))
		.collect()
}

pub fn part1() -> usize {
	let (tunnels, rates) = get_tunnels_and_rates();
	let distances = get_all_distances(&tunnels);
	println!("Tunnels: {tunnels:?}");
	println!("Rates: {rates:?}");
	println!("Distances: {distances:?}");
	// Guaranteed optimal: try every permutation of non-zero valves. Requires at
	// least O(n!) time.
	//
	// Decent heuristic that's not guaranteed optimal: greedily always choose
	// the next valve based on which one will reduce the most total pressure,
	// factoring in travel time.
	//
	// Note: This is suboptimal for instance when two nearby valves are each
	// almost as good as the greedy choice. Essentially it doesn't account for
	// the value of subsequent moves. The real input is long enough that this
	// might be a real problem.
	//
	// Going to try exhaustive search with some pruning.
	0
}

pub fn part2() -> usize {
	0
}
