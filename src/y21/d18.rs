use std::ops::Add;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 3359);
crate::test::test_part!(test2, part2, 4616);

pub fn part1() -> u64 {
	read_numbers()
		.reduce(Add::add)
		.unwrap()
		.reduce()
		.magnitude()
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
	fn reduce(self) -> Number {
		match self.explode(0) {
			Ok((_, exploded, _)) => exploded.reduce(),
			Err(original) => match original.split() {
				Ok(split) => split.reduce(),
				Err(original) => original,
			},
		}
	}

	fn explode(self, depth: usize) -> Result<(u64, Number, u64), Number> {
		match self {
			Number::Pair(pair) => {
				if depth == 4 {
					let first = if let Number::Regular(first) = pair.0 {
						first
					} else {
						panic!("irregular exploded pair");
					};
					let second = if let Number::Regular(second) = pair.1 {
						second
					} else {
						panic!("irregular exploded pair");
					};
					Ok((first, Number::Regular(0), second))
				} else {
					match pair.0.explode(depth + 1) {
						Ok((left, first, right)) => Ok((
							left,
							Number::Pair(Box::new((first, pair.1.add_left(right)))),
							0,
						)),
						Err(first) => match pair.1.explode(depth + 1) {
							Ok((left, second, right)) => Ok((
								0,
								Number::Pair(Box::new((first.add_right(left), second))),
								right,
							)),
							Err(second) => Err(Number::Pair(Box::new((first, second)))),
						},
					}
				}
			}
			regular => Err(regular),
		}
	}

	fn add_left(self, amount: u64) -> Number {
		match self {
			Number::Regular(n) => Number::Regular(n + amount),
			Number::Pair(pair) => Number::Pair(Box::new((pair.0.add_left(amount), pair.1))),
		}
	}

	fn add_right(self, amount: u64) -> Number {
		match self {
			Number::Regular(n) => Number::Regular(n + amount),
			Number::Pair(pair) => Number::Pair(Box::new((pair.0, pair.1.add_right(amount)))),
		}
	}

	fn split(self) -> Result<Number, Number> {
		match self {
			Number::Regular(n) => {
				if n >= 10 {
					Ok(Number::Pair(Box::new((
						Number::Regular(n / 2),
						Number::Regular(n / 2 + n % 2),
					))))
				} else {
					Err(Number::Regular(n))
				}
			}
			Number::Pair(pair) => match pair.0.split() {
				Ok(first) => Ok(Number::Pair(Box::new((first, pair.1)))),
				Err(first) => match pair.1.split() {
					Ok(second) => Ok(Number::Pair(Box::new((first, second)))),
					Err(second) => Err(Number::Pair(Box::new((first, second)))),
				},
			},
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
		Number::Pair(Box::new((self, rhs))).reduce()
	}
}

impl std::fmt::Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Number::Regular(n) => write!(f, "{}", n),
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
