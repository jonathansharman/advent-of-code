use itertools::Itertools;

crate::test::test_part!(test1, part1, 1538);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let input = std::fs::read_to_string("input/2022/06.txt").unwrap();
	for (i, (a, b, c, d)) in input.as_bytes().iter().tuple_windows().enumerate()
	{
		if a != b && a != c && a != d && b != c && b != d && c != d {
			return i + 4;
		}
	}
	panic!()
}

pub fn part2() -> usize {
	0
}
