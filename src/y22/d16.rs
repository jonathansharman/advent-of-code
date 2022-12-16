use std::collections::HashMap;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
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
	println!("Tunnels: {tunnels:?}");
	println!("Rates: {rates:?}");
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
