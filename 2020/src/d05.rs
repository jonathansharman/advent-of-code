use itertools::Itertools;

use std::collections::BTreeSet;

aoc::test::test_part!(test1, part1, 904);
aoc::test::test_part!(test2, part2, 669);

const INPUT: &str = include_str!("input/05.txt");

pub fn part1() -> u32 {
	INPUT
		.lines()
		.map(|line| Pass::from(line.as_bytes()).seat_id())
		.max()
		.unwrap_or_default()
}

pub fn part2() -> u32 {
	let seat_ids: BTreeSet<_> = INPUT
		.lines()
		.map(|line| Pass::from(line.as_bytes()).seat_id())
		.collect();
	for (current, next) in seat_ids.into_iter().tuple_windows() {
		let expected_next = current + 1;
		if next != expected_next {
			return expected_next;
		}
	}
	0
}

struct Pass {
	row: u32,
	col: u32,
}

impl Pass {
	fn seat_id(&self) -> u32 {
		8 * self.row + self.col
	}
}

impl From<&[u8]> for Pass {
	fn from(bytes: &[u8]) -> Self {
		let row = bytes[0..7]
			.iter()
			.fold(0, |acc, &byte| 2 * acc + ((byte == b'B') as u32));
		let col = bytes[7..]
			.iter()
			.fold(0, |acc, &byte| 2 * acc + ((byte == b'R') as u32));
		Pass { row, col }
	}
}
