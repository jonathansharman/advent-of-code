use std::collections::{HashMap, VecDeque};

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
	let mut first_match_maps = Vec::new();
	input!().parse_lines().for_each(|mut secret: i64| {
		let mut first_match_map = HashMap::new();
		let mut changes = VecDeque::new();
		for _ in 0..EVOLUTIONS {
			let next = evolve(secret);
			let price = next % 10;
			changes.push_back(price - secret % 10);
			secret = next;
			if changes.len() == 5 {
				changes.pop_front();
				first_match_map
					.entry([changes[0], changes[1], changes[2], changes[3]])
					.or_insert(price);
			}
		}
		first_match_maps.push(first_match_map);
	});
	let mut max_bananas = 0;
	for i in -9..=9 {
		for j in -9..=9 {
			for k in -9..=9 {
				for l in -9..=9 {
					let sequence = [i, j, k, l];
					let bananas = first_match_maps
						.par_iter()
						.map(|m| m.get(&sequence).unwrap_or(&0))
						.sum();
					max_bananas = max_bananas.max(bananas);
				}
			}
		}
	}
	max_bananas
}
