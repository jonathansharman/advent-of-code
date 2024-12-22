use aoc::input::{input, ParseCommaSeparated};

use std::collections::{hash_map::Entry, HashMap};

aoc::test::test_part!(test1, part1, 379114);
aoc::test::test_part!(test2, part2, 1702631502303);

pub fn part1() -> i64 {
	fish(80)
}

pub fn part2() -> i64 {
	fish(256)
}

pub fn fish(days: i64) -> i64 {
	let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
	input!()
		.parse_comma_separated()
		.fold(0, |acc, timer| acc + get_count(&mut cache, timer, days))
}

fn get_count(
	cache: &mut HashMap<(i64, i64), i64>,
	timer: i64,
	days: i64,
) -> i64 {
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
