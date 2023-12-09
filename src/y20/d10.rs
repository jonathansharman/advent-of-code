use itertools::Itertools;

use crate::io::parse_lines;

crate::test::test_part!(test1, part1, 2592);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut adapters = parse_lines("input/2020/10.txt")
		.sorted()
		.collect::<Vec<usize>>();
	adapters.insert(0, 0);
	adapters.push(adapters.last().unwrap() + 3);
	let (diff1s, diff3s) = adapters
		.into_iter()
		.tuple_windows()
		.map(|(a, b)| b - a)
		.fold((0, 0), |(diff1s, diff3s), diff| match diff {
			1 => (diff1s + 1, diff3s),
			3 => (diff1s, diff3s + 1),
			_ => (diff1s, diff3s),
		});
	diff1s * diff3s
}

pub fn part2() -> usize {
	0
}
