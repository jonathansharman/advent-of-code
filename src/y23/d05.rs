use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 484023871);
crate::test::test_part!(test2, part2, 46294175);

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

pub fn part1() -> usize {
	let mut lines = read_lines("input/2023/05.txt");
	let seeds = lines
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.map(|s| s.parse().unwrap())
		.collect::<Vec<usize>>();
	lines.next();
	// source → (destination, length)
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
		maps.push(map);
	}
	seeds
		.into_iter()
		.map(|mut src| {
			for map in maps.iter() {
				for range_map in map.iter() {
					if let Some(dst) = range_map.map(src) {
						src = dst;
						break;
					}
				}
			}
			src
		})
		.min()
		.unwrap()
}

pub fn part2() -> usize {
	let mut lines = read_lines("input/2023/05.txt");
	let seeds = lines
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.map(|s| s.parse().unwrap())
		.collect::<Vec<usize>>();
	lines.next();
	// source → (destination, length)
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
		maps.push(map);
	}

	seeds
		.into_iter()
		.tuples()
		.flat_map(|(seed, len)| (seed..seed + len))
		.map(|mut src| {
			for map in maps.iter() {
				for range_map in map.iter() {
					if let Some(dst) = range_map.map(src) {
						src = dst;
						break;
					}
				}
			}
			src
		})
		.min()
		.unwrap()
}
