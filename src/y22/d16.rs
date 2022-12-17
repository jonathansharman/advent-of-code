use std::collections::{HashMap, HashSet, VecDeque};

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
	tunnels: HashMap<String, Vec<String>>,
) -> HashMap<String, HashMap<String, i32>> {
	tunnels
		.keys()
		.map(|v| (v.clone(), get_one_distances(&tunnels, v)))
		.collect()
}

const TIME_LIMIT: i32 = 30;

fn max_pressure(
	rates: &HashMap<String, i32>,
	distances: &HashMap<String, HashMap<String, i32>>,
	time: i32,
	total_rate: i32,
	location: &str,
	remaining_valves: HashSet<String>,
) -> i32 {
	remaining_valves
		.iter()
		.map(|next| {
			let dt = distances[location][next] + 1;
			if time + dt > TIME_LIMIT {
				return (TIME_LIMIT - time) * total_rate;
			}
			let time = time + dt;
			let mut child_remaining_values = remaining_valves.clone();
			child_remaining_values.remove(next);
			let child_pressure = max_pressure(
				rates,
				distances,
				time,
				total_rate + rates[next],
				next,
				child_remaining_values,
			);
			child_pressure + dt * total_rate
		})
		.max()
		.unwrap_or((TIME_LIMIT - time) * total_rate)
}

pub fn part1() -> i32 {
	let (tunnels, rates) = get_tunnels_and_rates();
	let distances = get_all_distances(tunnels);
	max_pressure(
		&rates,
		&distances,
		0,
		0,
		"AA",
		rates.keys().cloned().collect(),
	)
}

pub fn part2() -> usize {
	0
}
