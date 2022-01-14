use std::ops::Add;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 3359);
crate::test::test_part!(test2, part2, 4616);

pub fn part1() -> u64 {
	read_numbers().reduce(Add::add).unwrap().magnitude()
}

pub fn part2() -> u64 {
	read_numbers()
		.permutations(2)
		.map(|ns| (ns[0].clone() + ns[1].clone()).magnitude())
		.max()
		.unwrap()
}

fn read_numbers() -> impl Iterator<Item = Number> {
	read_lines("input/2021/18.txt").map(|line| -> Number { parse_number(&mut line.as_bytes()) })
}

#[derive(Clone)]
enum Number {
	Regular(u64),
	Pair(Box<(Number, Number)>),
}

impl Number {
	fn reduce(&mut self) {
		while self.explode(0).is_some() || self.split() {}
	}

	fn explode(&mut self, depth: usize) -> Option<(u64, u64)> {
		if let Number::Pair(pair) = self {
			if depth == 4 {
				if let (Number::Regular(first), Number::Regular(second)) = **pair {
					*self = Number::Regular(0);
					return Some((first, second));
				}
				panic!("irregular exploded pair");
			} else {
				if let Some((left, right)) = pair.0.explode(depth + 1) {
					pair.1.add_left(right);
					return Some((left, 0));
				}
				if let Some((left, right)) = pair.1.explode(depth + 1) {
					pair.0.add_right(left);
					return Some((0, right));
				}
			}
		}
		None
	}

	fn add_left(&mut self, amount: u64) {
		match self {
			Number::Regular(n) => *n += amount,
			Number::Pair(pair) => pair.0.add_left(amount),
		}
	}

	fn add_right(&mut self, amount: u64) {
		match self {
			Number::Regular(n) => *n += amount,
			Number::Pair(pair) => pair.1.add_right(amount),
		}
	}

	fn split(&mut self) -> bool {
		match self {
			Number::Regular(n) => {
				if *n >= 10 {
					*self = Number::Pair(Box::new((
						Number::Regular(*n / 2),
						Number::Regular(*n / 2 + *n % 2),
					)));
					true
				} else {
					false
				}
			}
			Number::Pair(pair) => pair.0.split() || pair.1.split(),
		}
	}

	fn magnitude(&self) -> u64 {
		match &self {
			Number::Regular(n) => *n,
			Number::Pair(pair) => 3 * pair.0.magnitude() + 2 * pair.1.magnitude(),
		}
	}
}

impl Add for Number {
	type Output = Number;

	fn add(self, rhs: Self) -> Self::Output {
		let mut sum = Number::Pair(Box::new((self, rhs)));
		sum.reduce();
		sum
	}
}

impl std::fmt::Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Number::Regular(n) => write!(f, "{n}"),
			Number::Pair(pair) => write!(f, "[{},{}]", pair.0, pair.1),
		}
	}
}

fn parse_byte(input: &mut &[u8], byte: u8) {
	if input[0] != byte {
		panic!("unexpected byte");
	}
	*input = &input[1..];
}

fn parse_number(input: &mut &[u8]) -> Number {
	if input[0] == b'[' {
		parse_byte(input, b'[');
		let first = parse_number(input);
		parse_byte(input, b',');
		let second = parse_number(input);
		parse_byte(input, b']');
		Number::Pair(Box::new((first, second)))
	} else {
		let end = input.iter().position(|&c| !c.is_ascii_digit()).unwrap();
		let n = std::str::from_utf8(&input[..end]).unwrap().parse().unwrap();
		*input = &input[end..];
		Number::Regular(n)
	}
}
