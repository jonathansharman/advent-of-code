use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, ?);
aoc::test::test_part!(test2, part2, ?);

fn ranges() -> impl Iterator<Item = (usize, usize)> {
	input!().split(',').map(|range| {
		range
			.split('-')
			.map(|s| s.parse::<usize>().unwrap())
			.collect_tuple()
			.unwrap()
	})
}

pub fn part1() -> usize {
	ranges()
		.flat_map(|(min, max)| {
			(min..=max).filter(|id| {
				let id = id.to_string();
				id[..id.len() / 2] == id[id.len() / 2..]
			})
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
