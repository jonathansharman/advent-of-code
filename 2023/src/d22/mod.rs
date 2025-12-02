use std::{
	cmp::Ordering,
	collections::{HashMap, HashSet},
};

use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 454);
aoc::test::test_part!(test2, part2, 74287);

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
	input!()
		.lines()
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

fn settle(bricks: &mut [Brick]) {
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
}

fn required_bricks(bricks: &[Brick]) -> HashSet<usize> {
	bricks
		.iter()
		.filter_map(|brick| {
			if brick.resting_on.len() == 1 {
				Some(*brick.resting_on.iter().next().unwrap())
			} else {
				None
			}
		})
		.collect::<HashSet<_>>()
}

pub fn part1() -> usize {
	let mut bricks = read_bricks();
	settle(&mut bricks);
	let required = required_bricks(&bricks).len();
	bricks.len() - required
}

fn drops(
	bricks: &[Brick],
	supporting: &HashMap<usize, HashSet<usize>>,
	i: usize,
) -> usize {
	let mut queue = vec![i];
	let mut removed = HashSet::new();
	while let Some(i) = queue.pop() {
		removed.insert(i);
		if let Some(supporting) = supporting.get(&i) {
			for &j in supporting {
				if bricks[j].resting_on.is_subset(&removed) {
					queue.push(j);
				}
			}
		}
	}
	removed.len() - 1
}

pub fn part2() -> usize {
	let mut bricks = read_bricks();
	settle(&mut bricks);
	let supporting = bricks.iter().enumerate().fold(
		HashMap::<usize, HashSet<usize>>::new(),
		|mut acc, (i, brick)| {
			for &j in &brick.resting_on {
				acc.entry(j).or_default().insert(i);
			}
			acc
		},
	);
	(0..bricks.len())
		.map(|i| drops(&bricks, &supporting, i))
		.sum()
}
