use std::collections::HashMap;

use itertools::Itertools;

use aoc::input::parse_lines;

aoc::test::test_part!(test1, part1, 2592);
aoc::test::test_part!(test2, part2, 198428693313536);

const INPUT: &str = include_str!("input/10.txt");

pub fn part1() -> usize {
	let mut adapters = parse_lines(INPUT).sorted().collect::<Vec<usize>>();
	adapters.insert(0, 0);
	adapters.push(adapters.last().unwrap() + 3);
	let (diff1s, diff3s) = adapters
		.into_iter()
		.tuple_windows()
		.map(|(a, b)| b - a)
		.fold((0, 0), |(diff1s, diff3s), diff| match diff {
			1 => (diff1s + 1, diff3s),
			3 => (diff1s, diff3s + 1),
			_ => (diff1s, diff3s),
		});
	diff1s * diff3s
}

fn valid_arrangements(
	adapters: &[usize],
	i: usize,
	cache: &mut HashMap<usize, usize>,
) -> usize {
	if let Some(n) = cache.get(&i) {
		return *n;
	}
	let n = adapters
		.iter()
		.enumerate()
		.skip(i + 1)
		.take(3)
		.filter_map(|(j, other)| {
			if other - adapters[i] <= 3 {
				Some(valid_arrangements(adapters, j, cache))
			} else {
				None
			}
		})
		.sum();
	cache.insert(i, n);
	n
}

pub fn part2() -> usize {
	// Note that due to part 1, we already know the end is reachable from any
	// index.
	let mut adapters = parse_lines(INPUT).sorted().collect::<Vec<usize>>();
	adapters.insert(0, 0);
	let mut cache = HashMap::new();
	cache.insert(adapters.len() - 1, 1);
	valid_arrangements(&adapters, 0, &mut cache)
}
