use std::collections::HashSet;

use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 484023871);
aoc::test::test_part!(test2, part2, 46294175);

struct RangeMap {
	dst: usize,
	src: usize,
	len: usize,
}

impl RangeMap {
	fn map(&self, n: usize) -> Option<usize> {
		if (self.src..self.src + self.len).contains(&n) {
			Some(self.dst + n - self.src)
		} else {
			None
		}
	}
}

struct Map(Vec<RangeMap>);

impl Map {
	fn map(&self, n: usize) -> usize {
		self.0
			.iter()
			.filter_map(|range_map| range_map.map(n))
			.next()
			.unwrap_or(n)
	}

	fn preimage(&self, n: usize) -> HashSet<usize> {
		let mut result = HashSet::new();
		// Check for ranges that explicitly map to this number.
		for range_map in self.0.iter() {
			let dst_range = range_map.dst..range_map.dst + range_map.len;
			if dst_range.contains(&n) {
				result.insert(range_map.src + n - range_map.dst);
			}
		}
		// Check if this number is implicitly mapped to from itself.
		if self.map(n) == n {
			result.insert(n);
		}
		result
	}
}

pub fn part1() -> usize {
	let mut lines = input!().lines();
	let seeds = lines
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.map(|s| s.parse().unwrap())
		.collect::<Vec<usize>>();
	lines.next();
	let mut maps = Vec::new();
	while lines.next().is_some() {
		let mut map = Vec::new();
		for line in lines.by_ref() {
			if line.is_empty() {
				break;
			}
			let numbers = line
				.split_whitespace()
				.map(|s| s.parse().unwrap())
				.collect::<Vec<_>>();
			map.push(RangeMap {
				dst: numbers[0],
				src: numbers[1],
				len: numbers[2],
			});
		}
		maps.push(Map(map));
	}
	seeds
		.into_iter()
		.map(|mut n| {
			for map in maps.iter() {
				n = map.map(n);
			}
			n
		})
		.min()
		.unwrap()
}

pub fn part2() -> usize {
	let mut lines = input!().lines();
	let seed_ranges = lines
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.map(|s| s.parse().unwrap())
		.tuples()
		.collect::<Vec<(usize, usize)>>();
	lines.next();
	let mut maps = Vec::new();
	while lines.next().is_some() {
		let mut map = Map(Vec::new());
		for line in lines.by_ref() {
			if line.is_empty() {
				break;
			}
			let numbers = line
				.split_whitespace()
				.map(|s| s.parse().unwrap())
				.collect::<Vec<_>>();
			map.0.push(RangeMap {
				dst: numbers[0],
				src: numbers[1],
				len: numbers[2],
			});
		}
		maps.push(map);
	}

	// Split each map at the boundaries of each of its source ranges.
	let mut split_sets = maps
		.iter()
		.map(|map| {
			map.0
				.iter()
				.flat_map(|range_map| {
					[range_map.src, range_map.src + range_map.len]
				})
				.collect::<HashSet<_>>()
		})
		.collect::<Vec<_>>();

	// Split map i at the preimages of all splits in map i + 1.
	for (i, map) in maps.iter().enumerate().rev().skip(1) {
		let mut new_splits = HashSet::new();
		for split in &split_sets[i + 1] {
			new_splits.extend(map.preimage(*split));
		}
		split_sets[i].extend(new_splits);
	}

	// Split the seed ranges into subranges whose images in location space are
	// contiguous.
	let mut queue = seed_ranges;
	let mut subranges = Vec::new();
	'outer: while let Some((src, len)) = queue.pop() {
		for split in split_sets[0].iter() {
			if (src + 1..src + len).contains(split) {
				queue.push((src, *split - src));
				queue.push((*split, src + len - *split));
				continue 'outer;
			}
		}
		subranges.push((src, len));
	}

	subranges
		.into_iter()
		// Only need to check the lowest element of each of the subranges.
		.map(|(mut n, _)| {
			for map in maps.iter() {
				n = map.map(n);
			}
			n
		})
		.min()
		.unwrap()
}
