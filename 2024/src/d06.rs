use std::collections::HashSet;

use aoc::io::read_lines;
use rayon::prelude::*;

aoc::test::test_part!(test1, part1, 5067);
aoc::test::test_part!(test2, part2, 1793);

type Point = (i32, i32);
type Tiles = Vec<Vec<char>>;

fn read() -> (Tiles, Point) {
	let mut guard: Point = (0, 0);
	let tiles: Vec<Vec<char>> = read_lines("input/06.txt")
		.enumerate()
		.map(|(i, line)| {
			line.chars()
				.enumerate()
				.map(|(j, c)| {
					if c == '^' {
						guard = (i as i32, j as i32);
						'.'
					} else {
						c
					}
				})
				.collect()
		})
		.collect();
	(tiles, guard)
}

fn escape_path(
	tiles: &Tiles,
	mut guard: Point,
	obstruction: Option<Point>,
) -> Option<HashSet<Point>> {
	let mut dir: Point = (-1, 0);
	let mut visited = HashSet::new();
	while let Some(tile) = tiles
		.get(guard.0 as usize)
		.and_then(|row| row.get(guard.1 as usize))
	{
		if visited.contains(&(guard, dir)) {
			// Cycle.
			return None;
		}
		if *tile == '#' || obstruction == Some(guard) {
			// Turn.
			guard.0 -= dir.0;
			guard.1 -= dir.1;
			dir = (dir.1, -dir.0)
		} else {
			visited.insert((guard, dir));
		}
		// Walk.
		guard.0 += dir.0;
		guard.1 += dir.1;
	}
	Some(
		visited
			.into_iter()
			.map(|(guard, _)| guard)
			.collect::<HashSet<_>>(),
	)
}

pub fn part1() -> usize {
	let (tiles, guard) = read();
	escape_path(&tiles, guard, None).unwrap().len()
}

pub fn part2() -> usize {
	let (tiles, guard) = read();
	escape_path(&tiles, guard, None)
		.unwrap()
		.par_iter()
		.filter(|&obstruction| {
			tiles[obstruction.0 as usize][obstruction.1 as usize] != '#'
				&& *obstruction != guard
				&& escape_path(&tiles, guard, Some(*obstruction)).is_none()
		})
		.count()
}
