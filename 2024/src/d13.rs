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

pub fn part1() -> i64 {
	read_machines()
		.into_iter()
		.filter_map(|machine| {
			let mut min = None;
			for a in 0..=100 {
				for b in 0..=100 {
					if a * machine.a + b * machine.b == machine.prize {
						let cost = 3 * a + b;
						min = Some(min.unwrap_or(cost).min(cost));
					}
				}
			}
			min
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
