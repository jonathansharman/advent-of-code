use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;
use z3::{
	ast::{Ast, Int},
	Config, Context, Solver,
};

use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 11995);
aoc::test::test_part!(test2, part2, 983620716335751);

#[derive(Clone, Copy, Debug)]
struct Vector([i64; 3]);

impl Vector {
	fn x(&self) -> i64 {
		self.0[0]
	}

	fn y(&self) -> i64 {
		self.0[1]
	}

	fn z(&self) -> i64 {
		self.0[2]
	}
}

impl FromStr for Vector {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Vector, Self::Err> {
		let (x, y, z) = s
			.split(", ")
			.map(|n| n.trim().parse())
			.collect_tuple()
			.unwrap();
		Ok(Vector([x?, y?, z?]))
	}
}

#[derive(Clone, Copy, Debug)]
struct Point([i64; 3]);

impl Point {
	fn x(&self) -> i64 {
		self.0[0]
	}

	fn y(&self) -> i64 {
		self.0[1]
	}

	fn z(&self) -> i64 {
		self.0[2]
	}
}

impl FromStr for Point {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Point, Self::Err> {
		let (x, y, z) = s
			.split(", ")
			.map(|n| n.trim().parse())
			.collect_tuple()
			.unwrap();
		Ok(Point([x?, y?, z?]))
	}
}

fn read_hail_stones() -> Vec<(Point, Vector)> {
	read_lines("input/24.txt")
		.map(|line| {
			let (p, v) = line.split_once(" @ ").unwrap();
			(p.parse().unwrap(), v.parse().unwrap())
		})
		.collect()
}

pub fn part1() -> usize {
	let hail_stones = read_hail_stones();
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
						+ v1.y() * p1.x()) as f64
						/ (v1.y() * v2.x() - v1.x() * v2.y()) as f64;
					let t = (p2.x() as f64 + s * v2.x() as f64 - p1.x() as f64)
						/ v1.x() as f64;
					let (ix, iy) = (
						p2.x() as f64 + s * v2.x() as f64,
						p2.y() as f64 + s * v2.y() as f64,
					);
					t >= 0.0
						&& s >= 0.0 && (MIN..=MAX).contains(&ix)
						&& (MIN..=MAX).contains(&iy)
				})
				.count()
		})
		.sum()
}

pub fn part2() -> i64 {
	let cfg = Config::default();
	let ctx = Context::new(&cfg);
	let solver = Solver::new(&ctx);
	let spx = Int::new_const(&ctx, "spx");
	let spy = Int::new_const(&ctx, "spy");
	let spz = Int::new_const(&ctx, "spz");
	let svx = Int::new_const(&ctx, "svx");
	let svy = Int::new_const(&ctx, "svy");
	let svz = Int::new_const(&ctx, "svz");
	for (hp, hv) in read_hail_stones() {
		let hpx = Int::from_i64(&ctx, hp.x());
		let hpy = Int::from_i64(&ctx, hp.y());
		let hpz = Int::from_i64(&ctx, hp.z());
		let hvx = Int::from_i64(&ctx, hv.x());
		let hvy = Int::from_i64(&ctx, hv.y());
		let hvz = Int::from_i64(&ctx, hv.z());
		solver.assert(
			&((&spx - &hpx) * (&hvy - &svy))
				._eq(&((&spy - &hpy) * (&hvx - &svx))),
		);
		solver.assert(
			&((&spx - &hpx) * (&hvz - &svz))
				._eq(&((&spz - &hpz) * (&hvx - &svx))),
		);
		solver.assert(
			&((&spy - &hpy) * (&hvz - &svz))
				._eq(&((&spz - &hpz) * (&hvy - &svy))),
		);
	}
	solver.check();
	let model = solver.get_model().unwrap();
	let (spx, spy, spz) = (
		model.eval(&spx, true).unwrap().as_i64().unwrap(),
		model.eval(&spy, true).unwrap().as_i64().unwrap(),
		model.eval(&spz, true).unwrap().as_i64().unwrap(),
	);
	spx + spy + spz
}
