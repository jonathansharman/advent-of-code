use crate::io::read_lines;

crate::test::test_part!(test1, part1, 512950);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	read_lines("input/2023/15.txt")
		.next()
		.unwrap()
		.split(',')
		.map(|s| {
			let mut value: usize = 0;
			for b in s.bytes() {
				value += b as usize;
				value *= 17;
				value %= 256;
			}
			value
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
