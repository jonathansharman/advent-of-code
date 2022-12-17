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

const TIME_LIMIT: i32 = 30;

pub fn part1() -> i32 {
	let (tunnels, rates) = get_tunnels_and_rates();
	let distances = get_all_distances(&tunnels);
	rates
		.keys()
		.permutations(rates.len())
		.map(|mut visit_order| {
			let mut pressure = 0;
			let mut total_rate = 0;
			let mut time = 0;
			let mut location = "AA";
			loop {
				let Some(next) = visit_order.pop() else { break };
				let distance = distances[location][next];
				let dt = distance + 1;
				if time + dt > TIME_LIMIT {
					break;
				}
				time += dt;
				pressure += dt * total_rate;
				location = next;
				total_rate += rates[location];
			}
			pressure + (TIME_LIMIT - time) * total_rate
		})
		.max()
		.unwrap_or_default()
}

pub fn part2() -> usize {
	0
}
