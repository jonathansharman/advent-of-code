use std::{cmp::Ordering, fmt::Debug};

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 5208);
crate::test::test_part!(test2, part2, 25792);

#[derive(Clone, Eq, PartialEq)]
enum Value {
	Number(i32),
	List(Vec<Value>),
}

impl Debug for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Number(value) => write!(f, "{value}"),
			Self::List(value) => write!(f, "{value:?}"),
		}
	}
}

impl Ord for Value {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Value::Number(l), Value::Number(r)) => l.cmp(r),
			(Value::List(l), Value::List(r)) => l.cmp(r),
			(Value::List(l), r @ Value::Number(_)) => l.cmp(&vec![r.clone()]),
			(l @ Value::Number(_), Value::List(r)) => vec![l.clone()].cmp(r),
		}
	}
}

impl PartialOrd for Value {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn parse_value(input: &mut &[u8]) -> Option<Value> {
	if input.is_empty() {
		None
	} else {
		parse_list(input).or_else(|| parse_number(input))
	}
}

fn parse_list(input: &mut &[u8]) -> Option<Value> {
	if input[0] == b'[' {
		*input = &(*input)[1..];
		let mut values = Vec::new();
		if let Some(value) = parse_value(input) {
			values.push(value);
		}
		loop {
			match input[0] {
				b']' => {
					*input = &(*input)[1..];
					return Some(Value::List(values));
				}
				b',' => {
					*input = &(*input)[1..];
					values.push(parse_value(input)?);
				}
				_ => return None,
			}
		}
	} else {
		None
	}
}

fn parse_number(input: &mut &[u8]) -> Option<Value> {
	let idx = input.iter().position(|c| !c.is_ascii_digit())?;
	let value = std::str::from_utf8(&input[..idx]).ok()?.parse().ok()?;
	*input = &(*input)[idx..];
	Some(Value::Number(value))
}

pub fn part1() -> usize {
	read_lines("input/2022/13.txt")
		.filter_map(|line| parse_value(&mut line.as_bytes()))
		.chunks(2)
		.into_iter()
		.enumerate()
		.map(|(i, pair)| {
			let pair = pair.collect_vec();
			if pair[0] < pair[1] {
				i + 1
			} else {
				0
			}
		})
		.sum()
}

pub fn part2() -> usize {
	let mut packets = read_lines("input/2022/13.txt")
		.filter_map(|line| parse_value(&mut line.as_bytes()))
		.collect_vec();
	let div1 = parse_value(&mut "[[2]]".as_bytes()).unwrap();
	let div2 = parse_value(&mut "[[6]]".as_bytes()).unwrap();
	packets.push(div1.clone());
	packets.push(div2.clone());
	packets.sort();
	let idx1 = packets.iter().position(|p| p == &div1).unwrap();
	let idx2 = packets.iter().position(|p| p == &div2).unwrap();
	(idx1 + 1) * (idx2 + 1)
}
