use std::ops::Add;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 3359);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> u64 {
	read_lines("input/2021/18.txt")
		.map(|line| -> Number { parse_number(&mut line.as_bytes()) })
		.reduce(Add::add)
		.unwrap()
		.reduce()
		.magnitude()
}

pub fn part2() -> usize {
	read_lines("input/2021/18.txt").count()
}

enum Number {
	Regular(u64),
	Pair(Box<Number>, Box<Number>),
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
			Number::Pair(first, second) => {
				if depth == 4 {
					let first = if let Number::Regular(first) = *first {
						first
					} else {
						panic!("irregular exploded pair");
					};
					let second = if let Number::Regular(second) = *second {
						second
					} else {
						panic!("irregular exploded pair");
					};
					Ok((first, Number::Regular(0), second))
				} else {
					match first.explode(depth + 1) {
						Ok((left, first, right)) => Ok((
							left,
							Number::Pair(Box::new(first), Box::new(second.add_left(right))),
							0,
						)),
						Err(first) => match second.explode(depth + 1) {
							Ok((left, second, right)) => Ok((
								0,
								Number::Pair(Box::new(first.add_right(left)), Box::new(second)),
								right,
							)),
							Err(second) => Err(Number::Pair(Box::new(first), Box::new(second))),
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
			Number::Pair(first, second) => Number::Pair(Box::new(first.add_left(amount)), second),
		}
	}

	fn add_right(self, amount: u64) -> Number {
		match self {
			Number::Regular(n) => Number::Regular(n + amount),
			Number::Pair(first, second) => Number::Pair(first, Box::new(second.add_right(amount))),
		}
	}

	fn split(self) -> Result<Number, Number> {
		match self {
			Number::Regular(n) => {
				if n >= 10 {
					Ok(Number::Pair(
						Box::new(Number::Regular(n / 2)),
						Box::new(Number::Regular(n / 2 + n % 2)),
					))
				} else {
					Err(Number::Regular(n))
				}
			}
			Number::Pair(first, second) => match first.split() {
				Ok(first) => Ok(Number::Pair(Box::new(first), second)),
				Err(first) => match second.split() {
					Ok(second) => Ok(Number::Pair(Box::new(first), Box::new(second))),
					Err(second) => Err(Number::Pair(Box::new(first), Box::new(second))),
				},
			},
		}
	}

	fn magnitude(&self) -> u64 {
		match &self {
			Number::Regular(n) => *n,
			Number::Pair(first, second) => 3 * first.magnitude() + 2 * second.magnitude(),
		}
	}
}

impl Add for Number {
	type Output = Number;

	fn add(self, rhs: Self) -> Self::Output {
		Number::Pair(Box::new(self), Box::new(rhs)).reduce()
	}
}

impl std::fmt::Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Number::Regular(n) => write!(f, "{}", n),
			Number::Pair(first, second) => write!(f, "[{},{}]", first, second),
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
		Number::Pair(Box::new(first), Box::new(second))
	} else {
		let end = input.iter().position(|&c| !c.is_ascii_digit()).unwrap();
		let n = std::str::from_utf8(&input[..end]).unwrap().parse().unwrap();
		*input = &input[end..];
		Number::Regular(n)
	}
}
