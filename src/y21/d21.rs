use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 684495);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> u32 {
	let (mut space1, mut space2) = read_lines("input/2021/21.txt")
		.map(|line| line.chars().last().unwrap().to_digit(10).unwrap())
		.collect_tuple()
		.unwrap();
	let (mut score1, mut score2) = (0, 0);
	let mut die = Die { value: 0, rolls: 0 };
	loop {
		space1 += die.roll() + die.roll() + die.roll();
		space1 = (space1 - 1) % 10 + 1;
		score1 += space1;
		if score1 >= 1000 {
			return score2 * die.rolls;
		}
		space2 += die.roll() + die.roll() + die.roll();
		space2 = (space2 - 1) % 10 + 1;
		score2 += space2;
		if score2 >= 1000 {
			return score1 * die.rolls;
		}
	}
}

pub fn part2() -> usize {
	let lines = read_lines("input/2021/21.txt");
	lines.count()
}

struct Die {
	value: u32,
	rolls: u32,
}

impl Die {
	fn roll(&mut self) -> u32 {
		self.value += 1;
		self.rolls += 1;
		if self.value > 100 {
			self.value = 1;
		}
		self.value
	}
}
