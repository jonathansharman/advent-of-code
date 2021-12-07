use std::collections::{hash_map::Entry, HashMap};

use crate::io::read_comma_separated_integers;

pub fn part1() -> i64 {
	fish(80)
}

pub fn part2() -> i64 {
	fish(256)
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
	#[test]
	fn part1() {
		assert_eq!(379114, super::part1());
	}

	#[test]
	fn part2() {
		assert_eq!(1702631502303, super::part2());
	}
}
