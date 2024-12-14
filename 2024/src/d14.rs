use std::cmp::Ordering;

use aoc::{
	grid::{Grid, Point, Vector},
	io::read_lines,
};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 230435667);
aoc::test::test_part!(test2, part2, 7709);

struct Robot {
	p0: Point,
	v: Vector,
}

impl Robot {
	fn p(&self, size: Vector, t: i64) -> Point {
		let p = self.p0 + t * self.v;
		let row = ((p.row % size.row) + size.row) % size.row;
		let col = ((p.col % size.col) + size.col) % size.col;
		Point::new(row, col)
	}
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
			let p = parse_tuple(p);
			let v = parse_tuple(v);
			Robot {
				p0: Point { row: p.1, col: p.0 },
				v: Vector { row: v.1, col: v.0 },
			}
		})
		.collect()
}

fn safety_factor(size: Vector, t: i64) -> usize {
	let (mut q0, mut q1, mut q2, mut q3) = (0, 0, 0, 0);
	read_robots().into_iter().for_each(|robot| {
		let p = robot.p(size, t);
		match (p.col.cmp(&(size.col / 2)), p.row.cmp(&(size.row / 2))) {
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
	safety_factor(Vector { row: 103, col: 101 }, 100)
}

fn has_tree(size: Vector, t: i64, robots: &[Robot]) -> bool {
	const NEEDLE: [bool; 31] = [true; 31];
	let mut grid = Grid::new(size, false);
	for robot in robots {
		grid[robot.p(size, t)] = true;
	}
	for row in grid.into_rows() {
		for i in 0..row.len() - NEEDLE.len() {
			if row[i..i + NEEDLE.len()] == NEEDLE {
				return true;
			}
		}
	}
	false
}

#[allow(unused)]
fn print_robots(size: Vector, t: i64, robots: &[Robot]) {
	let mut grid = Grid::new(size, '.');
	for robot in robots {
		grid[robot.p(size, t)] = '*';
	}
	for row in grid.into_rows() {
		for c in row {
			print!("{c}");
		}
		println!();
	}
}

pub fn part2() -> i64 {
	let robots = read_robots();
	for t in 0.. {
		if has_tree(Vector { row: 103, col: 101 }, t, &robots) {
			return t;
		}
	}
	unreachable!()
}
