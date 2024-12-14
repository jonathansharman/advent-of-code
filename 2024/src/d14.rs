use std::cmp::Ordering;

use aoc::{define_point_and_vector, io::read_lines};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 230435667);
aoc::test::test_part!(test2, part2, ?);

define_point_and_vector!(Point, Vector, x, y, i64);

struct Robot {
	p: Point,
	v: Vector,
}

fn read_robots() -> Vec<Robot> {
	read_lines("input/14.txt")
		.map(|line| {
			let parse_tuple = |s: &str| {
				s[2..]
					.split(',')
					.map(|n| n.parse().unwrap())
					.collect_tuple::<(i64, i64)>()
					.unwrap()
			};
			let (p, v) = line.split_once(' ').unwrap();
			Robot {
				p: parse_tuple(p).into(),
				v: parse_tuple(v).into(),
			}
		})
		.collect()
}

fn safety_factor(size: Vector, seconds: i64) -> usize {
	let (mut q0, mut q1, mut q2, mut q3) = (0, 0, 0, 0);
	read_robots().into_iter().for_each(|robot| {
		let p = robot.p + seconds * robot.v;
		let px = ((p.x % size.x) + size.x) % size.x;
		let py = ((p.y % size.y) + size.y) % size.y;
		match (px.cmp(&(size.x / 2)), py.cmp(&(size.y / 2))) {
			(Ordering::Less, Ordering::Less) => q0 += 1,
			(Ordering::Greater, Ordering::Less) => q1 += 1,
			(Ordering::Less, Ordering::Greater) => q2 += 1,
			(Ordering::Greater, Ordering::Greater) => q3 += 1,
			_ => {}
		};
	});
	q0 * q1 * q2 * q3
}

pub fn part1() -> usize {
	safety_factor(Vector::new(101, 103), 100)
}

pub fn part2() -> usize {
	0
}
