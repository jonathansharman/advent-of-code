use aoc::input;

aoc::test::test_part!(test1, part1, 17408);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	input!()
		.lines()
		.map(|bank| {
			let batteries = bank.as_bytes();
			batteries[..batteries.len() - 1]
				.iter()
				.enumerate()
				.map(|(i, b1)| {
					batteries[i + 1..]
						.iter()
						.map(|b2| (10 * (b1 - b'0') + b2 - b'0') as usize)
						.max()
						.unwrap()
				})
				.max()
				.unwrap()
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
