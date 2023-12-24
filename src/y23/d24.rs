use std::str::FromStr;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 11995);
crate::test::test_part!(test2, part2, ?);

#[derive(Clone, Copy, Debug)]
struct Vector([f64; 3]);

impl Vector {
	fn x(&self) -> f64 {
		self.0[0]
	}

	fn y(&self) -> f64 {
		self.0[1]
	}
}

impl FromStr for Vector {
	type Err = ();

	fn from_str(s: &str) -> Result<Vector, Self::Err> {
		let (x, y, z) = s
			.split(", ")
			.map(|n| n.trim().parse().unwrap())
			.collect_tuple()
			.unwrap();
		Ok(Vector([x, y, z]))
	}
}

#[derive(Clone, Copy, Debug)]
struct Point([f64; 3]);

impl Point {
	fn x(&self) -> f64 {
		self.0[0]
	}

	fn y(&self) -> f64 {
		self.0[1]
	}
}

impl FromStr for Point {
	type Err = ();

	fn from_str(s: &str) -> Result<Point, Self::Err> {
		let (x, y, z) = s
			.split(", ")
			.map(|n| n.trim().parse().unwrap())
			.collect_tuple()
			.unwrap();
		Ok(Point([x, y, z]))
	}
}

pub fn part1() -> usize {
	let hail_stones: Vec<(Point, Vector)> = read_lines("input/2023/24.txt")
		.map(|line| {
			let (p, v) = line.split_once(" @ ").unwrap();
			(p.parse().unwrap(), v.parse().unwrap())
		})
		.collect();
	hail_stones
		.iter()
		.enumerate()
		.map(|(i, (p1, v1))| {
			hail_stones
				.iter()
				.skip(i + 1)
				.filter(|(p2, v2)| {
					const MIN: f64 = 200_000_000_000_000.0;
					const MAX: f64 = 400_000_000_000_000.0;

					let s = (v1.x() * p2.y()
						- v1.x() * p1.y() - v1.y() * p2.x()
						+ v1.y() * p1.x()) / (v1.y() * v2.x()
						- v1.x() * v2.y());
					let t = (p2.x() + s * v2.x() - p1.x()) / v1.x();
					let (ix, iy) = (p2.x() + s * v2.x(), p2.y() + s * v2.y());
					t >= 0.0
						&& s >= 0.0 && (MIN..=MAX).contains(&ix)
						&& (MIN..=MAX).contains(&iy)
				})
				.count()
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
