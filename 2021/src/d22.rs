use itertools::Itertools;
use std::{
	collections::{HashMap, HashSet},
	fmt::Display,
	ops::Sub,
};

aoc::test::test_part!(test1, part1, 570915);
aoc::test::test_part!(test2, part2, 1268313839428137);

const INPUT: &str = include_str!("input/22.txt");

pub fn part1() -> usize {
	let mut cubes = HashMap::new();
	for step in read_reboot_steps() {
		for x in step.region.begin[0].max(-50)..step.region.end[0].min(51) {
			for y in step.region.begin[1].max(-50)..step.region.end[1].min(51) {
				for z in
					step.region.begin[2].max(-50)..step.region.end[2].min(51)
				{
					cubes.insert([x, y, z], step.on);
				}
			}
		}
	}
	cubes.into_values().filter(|&cube| cube).count()
}

pub fn part2() -> i64 {
	let mut on_set: Vec<Region> = Vec::new();
	for step in read_reboot_steps() {
		if step.on {
			let mut step_set = HashSet::from([step.region]);
			for on_region in on_set.iter() {
				step_set = step_set
					.into_iter()
					.flat_map(|step_region| &step_region - on_region)
					.collect();
			}
			on_set.extend(step_set);
		} else {
			on_set = on_set
				.into_iter()
				.flat_map(|on_region| &on_region - &step.region)
				.collect();
		}
	}
	on_set.iter().map(Region::volume).sum()
}

type Point = [i64; 3];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Region {
	begin: Point,
	end: Point,
}

impl Region {
	fn volume(&self) -> i64 {
		self.begin
			.iter()
			.zip_eq(self.end.iter())
			.map(|(begin, end)| end - begin)
			.product()
	}

	fn intersection(&self, other: &Region) -> Option<Region> {
		let begin = [
			self.begin[0].max(other.begin[0]),
			self.begin[1].max(other.begin[1]),
			self.begin[2].max(other.begin[2]),
		];
		let end = [
			self.end[0].min(other.end[0]),
			self.end[1].min(other.end[1]),
			self.end[2].min(other.end[2]),
		];
		if begin.iter().zip_eq(&end).any(|(begin, end)| begin > end) {
			None
		} else {
			Some(Region { begin, end })
		}
	}

	fn abuts(&self, point: Point) -> bool {
		(0..3).all(|axis| {
			self.begin[axis] <= point[axis] && point[axis] <= self.end[axis]
		})
	}

	fn split_around_point(self, point: Point) -> Vec<Region> {
		if self.abuts(point) {
			[
				Region {
					begin: self.begin,
					end: point,
				},
				Region {
					begin: [self.begin[0], self.begin[1], point[2]],
					end: [point[0], point[1], self.end[2]],
				},
				Region {
					begin: [self.begin[0], point[1], self.begin[2]],
					end: [point[0], self.end[1], point[2]],
				},
				Region {
					begin: [self.begin[0], point[1], point[2]],
					end: [point[0], self.end[1], self.end[2]],
				},
				Region {
					begin: [point[0], self.begin[1], self.begin[2]],
					end: [self.end[0], point[1], point[2]],
				},
				Region {
					begin: [point[0], self.begin[1], point[2]],
					end: [self.end[0], point[1], self.end[2]],
				},
				Region {
					begin: [point[0], point[1], self.begin[2]],
					end: [self.end[0], self.end[1], point[2]],
				},
				Region {
					begin: point,
					end: self.end,
				},
			]
			.into_iter()
			.filter(|region| region.volume() > 0)
			.collect()
		} else {
			vec![self]
		}
	}

	fn split_around_subregion(self, other: &Region) -> HashSet<Region> {
		let mut set = HashSet::from([self]);
		for corner in other.corners() {
			if self.abuts(corner) {
				set = set
					.into_iter()
					.flat_map(|region| region.split_around_point(corner))
					.collect();
			}
		}
		set
	}

	fn corners(&self) -> [Point; 8] {
		let mut result = [[0; 3]; 8];
		let mut idx = 0;
		for x in [self.begin[0], self.end[0]] {
			for y in [self.begin[1], self.end[1]] {
				for z in [self.begin[2], self.end[2]] {
					result[idx] = [x, y, z];
					idx += 1;
				}
			}
		}
		result
	}
}

impl Sub<&Region> for &Region {
	type Output = HashSet<Region>;

	/// Returns `self` \ `rhs` as disjoint regions.
	fn sub(self, rhs: &Region) -> Self::Output {
		if let Some(intersection) = self.intersection(rhs) {
			let mut splits = self.split_around_subregion(&intersection);
			splits.remove(&intersection);
			splits
		} else {
			HashSet::from([*self])
		}
	}
}

impl Display for Region {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"x={}..{},y={}..{},z={}..{}",
			self.begin[0],
			self.end[0] - 1,
			self.begin[1],
			self.end[1] - 1,
			self.begin[2],
			self.end[2] - 1
		)
	}
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct RebootStep {
	on: bool,
	region: Region,
}

fn read_reboot_steps() -> Vec<RebootStep> {
	INPUT
		.lines()
		.map(|line| {
			let on = line.as_bytes()[1] == b'n';
			let x_str_begin = line.find("x=").unwrap() + 2;
			let x_str_end = line.find(",y").unwrap();
			let y_str_begin = line.find("y=").unwrap() + 2;
			let y_str_end = line.find(",z").unwrap();
			let z_str_begin = line.find("z=").unwrap() + 2;
			let (x_begin, x_end) = parse_range(&line[x_str_begin..x_str_end]);
			let (y_begin, y_end) = parse_range(&line[y_str_begin..y_str_end]);
			let (z_begin, z_end) = parse_range(&line[z_str_begin..]);
			RebootStep {
				on,
				region: Region {
					begin: [x_begin, y_begin, z_begin],
					end: [x_end, y_end, z_end],
				},
			}
		})
		.collect()
}

fn parse_range(range: &str) -> (i64, i64) {
	range
		.split("..")
		.map(|n| n.parse().unwrap())
		.collect_tuple()
		// Use semi-inclusive bounds
		.map(|(a, b)| (a, b + 1))
		.unwrap()
}
