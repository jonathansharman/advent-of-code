use std::{
	cmp::Ordering,
	collections::{HashMap, HashSet},
};

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 454);
crate::test::test_part!(test2, part2, ?);

struct Point {
	x: usize,
	y: usize,
	z: usize,
}

struct Brick {
	start: Point,
	end: Point,
	resting_on: HashSet<usize>,
}

impl Brick {
	fn get_cubes(&self) -> Vec<Point> {
		let mut cubes = Vec::new();
		for x in self.start.x..=self.end.x {
			for y in self.start.y..=self.end.y {
				for z in self.start.z..=self.end.z {
					cubes.push(Point { x, y, z });
				}
			}
		}
		cubes
	}
}

fn read_bricks() -> Vec<Brick> {
	read_lines("input/2023/22.txt")
		.map(|line| {
			let (start, end) =
				line.split('~').map(parse_point).collect_tuple().unwrap();
			Brick {
				start,
				end,
				resting_on: HashSet::new(),
			}
		})
		.collect()
}

fn parse_point(s: &str) -> Point {
	let (x, y, z) = s
		.splitn(3, ',')
		.map(|n| n.parse().unwrap())
		.collect_tuple()
		.unwrap();
	Point { x, y, z }
}

pub fn part1() -> usize {
	let mut bricks = read_bricks();
	// Process bricks from low to high.
	bricks.sort_by_key(|brick| brick.start.z);
	// (x, y) -> (brick index, z)
	let mut height_map: HashMap<(usize, usize), (usize, usize)> =
		HashMap::new();
	for (i, brick) in bricks.iter_mut().enumerate() {
		let mut z_max = 0;
		for cube in brick.get_cubes() {
			if let Some(&(j, z)) = height_map.get(&(cube.x, cube.y)) {
				match z.cmp(&z_max) {
					Ordering::Less => {}
					Ordering::Equal => {
						brick.resting_on.insert(j);
					}
					Ordering::Greater => {
						brick.resting_on.clear();
						brick.resting_on.insert(j);
						z_max = z;
					}
				}
			}
		}
		let height = brick.end.z - brick.start.z;
		brick.start.z = z_max + 1;
		brick.end.z = brick.start.z + height;
		for cube in brick.get_cubes() {
			height_map.insert((cube.x, cube.y), (i, cube.z));
		}
	}
	let total = bricks.len();
	let required = bricks
		.into_iter()
		.filter_map(|brick| {
			if brick.resting_on.len() == 1 {
				Some(brick.resting_on.into_iter().next().unwrap())
			} else {
				None
			}
		})
		.collect::<HashSet<_>>()
		.len();
	total - required
}

pub fn part2() -> usize {
	0
}
