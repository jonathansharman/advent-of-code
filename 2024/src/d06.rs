use std::collections::{HashMap, HashSet};

use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 5067);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut guard: (i32, i32) = (0, 0);
	let mut dir: (i32, i32) = (-1, 0);
	let tiles: HashMap<(i32, i32), char> = read_lines("input/06.txt")
		.enumerate()
		.flat_map(|(i, line)| {
			line.chars()
				.enumerate()
				.map(|(j, c)| {
					let coords = (i as i32, j as i32);
					let c = if c == '^' {
						guard = coords;
						'.'
					} else {
						c
					};
					(coords, c)
				})
				.collect::<Vec<_>>()
		})
		.collect();
	let mut visited = HashSet::new();
	while let Some(tile) = tiles.get(&guard) {
		if *tile == '#' {
			// Turn.
			guard.0 -= dir.0;
			guard.1 -= dir.1;
			dir = (dir.1, -dir.0)
		} else {
			visited.insert(guard);
		}
		// Walk.
		guard.0 += dir.0;
		guard.1 += dir.1;
	}
	visited.len()
}

pub fn part2() -> usize {
	0
}
