use crate::{io::read_comma_separated_integers, solution::Solution};

use std::collections::{hash_map::Entry, HashMap};

pub struct Day6;

impl Solution for Day6 {
	fn year(&self) -> u32 {
		2021
	}

	fn day(&self) -> u32 {
		5
	}

	fn part1(&self) -> i64 {
		fish(80)
	}

	fn part2(&self) -> i64 {
		fish(256)
	}
}

pub fn fish(days: i64) -> i64 {
	let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
	read_comma_separated_integers("input/2021/6.txt")
		.fold(0, |acc, timer| acc + get_count(&mut cache, timer, days))
}

fn get_count(cache: &mut HashMap<(i64, i64), i64>, timer: i64, days: i64) -> i64 {
	if days == 0 {
		return 1;
	}
	if let Entry::Occupied(kv) = cache.entry((timer, days)) {
		return *kv.get();
	}

	let count = if timer > 0 {
		get_count(cache, timer - 1, days - 1)
	} else {
		get_count(cache, 6, days - 1) + get_count(cache, 8, days - 1)
	};
	cache.insert((timer, days), count);
	count
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(379114, Day6.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(1702631502303, Day6.part2());
	}
}
