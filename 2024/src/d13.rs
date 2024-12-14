use aoc::{define_point_and_vector, io::read_lines};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 29187);
aoc::test::test_part!(test2, part2, 99968222587852);

define_point_and_vector!(Point, Vector, x, y, i128);

#[derive(Debug)]
struct Machine {
	a: Vector,
	b: Vector,
	p: Point,
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
			let a = Vector {
				x: left.parse::<i128>().unwrap(),
				y: right.parse().unwrap(),
			};

			let (left, right) = chunk[1]
				.strip_prefix("Button B: X+")
				.unwrap()
				.split_once(", Y+")
				.unwrap();
			let b = Vector {
				x: left.parse::<i128>().unwrap(),
				y: right.parse().unwrap(),
			};

			let (left, right) = chunk[2]
				.strip_prefix("Prize: X=")
				.unwrap()
				.split_once(", Y=")
				.unwrap();
			let prize = Point {
				x: left.parse::<i128>().unwrap(),
				y: right.parse().unwrap(),
			};

			Machine { a, b, p: prize }
		})
		.collect()
}

fn min_cost(m: Machine) -> Option<i128> {
	let b_numer = m.a.y * m.p.x - m.a.x * m.p.y;
	let b_denom = m.a.y * m.b.x - m.a.x * m.b.y;
	if b_denom == 0 || b_numer % b_denom != 0 {
		return None;
	}
	let b = b_numer / b_denom;

	let a_numer = m.p.x - m.b.x * b;
	let a_denom = m.a.x;
	if a_numer % a_denom != 0 {
		return None;
	}
	let a = a_numer / a_denom;

	let cost = 3 * a + b;
	Some(cost)
}

pub fn part1() -> i128 {
	read_machines().into_iter().filter_map(min_cost).sum()
}

pub fn part2() -> i128 {
	const ERROR: i128 = 10000000000000;
	const ERROR_VEC: Vector = Vector { x: ERROR, y: ERROR };
	read_machines()
		.into_iter()
		.map(|machine| Machine {
			p: ERROR_VEC + machine.p,
			..machine
		})
		.filter_map(min_cost)
		.sum()
}
