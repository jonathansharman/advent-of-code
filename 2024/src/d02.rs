use aoc::io::read_lines;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 591);
aoc::test::test_part!(test2, part2, 621);

pub fn part1() -> usize {
	read_lines("input/02.txt")
		.filter(|report| {
			report
				.split_whitespace()
				.map(|level| level.parse::<i32>().unwrap())
				.tuple_windows()
				.all(|(a, b, c)| {
					let (d1, d2) = (b - a, c - b);
					d1.signum() == d2.signum()
						&& ((1..=3).contains(&d1.abs()))
						&& ((1..=3).contains(&d2.abs()))
				})
		})
		.count()
}

pub fn part2() -> usize {
	read_lines("input/02.txt")
		.filter(|report| {
			let levels: Vec<i32> = report
				.split_whitespace()
				.map(|level| level.parse().unwrap())
				.collect();
			(0..levels.len()).any(|skipped| {
				levels
					.iter()
					.enumerate()
					.filter_map(|(i, level)| (i != skipped).then_some(level))
					.tuple_windows()
					.all(|(a, b, c)| {
						let (d1, d2) = (b - a, c - b);
						d1.signum() == d2.signum()
							&& ((1..=3).contains(&d1.abs()))
							&& ((1..=3).contains(&d2.abs()))
					})
			})
		})
		.count()
}
