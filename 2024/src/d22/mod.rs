use std::collections::{HashMap, HashSet, VecDeque};

use aoc::input::{input, ParseLines};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

aoc::test::test_part!(test1, part1, 14180628689);
aoc::test::test_part!(test2, part2, 1690);

const MODULUS: i64 = 16777216;
const EVOLUTIONS: usize = 2000;

fn evolve(mut secret: i64) -> i64 {
	secret = (secret ^ (64 * secret)) % MODULUS;
	secret = (secret ^ (secret / 32)) % MODULUS;
	secret = (secret ^ (2048 * secret)) % MODULUS;
	secret
}

pub fn part1() -> i64 {
	input!()
		.parse_lines()
		.map(|mut secret: i64| {
			for _ in 0..EVOLUTIONS {
				secret = evolve(secret);
			}
			secret
		})
		.sum()
}

pub fn part2() -> i64 {
	let mut all_sequences = HashSet::new();
	let mut first_match_maps = Vec::new();
	input!().parse_lines().for_each(|mut secret: i64| {
		let mut first_match_map = HashMap::new();
		let mut changes = VecDeque::new();
		for _ in 0..EVOLUTIONS {
			let next = evolve(secret);
			let price = next % 10;
			let change = price - secret % 10;
			changes.push_back(change);
			secret = next;
			if changes.len() == 5 {
				changes.pop_front();
				let sequence = [changes[0], changes[1], changes[2], changes[3]];
				first_match_map.entry(sequence).or_insert(price);
				all_sequences.insert(sequence);
			}
		}
		first_match_maps.push(first_match_map);
	});
	all_sequences
		.par_iter()
		.map(|sequence| {
			first_match_maps
				.iter()
				.map(|m| m.get(sequence).unwrap_or(&0))
				.sum()
		})
		.max()
		.unwrap()
}
