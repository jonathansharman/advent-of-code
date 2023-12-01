use crate::io::read_lines;

crate::test::test_part!(test1, part1, 55971);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	read_lines("input/2023/01.txt")
		.map(|line| {
			let digits = line
				.bytes()
				.filter_map(|c| {
					c.is_ascii_digit().then_some((c - b'0') as usize)
				})
				.collect::<Vec<_>>();
			10 * digits[0] + digits.last().unwrap()
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
