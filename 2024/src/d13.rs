use aoc::{grid::Point, io::read_lines};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 29187);
aoc::test::test_part!(test2, part2, ?);

struct Machine {
	a: Point,
	b: Point,
	prize: Point,
}

fn read_machines() -> Vec<Machine> {
	read_lines("input/13.txt")
		.filter(|line| !line.is_empty())
		.chunks(3)
		.into_iter()
		.map(|chunk| {
			let chunk: Vec<String> = chunk.collect();

			let (left, right) = chunk[0]
				.strip_prefix("Button A: X+")
				.unwrap()
				.split_once(", Y+")
				.unwrap();
			let a = Point {
				col: left.parse::<i64>().unwrap(),
				row: right.parse().unwrap(),
			};

			let (left, right) = chunk[1]
				.strip_prefix("Button B: X+")
				.unwrap()
				.split_once(", Y+")
				.unwrap();
			let b = Point {
				col: left.parse::<i64>().unwrap(),
				row: right.parse().unwrap(),
			};

			let (left, right) = chunk[2]
				.strip_prefix("Prize: X=")
				.unwrap()
				.split_once(", Y=")
				.unwrap();
			let prize = Point {
				col: left.parse::<i64>().unwrap(),
				row: right.parse().unwrap(),
			};

			Machine { a, b, prize }
		})
		.collect()
}

fn min_cost(machine: Machine) -> Option<i64> {
	(0..=100)
		.cartesian_product(0..=100)
		.filter_map(|(a, b)| {
			if a * machine.a + b * machine.b == machine.prize {
				Some(3 * a + b)
			} else {
				None
			}
		})
		.min()
}

pub fn part1() -> i64 {
	read_machines().into_iter().filter_map(min_cost).sum()
}

pub fn part2() -> i64 {
	const ERROR: i64 = 10000000000000;
	read_machines()
		.into_iter()
		.map(|machine| Machine {
			prize: Point::from((ERROR, ERROR)) + machine.prize,
			..machine
		})
		.filter_map(min_cost)
		.sum()
}
