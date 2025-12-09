use aoc::{grid::Point, input, input::ParseCommaSeparated};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 4765757080);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> u64 {
	let reds: Vec<Point> = input!()
		.lines()
		.map(|line| {
			let (row, col) =
				line.parse_comma_separated().collect_tuple().unwrap();
			Point::new(row, col)
		})
		.collect();
	reds.iter()
		.cartesian_product(reds.iter())
		.map(|(p1, p2)| {
			(p1.row.abs_diff(p2.row) + 1) * (p1.col.abs_diff(p2.col) + 1)
		})
		.max()
		.unwrap()
}

pub fn part2() -> usize {
	0
}
