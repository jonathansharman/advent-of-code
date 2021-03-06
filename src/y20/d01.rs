use crate::io::parse_lines;
use itertools::Itertools;

crate::test::test_part!(test1, part1, 731731);
crate::test::test_part!(test2, part2, 116115990);

pub fn part1() -> i64 {
	solve(2)
}

pub fn part2() -> i64 {
	solve(3)
}

fn solve(k: usize) -> i64 {
	parse_lines("input/2020/01.txt")
		.combinations(k)
		.filter_map(|elems| {
			if elems.iter().sum::<i64>() == 2020 {
				Some(elems.iter().product())
			} else {
				None
			}
		})
		.next()
		.unwrap()
}
