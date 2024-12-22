use std::collections::HashSet;

use aoc::input;
use rayon::prelude::*;

aoc::test::test_part!(test1, part1, 5067);
aoc::test::test_part!(test2, part2, 1793);

type Point = (i32, i32);
type Dir = (i32, i32);
type Tiles = Vec<Vec<char>>;
type Path = HashSet<(Dir, Point)>;

fn read() -> (Tiles, Point) {
	let mut guard: Point = (0, 0);
	let tiles: Vec<Vec<char>> = input!()
		.lines()
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
	mut pos: Point,
	obstruction: Option<Point>,
) -> Option<Path> {
	let mut dir = (-1, 0);
	let mut visited = HashSet::new();
	while let Some(tile) = tiles
		.get(pos.0 as usize)
		.and_then(|row| row.get(pos.1 as usize))
	{
		if visited.contains(&(pos, dir)) {
			// Cycle.
			return None;
		}
		if *tile == '#' || obstruction == Some(pos) {
			// Turn.
			pos.0 -= dir.0;
			pos.1 -= dir.1;
			dir = (dir.1, -dir.0)
		} else {
			visited.insert((pos, dir));
		}
		// Walk.
		pos.0 += dir.0;
		pos.1 += dir.1;
	}
	Some(visited)
}

fn path_coords(path: Path) -> HashSet<Point> {
	path.into_iter().map(|(pos, _)| pos).collect()
}

pub fn part1() -> usize {
	let (tiles, guard) = read();
	path_coords(escape_path(&tiles, guard, None).unwrap()).len()
}

pub fn part2() -> usize {
	let (tiles, guard) = read();
	path_coords(escape_path(&tiles, guard, None).unwrap())
		.par_iter()
		.filter(|&obstruction| {
			tiles[obstruction.0 as usize][obstruction.1 as usize] != '#'
				&& *obstruction != guard
				&& escape_path(&tiles, guard, Some(*obstruction)).is_none()
		})
		.count()
}
